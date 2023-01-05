use crate::global::http_get;
use serde_json::Value;
use std::{
    fs::{self, File},
    io::Write
};

// Define the current working directory as a global variable
lazy_static::lazy_static! {
    static ref CURR_DIR: String = match std::env::current_dir() {
        Ok(path) => match path.into_os_string().into_string() {
            Ok(p) => p,
            Err(_) => panic!("failed to get current working directory."),
        },
        Err(_) => panic!("failed to get current working directory."),
    };
}


// Initialize the update command
pub async fn init(path: &str) {
    // Convert the path to a mutable variable
    let mut path: String = path.to_string();

    // Remove any trailing slashes
    while path.ends_with("/") {
        path = path[..path.len() - 1].to_string();
    }

    // If the path contains http (a url)
    if path.contains("github.com/") {
        if !path.starts_with("https://") {
            path = format!("https://{}", path);
            return import_with_url(&path).await;
        }
        return import_with_url(&path).await;
    }

    // Split the provided path to get the user and repo
    let split_path: Vec<&str> = path.split("/").collect();
    let user: &str = split_path[0];
    let repo: &str = split_path[1];

    // Create the repo url
    path = format!("https://github.com/{}/{}", user, repo);

    // Create the import file using the repo url
    create_import_with_repo(&path).await;
}

// The import_with_url() function is used to
// import the provided file using the github
// url the user provided.
async fn import_with_url(path: &str) {
    // Split the path to determine what type of
    // github url it is.
    let split_path: Vec<&str> = path.split("/").collect();

    // If the provided url is just the repo url...
    if split_path.len() <= 5 {
        // Create the .sty file from the repo content
        return create_import_with_repo(path).await;
    }

    // Create a mutable path variable
    let mut path: String = path.to_string();

    // Replace the /blob/ with /raw/ to get the
    // raw contents of the file.
    if path.contains("/blob/") {
        path = path.replace("/blob/", "/raw/");
    }

    // Get the import name and contents
    let import: String = extract_import_name(&path);
    let contents: String = get_import_contents(&path).await;

    // Create new import file
    create_import_file(&import, &contents).await;
}

// The create_import_with_repo() function is used to
// create a new .sty file using the provided github
// repo url. This works by getting the provided repos
// file contents then iterating over them to check which
// file names end with .sty
//
// If the file ends with .sty then the download_url
// for that file is grabbed then used to get the content
// of that file. The file contents is then copied to a
// local file with that file name.
async fn create_import_with_repo(path: &str) {
    let mut path: String = path.to_string();

    // If the provided url has .git on the end of it
    if path.ends_with(".git") {
        path = path[..path.len() - 4].to_string();
    }

    // Convert the url to the github api url.
    let split_path: Vec<&str> = path.split("/").collect();
    path = format!(
        "https://api.github.com/repos/{}/{}/contents/",
        split_path[3], split_path[4]
    );

    // Get the new .sty file path from the repo url
    match get_import_url_from_repo(&path).await {
        Some(path) => {
            // Get the import and contents
            let import: String = extract_import_name(&path);
            let contents: String = get_import_contents(&path).await;

            // Create new import file
            create_import_file(&import, &contents).await;
        }
        None => panic!("failed to get import url from repo."),
    }
}

// The get_import_url_from_repo() function is used to get the
// .sty file download url from the provided github repo.
async fn get_import_url_from_repo(path: &str) -> Option<String> {
    // Get the http request response
    let resp: reqwest::Response = http_get(path).await;

    // Get the response json as a serde_json::Value
    let json: Value = match resp.text().await {
        Ok(body) => match serde_json::from_str(&body) {
            Ok(j) => j,
            Err(e) => panic!("failed to parse response json. {:?}", e),
        },
        Err(e) => panic!("failed to extract http response body. {:?}", e),
    };

    // Convert the json response to an array
    let json: &Vec<Value> = match json.as_array() {
        Some(j) => j,
        None => panic!("failed to parse http response json."),
    };

    // Declare an empty variables
    let mut snippets_url: String = String::new();
    let mut import_url: String = String::new();
    let mut import_name: String = String::new();

    // Iterate over the array to find the .sty file
    for map in json {
        match map.get("name") {
            None => (),
            Some(n) => {
                let name: String = n.to_string();

                // Check for snippets.json
                if name.contains("snippets") {
                    snippets_url = match map.get("download_url") {
                        Some(download_url) => download_url.to_string().replace("\"", ""),
                        None => snippets_url,
                    }
                }

                // Else, if the file name ends with .sty
                else if name.ends_with(".sty\"") {
                    // Get the import name
                    import_name = name
                        .replace("\"", "")
                        .replace(".sty", "");

                    // Get the download url
                    import_url = match map.get("download_url") {
                        Some(download_url) => download_url.to_string().replace("\"", ""),
                        None => import_url
                    }
                }
            }
        }
    }

    // Check if the snippets url is not empty
    if snippets_url.len() > 0 {
        import_snippets_from_repo(&import_name, &snippets_url).await
    }

    // Return the import url
    match import_url.len() > 0 {
        true => Some(import_url),
        false => None,
    }
}

