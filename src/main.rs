use std::env;
mod install;

//
// Example Import:
//      cargo run install realTristan/realtristan.sty --release
//
//      or
//
//      lat install realTristan/realtristan.sty
//

// Main function
#[tokio::main]
async fn main() {
    // Get the provided arguments and query
    let args: Vec<String> = env::args().collect();

    // Get the query (install, i, etc.)
    if args.len() < 2 {
        println!("not enough arguments provided. ex: lat install realTristan/realtristan.sty");
        return;
    }
    let query: &str = &args[1];

    // If the user's trying to install an import..
    if query == "i" || query == "install" {
        if args.len() < 3 {
            println!("not enough arguments provided. ex: lat install realTristan/realtristan.sty");
            return;
        }
        install::init(&args).await;
    }
}
