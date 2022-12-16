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
        println!("{}", help_command());
        return;
    }

    // Get the query argument
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
            let path: Option<String> = short::get_long_from_json(bin_path, path);
            if path != None {
                install::init(&path.unwrap()).await;
            }
        }
    }
    // Update Command
    else if query == "-s" || query == "-short" {
        short::init(bin_path, args).await;
    }
    // Update Command
    else if query == "-u" || query == "-update" {
        update::init(bin_path).await;
    }
}

// Help command string
fn help_command() -> String {
    return String::from(
        "
Welcome to lat.cli

    Import Package:
        $ lat -install (github repo url)
        $ lat -install https://github.com/realTristan/realtristan.sty
        $ lat -install (shortcut name)
    
    Create Shortcuts:
        $ lat -short -new (shortcut name) (shortcut path)
        $ lat -short -new rt realTristan/realtristan.sty
        
    List Shortcuts:
        $ lat -short -ls
    
    Delete Shortcuts:
        $ lat -short -remove (shortcut name)
        $ lat -short -empty
    
    Update CLI:
        $ lat -update
    ",
    );
}
