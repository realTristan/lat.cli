use std::env;
mod install;
mod update;

//
// Example Import:
//      cargo run -- -install realTristan/realtristan.sty --release
//
//      or
//
//      lat -install realTristan/realtristan.sty
//

//
// Example Update:
//      cargo run -- -update --release
//
//      or
//
//      lat -update
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

    // Install command
    if query == "-i" || query == "-install" {
        if args.len() < 3 {
            println!("not enough arguments provided. ex: lat install realTristan/realtristan.sty");
            return;
        }
        install::init(&args).await;
    }
    // Update Command
    else if query == "-u" || query == "-update" {
        update::init().await;
    }
}
