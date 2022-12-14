use std::fs::File;
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
        Err(e) => panic!("failed to fetch version body {:?}", e),
    };

    // Open the current lat executable file
    let file = File::open("lat");
    let mut file: File = match file {
        Ok(f) => f,
        Err(e) => panic!("failed to open current lat version. {:?}", e),
    };

    // Copy the request body bytes to the new file
    let res = io::copy(&mut body.as_bytes(), &mut file);
    let res: u64 = match res {
        Ok(r) => r,
        Err(e) => panic!("failed to copy response data to lat executable. {:?}", e),
    };

    // If the update was a success...
    if res > 0 {
        println!("successfully updated lat.cli")
    }
    // Else, if the update was not a success...
    else {
        println!("something went wrong... please try updating again.")
    }
}
