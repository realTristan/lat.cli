use serde_json::Value;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

// Initialize the update command
pub async fn init(path: &str) {
    // Get current working directory
    let dir: Option<String> = get_current_dir();
    let dir: String = match dir {
        Some(d) => d,
        None => panic!("failed to get current working directory."),
    };

    // If the path contains http (a url)
    if path.contains("http") {
        if !path.contains("github") {
            panic!("invalid github url.")
        }
        return import_with_url(&dir, path).await;
    }
    import_no_url(&dir, path).await;
}

// The import_with_url() function is used to
// import the provided file using the github
// url the user provided.
async fn import_with_url(dir: &str, path: &str) {
    // Split the path to get the import name
    let split_path: Vec<&str> = path.split("/").collect();

    // If the provided url is just the repo url...
    if split_path.len() < 5 {
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
    let import: String = extract_import_name(split_path);
    let contents: String = get_import_contents(&_path).await;

    // Create new import file
    create_import_file(dir, &import, &contents).await;
}

// The import_no_url() function is used to create
// a new import file if the path is formatted as
// github_user/github_repo
async fn import_no_url(dir: &str, path: &str) {
    // split_path the provided path to get the github user
    // and the import repository.
    let split_path: Vec<&str> = path.split("/").collect();
    let user: &str = split_path[0];
    let import: &str = split_path[1];

    // Create a new raw github path using the user and import
    let path: &str = &format!(
        "https://raw.githubusercontent.com/{}/{}/main/{}",
        user, import, import
    );

    // Get import file contents
    let contents: String = get_import_contents(path).await;

    // Create new import file
    create_import_file(dir, import, &contents).await;
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
    let mut _path: String = path.to_string();

    // If the provided url has .git on the end of it
    if _path.ends_with(".git") || _path.ends_with(".git/") {
        let index = path.find(".git");
        if index != None {
            // Remove the .git from the url
            _path = path[..index.unwrap()].to_string();
        }
    }
    let _path_: Option<String> = get_import_url_from_repo(&_path).await;
    if _path_ != None {
        let _path_: String = _path_.unwrap();

        // Split the path to get the import name
        let split_path: Vec<&str> = _path_.split("/").collect();

        // Get the import and contents
        let import: String = extract_import_name(split_path);
        let contents: String = get_import_contents(&_path_).await;

        // Create new import file
        create_import_file(dir, &import, &contents).await;
    }
}

// The get_import_url_from_repo() function is used to get the
// .sty file download url from the provided github repo.
async fn get_import_url_from_repo(path: &str) -> Option<String> {
    let resp = reqwest::get(path).await;
    let resp: reqwest::Response = match resp {
        Ok(r) => r,
        Err(e) => panic!("failed to request provided url. {:?}", e),
    };
    let text = resp.text().await;
    let text: String = match text {
        Ok(text) => text,
        Err(e) => panic!("failed to extract http response text. {:?}", e),
    };
    let json: Value = match serde_json::from_str(&text) {
        Ok(j) => j,
        Err(e) => panic!("failed to parse lat.data.json. {:?}", e),
    };

    for j in json.as_object() {
        let name: Option<&Value> = j.get("name");
        if name != None {
            if name.unwrap().to_string().ends_with(".sty") {
                let download_url: Option<&Value> = j.get("download_url");
                if download_url != None {
                    return Some(download_url.unwrap().to_string());
                }
            }
        }
    }
    return None;
}

// The extract_import_name() function is used
// to extract the import name of the provided path.
fn extract_import_name(split_path: Vec<&str>) -> String {
    // Get the import name using the split path array
    let import: String;
    if split_path[split_path.len() - 1].len() > 0 {
        import = split_path[split_path.len() - 1].to_string();
    } else {
        import = split_path[split_path.len() - 2].to_string();
    }
    return import;
}

// The get_import_contents() function is used to get
// the raw file content from the github repo using the
// provided path.
async fn get_import_contents(path: &str) -> String {
    // Send the http request to the github url
    let resp = reqwest::get(path).await;
    let resp: reqwest::Response = match resp {
        Ok(r) => r,
        Err(e) => panic!("failed to request provided url. {:?}", e),
    };
    let text = resp.text().await;
    return match text {
        Ok(text) => text,
        Err(e) => panic!("failed to extract http response text. {:?}", e),
    };
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
