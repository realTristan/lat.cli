use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

// Initialize the update command
pub async fn init(path: &str) {
    // Get user and import name
    let (user, import) = get_user_and_import(path);

    // Get import file contents
    let contents: String = get_import_contents(user, import).await;

    // Get current working directory
    let dir: Option<String> = get_current_dir();
    if dir == None {
        println!("failed to get current working directory.");
        return;
    }
    let dir: &str = &dir.unwrap();

    // Create new import file
    create_new_import_file(dir, import, contents).await;
}

// The get_user_and_import() function is used to get the
// github user and the import name. If you want to make
// your own import, create the repo with the same name as
// your .sty file.
fn get_user_and_import(path: &str) -> (&str, &str) {
    let split: Vec<&str> = path.split("/").collect();
    let user: &str = split[0];
    let import: &str = split[1];
    return (user, import);
}

// The get_import_contents() function is used to get
// the raw file content from the github repo using the
// provided user and import name.
async fn get_import_contents(user: &str, import: &str) -> String {
    let path: String = format!(
        "https://raw.githubusercontent.com/{}/{}/main/{}",
        user, import, import
    );

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

// The create_new_import_file() is used to create the
// new .sty file with the import name and write the
// contents into the file
async fn create_new_import_file(dir: &str, import: &str, contents: String) {
    // If the import already exists, return the function.
    if import_already_exists(dir, import.to_string()) {
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
fn import_already_exists(dir: &str, import: String) -> bool {
    let data = fs::read(format!("{}/{}", dir, import));
    return match data {
        Ok(_) => true,
        Err(_) => false,
    };
}
