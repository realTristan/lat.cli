use std::env;
mod install;
mod short;
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
    // Get the bin folder path buf
    let path_buf = match env::current_exe() {
        Ok(buf) => buf,
        Err(e) => panic!("failed to fetch lat $PATH. {:?}", e),
    };

    // Get the bin folder ($PATH)
    let bin_path: &str = match path_buf.parent() {
        Some(path) => path.to_str().unwrap().clone(),
        None => panic!("failed to unwrap lat $PATH."),
    };

    // Get the provided arguments
    let args: Vec<String> = env::args().collect();

    // Get the query (install, i, etc.)
    if args.len() < 2 {
        println!(
            "\nWelcome to lat.cli\n\n  Import Package:\n    $ lat -install (github_user)/(repo_name.sty)\n    $ lat -install realTristan/realtristan.sty\n\n  Create Shortcut:\n    $ lat -short -new (shortcut_name) (shortcut_path)\n    $ lat -short -new rt realTristan/realtristan.sty\n\n  List Shortcuts:\n    $ lat -short -ls\n\n  Clear Shortcuts:\n    $ lat -short -empty\n\n  Update CLI:\n    $ lat -update\n"
        );
        return;
    }
    let query: &str = &args[1];

    // Install command
    if query == "-i" || query == "-install" {
        if args.len() < 3 {
            println!("not enough arguments provided. ex: lat -install realTristan/realtristan.sty");
            return;
        }
        let path: &str = &args[2];

        // Else if the path contains just one /
        if path.contains("/") {
            install::init(path).await;
        }
        // Else, the provided is a short..
        else {
            let path: String = short::get_long_from_json(bin_path, path);
            install::init(&path).await;
        }
    }
    // Update Command
    else if query == "-s" || query == "-short" {
        short::init(bin_path, args).await;
    }
    // Update Command
    else if query == "-u" || query == "-update" {
        update::init().await;
    }
}
