use std::process;

mod times;
mod general;
mod archive;

const NVC_VERSION: &str = "1.0";

fn main() {
    let args: Vec<_> = std::env::args().collect();
    
    if args.len() != 2 {
        println!("{} [folder]", args[0]);
        process::exit(1);
    }

    // Stopwatch
    let boot_systime: i64 = *&times::get_now_unix();

    // Check if directory exists
    let mut path_to_folder = String::from(&args[1]);
    if !general::path_exists(&path_to_folder) {
        let full_path = format!("{}/{}", general::get_cwd(), *&path_to_folder);
        path_to_folder = full_path;
        if !general::path_exists(&path_to_folder) {
            println!("{} does not exist. Syntax: {} [folder]", *&path_to_folder, args[0]);
            process::exit(1);
        }
    }

    println!("NVC {}-rust. Since 4/20/22.", NVC_VERSION);
    match archive::make(&path_to_folder,
         &format!("{}/{}.zip", *&general::get_cwd(), *&times::get_formed_date()), 
         Some(zip::CompressionMethod::Stored).unwrap()) {
        Ok(_) => {
            let finish_time: i64 = *&times::get_now_unix() - *&boot_systime;
            println!("{}.zip created in {}ms", times::get_formed_date(), finish_time);        
        },
        Err(e) => println!("An error occured: {}", e)
    };
}