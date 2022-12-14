use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

//
// Example Import:
//      cargo run install realTristan/realtristan.sty
//

// Main function
#[tokio::main]
async fn main() {
    // Get the provided arguments and query
    let args: Vec<String> = env::args().collect();
    let query: &str = &args[1];

    // If the user's trying to install an import..
    if query == "i" || query == "install" {
        // Get the import path
        let path: &str = &args[2];

        // Get user and import name
        let (user, import) = get_user_and_import(path);

        // Get import file contents
        let contents: String = get_import_contents(user, import).await;

        // Get current working directory
        let dir: &str = &get_current_dir();

        // Create new import file
        create_new_import_file(dir, import, contents)
    }
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
    let resp: reqwest::Response = reqwest::get(path).await.unwrap();
    return resp.text().await.unwrap();
}

// Get the current working directory. This is where
// the folder containing the imports will be located.
// %CURRENT_DIR%/file.sty..
fn get_current_dir() -> String {
    let res = env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "failed to get working directory".to_string(),
    }
}

// The create_new_import_file() is used to create the
// new .sty file with the import name and write the
// contents into the file
fn create_new_import_file(dir: &str, import: &str, contents: String) {
    if import_already_exists(dir, import.to_string()) {
        return;
    }
    let file = File::create(format!("{}/{}", dir, import));
    let mut file: File = file.unwrap();
    file.write_all(contents.as_bytes())
        .expect("failed to write to import file");
}

// Check if the import already exists. This function
// is required to avoid file errors. I might change it
// to overwrite the current file with the new import depending
// on what's more convenient.
fn import_already_exists(dir: &str, import: String) -> bool {
    let data = fs::read(format!("{}/{}", dir, import));
    match data {
        Ok(_) => true,
        Err(_) => false,
    }
}
