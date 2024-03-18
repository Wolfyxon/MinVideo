mod min_video;
use min_video::Video;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;

struct Option<'a> {
    alias: &'a str,
    callback: fn(args: Vec<String>),
    usage: &'a str,
    minimum_args: usize
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help_option(Vec::new());
        return;
    }

    let alias = args[1].as_str();
    
    let mut cmd_args = args.clone();
    cmd_args.remove(0);
    cmd_args.remove(0);
    

    for option in get_options() {
        if alias == option.alias {
            if cmd_args.len() < option.minimum_args {
                println!("error: This option requires at least {} arguments", option.minimum_args);
                return;
            }

            (option.callback)(cmd_args);
            return;
        }
    }

    println!("error: Unrecognized option: {}", alias);
    
}

fn get_options() -> Vec<Option<'static>> {
    return vec![
        Option {
            alias: "help",
            callback: help_option,
            usage: "",
            minimum_args: 0
        },

        Option {
            alias: "parse",
            callback: parse_option,
            usage: "<path>",
            minimum_args: 1
        }
    ];

}

fn help_option(_args: Vec<String>) {
    println!("MinVideo command line tool");
    println!("Source: https://github.com/Wolfyxon/MinVideo");
}

fn parse_option(args: Vec<String>) {
    let path = args[0].to_string();

    let mut file = File::open(&path).expect(format!("error: File {} not found", path).as_str());
    let metadata = fs::metadata(&path).expect("error: unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("error: buffer overflow");

    let w = Video::get_width_from_data(&buffer);
    let h = Video::get_height_from_data(&buffer);
    let frames = Video::get_frame_amount_from_data(&buffer);

    println!("Size: {}x{}", w, h);
    println!("Frames: {}", frames);
}