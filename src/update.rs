use std::fs::{self, File};
use std::io;

// Initialize the update command
pub async fn init() {
    // Send the http request to the github url
    let resp = reqwest::get("https://raw.githubusercontent.com/realTristan/lat.cli/main/lat").await;
    let resp: reqwest::Response = match resp {
        Ok(r) => r,
        Err(e) => panic!("failed to fetch version url. {:?}", e),
    };

    // Get the request body
    let body = resp.text().await;
    let body: String = match body {
        Ok(b) => b,
        Err(e) => panic!("failed to reach version body {:?}", e),
    };

    // Create the new executable file
    let file = File::create("_lat");
    let mut file: File = match file {
        Ok(f) => f,
        Err(e) => panic!("failed to create new version executable. {:?}", e),
    };

    // Copy the request body bytes to the new file
    let res = io::copy(&mut body.as_bytes(), &mut file);
    let res: u64 = match res {
        Ok(r) => r,
        Err(copy_error) => match fs::remove_file("_lat") {
            Ok(_) => panic!(
                "failed to copy response data to new executable. {:?}",
                copy_error
            ),
            Err(rm_error) => panic!("failed to remove temporary _lat executable. {:?}", rm_error),
        },
    };

    // If the copy to the temporary _lat executable
    // was a success...
    if res > 0 {
        match fs::remove_file("lat") {
            Ok(_) => match fs::rename("_lat", "lat") {
                Ok(_) => println!("successfully updated lat.cli"),
                Err(_) => {
                    panic!("failed to convert _lat to lat. use \"_lat update\" to try again.")
                }
            },
            Err(_) => match fs::remove_file("_lat") {
                Ok(_) => panic!("failed to remove existing lat file."),
                Err(_) => panic!("failed to remove existing lat file and temporary lat file. please visit your $PATH to update manually."),
            },
        }
    }
}
