use serde_json::Value;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

// Initialize the update command
pub async fn init(path: &str) {
    let mut path: String = path.to_string();

    // Get current working directory
    let dir: Option<String> = get_current_dir();
    let dir: String = match dir {
        Some(d) => d,
        None => panic!("failed to get current working directory."),
    };

    // Remove any trailing slashes
    while path.ends_with("/") {
        path = path[..path.len() - 1].to_string();
    }

    // If the path contains http (a url)
    if path.contains("github.com/") {
        if !path.starts_with("https://") {
            path = format!("https://{}", path);
            return import_with_url(&dir, &path).await;
        }
        return import_with_url(&dir, &path).await;
    }

    // Split the provided path to get the user and repo
    let split_path: Vec<&str> = path.split("/").collect();
    let user: &str = split_path[0];
    let repo: &str = split_path[1];

    // Create the repo url
    path = format!("https://github.com/{}/{}", user, repo);

    // Create the import file using the repo url
    create_import_with_repo(&dir, &path).await;
}

// Get the current working directory. This is where
// the folder containing the imports will be located.
// %CURRENT_DIR%/file.sty..
fn get_current_dir() -> Option<String> {
    let res = env::current_dir();
    return match res {
        Ok(path) => Some(path.into_os_string().into_string().unwrap()),
        Err(_) => None,
    };
}

// The import_with_url() function is used to
// import the provided file using the github
// url the user provided.
async fn import_with_url(dir: &str, path: &str) {
    // Split the path to determine what type of
    // github url it is.
    let split_path: Vec<&str> = path.split("/").collect();

    // If the provided url is just the repo url...
    if split_path.len() <= 6 {
        // Create the .sty file from the repo content
        return create_import_with_repo(dir, path).await;
    }

    // Create a mutable path variable
    let mut _path: String = path.to_string();

    // Replace the /blob/ with /raw/ to get the
    // raw contents of the file.
    if path.contains("/blob/") {
        _path = path.replace("/blob/", "/raw/");
    }

    // Get the import name and contents
    let import: String = extract_import_name(&_path);
    let contents: String = get_import_contents(&_path).await;

    // Create new import file
    create_import_file(dir, &import, &contents).await;
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
async fn create_import_with_repo(dir: &str, path: &str) {
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
    let _path: Option<String> = get_import_url_from_repo(&path).await;
    if _path != None {
        let _path: String = _path.unwrap();

        // Get the import and contents
        let import: String = extract_import_name(&_path);
        let contents: String = get_import_contents(&_path).await;

        // Create new import file
        create_import_file(dir, &import, &contents).await;
    }
}

// The get_import_url_from_repo() function is used to get the
// .sty file download url from the provided github repo.
async fn get_import_url_from_repo(path: &str) -> Option<String> {
    let client: reqwest::Client = reqwest::Client::new();
    let resp = client.get(path)
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36")
        .send().await;
    let resp: reqwest::Response = match resp {
        Ok(r) => r,
        Err(e) => panic!("failed to request provided url. {:?}", e),
    };

    // Get the response body
    let body: String = match resp.text().await {
        Ok(body) => body,
        Err(e) => panic!("failed to extract http response body. {:?}", e),
    };
    let json: Value = match serde_json::from_str(&body) {
        Ok(j) => j,
        Err(e) => panic!("failed to parse lat.data.json. {:?}", e),
    };

    // Convert the json response to an array
    let json: &Vec<Value> = match json.as_array() {
        Some(j) => j,
        None => panic!("failed to parse lat.data.json."),
    };

    // Iterate over the array to find the .sty file
    for map in json {
        let name: Option<&Value> = map.get("name");
        if name != None {
            let name: String = name.unwrap().to_string();
            if name.ends_with(".sty\"") {
                let download_url: Option<&Value> = map.get("download_url");
                if download_url != None {
                    return Some(download_url.unwrap().to_string().replace("\"", ""));
                }
            }
        }
    }
    return None;
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
    // Send the http request to the github url
    let client: reqwest::Client = reqwest::Client::new();
    let resp = client.get(path)
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36")
        .send().await;
    let resp: reqwest::Response = match resp {
        Ok(r) => r,
        Err(e) => panic!("failed to request provided url. {:?}", e),
    };
    return match resp.text().await {
        Ok(body) => body,
        Err(e) => panic!("failed to extract http response body. {:?}", e),
    };
}

// The create_import_file() is used to create the
// new .sty file with the import name and write the
// contents into the file
async fn create_import_file(dir: &str, import: &str, contents: &str) {
    // If the import already exists, return the function.
    if import_already_exists(dir, import) {
        return;
    }

    // Create a new file with the name of the provided import.
    let file = File::create(format!("{}/{}", dir, import));
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
fn import_already_exists(dir: &str, import: &str) -> bool {
    let data = fs::read(format!("{}/{}", dir, import));
    return match data {
        Ok(_) => true,
        Err(_) => false,
    };
}
