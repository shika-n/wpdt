use std::{env, fs::read_to_string, io::Error, path, process::exit};

fn find_config(target_file: &str) -> Result<String, Error> {
    let mut path = path::absolute(target_file)?;
    loop {
        let parent = path.parent();
        if let None = parent {
            break;
        }
        let entries = parent.unwrap().read_dir()?;
        for entry_result in entries {
            let entry = entry_result?;
            if entry.file_name() == "offsets.conf" {
                return Ok(entry.path().to_string_lossy().to_string());
            }
        }
        path = parent.unwrap().to_path_buf();
    }

    return Err(Error::new(
        std::io::ErrorKind::NotFound,
        "Config file not found",
    ));
}

fn get_config_for(config_file: &str, target_file: &str) -> Result<String, Error> {
    let configs = read_to_string(path::absolute(config_file)?)?;

    let filepath = path::absolute(target_file)?;

    let filename = filepath.file_name();
    if let None = filename {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Failed to get filename",
        ));
    }

    for config in configs.lines() {
        if config.starts_with(filename.unwrap().to_string_lossy().to_string().as_str()) {
            return Ok(config.to_owned());
        }
    }

    return Ok("No config available for file".to_owned());
}

fn main() {
    let full_args: Vec<String> = env::args().collect();
    if full_args.len() <= 1 {
        println!("No file given");
        exit(1);
    }

    let target_file: &str = &full_args[1];
    let mut output_folder: &str = "";

    let args = &full_args[2..];
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-o" => {
                i += 1;
                if i >= args.len() {
                    println!("No output directory given");
                    exit(1);
                }
                output_folder = &args[i];
            }
            _ => {
                println!("Unknown argument: {}", arg);
                exit(1);
            }
        }

        i += 1;
    }

    if target_file.is_empty() || output_folder.is_empty() {
        println!("Input file and output folder required");
        exit(1);
    }

    let config_path = find_config(target_file);
    if let Err(err) = config_path {
        println!("Error: {}", err);
        exit(1);
    }

    let config = get_config_for(config_path.as_ref().unwrap(), target_file);
    if let Err(err) = config {
        println!("Error: {}", err);
        exit(1);
    }

    println!("Config: {}", config_path.unwrap());
    println!("Val: {}", config.unwrap());

    println!("{} -> {}", target_file, output_folder);
}