// The import_snippets_from_repo() function is used to
// import the snippets.json file from the provided
// github repo url into the .vscode directory.
async fn import_snippets_from_repo(name: &str, url: &str) {
    // Send an http get request to the provided url
    let resp = http_get(&url).await;

    // Then parse the response json
    let json: Value = match resp.text().await {
        Ok(body) => match serde_json::from_str(&body) {
            Ok(j) => j,
            Err(e) => return println!("failed to parse response json. {:?}", e),
        },
        Err(e) => return println!("failed to extract http response body. {:?}", e),
    };

    // Check if the .vscode directory exists
    match std::fs::metadata(format!("{}/.vscode", CURR_DIR.to_string())) {
        Ok(_) => (),
        Err(_) => {
            match std::fs::create_dir(format!("{}/.vscode", CURR_DIR.to_string())) {
                Ok(_) => (),
                Err(e) => return println!("failed to create .vscode directory. {:?}", e),
            }
        }
    }

    // Create the snippets file for the import
    match File::create(format!(
        "{}/.vscode/{}.code-snippets", CURR_DIR.to_string(), name)
    ) {
        Ok(mut file) => {
            match file.write_all(json.to_string().as_bytes()) {
                Ok(_) => println!("imported snippets file."),
                Err(e) => return println!("failed to write to snippets file. {:?}", e),
            }
        }
        Err(e) => return println!("failed to create snippets file. {:?}", e),
    }
}

// The extract_import_name() function is used
// to extract the import name of the provided path.
fn extract_import_name(path: &str) -> String {
    // Split the path to get the import name
    let split_path: Vec<&str> = path.split("/").collect();

    // Get the import name using the split path array
    return split_path[split_path.len() - 1].to_string();
}

// The get_import_contents() function is used to get
// the raw file content from the github repo using the
// provided path.
async fn get_import_contents(path: &str) -> String {
    // Send the http request and get the response
    let resp: reqwest::Response = http_get(path).await;

    // Get the response body
    return match resp.text().await {
        Ok(body) => body,
        Err(e) => panic!("failed to extract http response body. {:?}", e),
    };
}

// The create_import_file() is used to create the
// new .sty file with the import name and write the
// contents into the file
async fn create_import_file(import: &str, contents: &str) {
    // If the import already exists, return the function.
    if import_already_exists(import) {
        println!("the import '{import}' already exists, would you like to install it anyways? (y/n)");
        let mut input: String = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                if !input.to_lowercase().contains("y") {
                    std::process::exit(1);
                }
            }
            Err(e) => panic!("failed to read user input. {:?}", e),
        };
    };

    // Create a new file with the name of the provided import.
    let working_dir: String = CURR_DIR.to_string();
    let file = File::create(format!("{}/{}", working_dir, import));
    
    let mut file = match file {
        Ok(file) => file,
        Err(e) => panic!("failed to create new import file. {:?}", e),
    };

    // Write the import contents (the text scraped from the github repo file)
    // to the new import file.
    let write_res = file.write_all(contents.as_bytes());
    match write_res {
        Ok(_) => println!("successfully imported package: {}", import),
        Err(e) => panic!("failed to write import data to import file. {:?}", e),
    }
}

// Check if the import already exists. This function
// is required to avoid file errors. I might change it
// to overwrite the current file with the new import depending
// on what's more convenient.
fn import_already_exists(import: &str) -> bool {
    let working_dir: String = CURR_DIR.to_string();
    let data = fs::read(format!("{}/{}", working_dir, import));
    return match data {
        Ok(_) => true,
        Err(_) => false,
    };
}
