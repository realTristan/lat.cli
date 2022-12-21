// Define the request client as a global variable
lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

// Send an http request to the provided url
pub(crate) async fn http_get(url: &str) -> reqwest::Response {
    return match CLIENT.get(url)
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/108.0.0.0 Safari/537.36")
        .send().await 
    {
        Ok(r) => r,
        Err(e) => panic!("failed to request provided url. {:?}", e),
    };
}