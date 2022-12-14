use serde_json::{Map, Value};
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
};

// The read_json() function is used to read
// the current data from the lat.data.json file.
// The function returns the serde_json::Value
// provided to us by the serde_json::from_str()
// function.
fn read_json(bin_path: &str) -> Value {
    let data: String = match fs::read_to_string(format!("{bin_path}/lat.data.json")) {
        Ok(d) => d,
        Err(_) => match File::create(format!("{bin_path}/lat.data.json")) {
            Ok(_) => "{}".to_string(),
            Err(e) => panic!("failed to parse lat.data.json {:?}", e),
        },
    };

    // Convert the data to a serde_json value
    let json: Value = match serde_json::from_str(&data) {
        Ok(j) => j,
        Err(e) => panic!("failed to parse lat.data.json. {:?}", e),
    };
    return json;
}

// The add_short_to_json() function adds a new short into
// the lat.data.json file. To do this, we read the current json
// data then set the short key with the long value inside
// the returned map. Then, we write the data to the file.
fn add_short_to_json(bin_path: &str, short: &str, long: &str) {
    let mut json: Value = read_json(bin_path);

    // Convert the provided long string to a serde_json value
    let long: Value = match serde_json::to_value(long) {
        Ok(l) => l,
        Err(e) => panic!("failed to convert long to value. {:?}", e),
    };

    // Update the json data
    json[short] = long;

    // Get the lat.data.json file
    let file: File = match File::create(format!("{bin_path}/lat.data.json")) {
        Ok(f) => f,
        Err(e) => panic!("failed to read lat.data.json. {:?}", e),
    };

    // Create a new writer for writing to the json file.
    let mut writer: BufWriter<File> = BufWriter::new(file);
    match serde_json::to_writer(&mut writer, &json) {
        Ok(_) => match writer.flush() {
            Ok(_) => println!("successfully created new short: '{}'", short),
            Err(e) => panic!("failed to flush lat.data.json. {:?}", e),
        },
        Err(e) => panic!("failed to save new short to lat.data.json. {:?}", e),
    };
}

// The list_shorts() function is used to list
// all of the current shorts inside the lat.data.json
// file. This function is required for determining
// which shorts you need to delete.
fn list_shorts(bin_path: &str) {
    // Get the current json data
    let json: Value = read_json(bin_path);
    let json: &Map<String, Value> = match json.as_object() {
        Some(j) => j,
        None => panic!("failed to read lat.data.json as object."),
    };

    // Print out all of the shorts
    for (key, value) in json {
        println!("{}: {}", key, serde_json::to_string(value).unwrap())
    }
}

// The empty_short_json() function is used to delete
// all of the shorts that are inside of the lat.data.json
// file. This command would be used to prevent too
// many shorts from being made.
fn empty_short_json(bin_path: &str) {
    // Get the lat.data.json file
    let file: File = match File::create(format!("{bin_path}/lat.data.json")) {
        Ok(f) => f,
        Err(e) => panic!("failed to read lat.data.json. {:?}", e),
    };

    // Create a new writer for writing to the json file.
    let mut writer: BufWriter<File> = BufWriter::new(file);
    match serde_json::to_writer(&mut writer, &Map::new()) {
        Ok(_) => match writer.flush() {
            Ok(_) => println!("successfully removed all shorts"),
            Err(e) => panic!("failed to flush lat.data.json. {:?}", e),
        },
        Err(e) => panic!("failed to save new short to lat.data.json. {:?}", e),
    };
}

// The remove_short_from_json() function is used
// to remove a shortcut from the lat.data.json file.
fn remove_short_from_json(bin_path: &str, short: &str) {
    // Get the current json data
    let json: Value = read_json(bin_path);
    let json: &mut Map<String, Value> = &mut match json.as_object() {
        Some(obj) => obj.to_owned(),
        None => panic!("failed to read lat.data.json as object."),
    };

    // Remove the shortcut from the map
    match json.remove(short) {
        Some(_) => println!("'{short}' removed from cache map... awaiting file update..."),
        None => panic!("failed to remove short from lat.data.json"),
    };

    // Get the lat.data.json file
    let file: File = match File::create(format!("{bin_path}/lat.data.json")) {
        Ok(f) => f,
        Err(e) => panic!("failed to read lat.data.json. {:?}", e),
    };

    // Create a new writer for writing to the json file.
    let mut writer: BufWriter<File> = BufWriter::new(file);
    match serde_json::to_writer(&mut writer, &json) {
        Ok(_) => match writer.flush() {
            Ok(_) => println!("successfully removed short: '{}'", short),
            Err(e) => panic!("failed to flush lat.data.json. {:?}", e),
        },
        Err(e) => panic!("failed to remove short to lat.data.json. {:?}", e),
    };
}

// The get_long_from_json() function is used to
// get the long version of the provided short which
// will be used by the install functions.
pub fn get_long_from_json(bin_path: &str, short: &str) -> Option<String> {
    let json: Value = read_json(bin_path);
    if let Some(long) = json[short].as_str() {
        return Some(long.to_string());
    }
    return None;
}

// The init() function is used to initialize
// the -short commands. If the user wants to
// create a new short, delete a short, list all
// shorts, or remove all shorts, this is where
// the specific functions are called.
pub async fn init(bin_path: &str, args: Vec<String>) {
    let action: &str = &args[2];

    // Add a short
    if action == "-n" || action == "-new" {
        if args.len() < 5 {
            println!("not enough arguments provided. ex: lat -short -new (shortcut_name) (shortcut_path)");
            return;
        }
        let short: &str = &args[3];
        let long: &str = &args[4];
        add_short_to_json(bin_path, short, long);
    }
    // List all shorts
    else if action == "-ls" || action == "-list" {
        list_shorts(bin_path);
    }
    // Delete all shorts
    else if action == "-empty" {
        empty_short_json(bin_path);
    }
    // Delete a short
    else if action == "-rm" || action == "-remove" {
        if args.len() < 4 {
            println!("not enough arguments provided. ex: lat -short -rm (shortcut_name)");
            return;
        }
        let short: &str = &args[3];
        remove_short_from_json(bin_path, short);
    }
}

// ./lat -s -n rt realTristan/realtristan.sty
// ./lat -s -ls
// ./lat -s -empty
