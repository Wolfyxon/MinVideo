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
    description: &'a str,
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

fn get_rgb_ansi(r: u8, g: u8, b: u8) -> String {
    return format!("\x1b[38;{};{};{};249m", r, g, b);
}

fn get_options() -> Vec<Option<'static>> {
    return vec![
        Option {
            alias: "help",
            callback: help_option,
            usage: "",
            description: "Shows this message",
            minimum_args: 0
        },

        Option {
            alias: "parse",
            callback: parse_option,
            usage: "<path>",
            description: "Shows info of a video saved in the MinVideo format",
            minimum_args: 1
        }
    ];

}

fn help_option(_args: Vec<String>) {
    println!("MinVideo command line tool");
    println!("Source: https://github.com/Wolfyxon/MinVideo");

    let mut max_option_len = 0;

    for option in get_options() {
        let ln = option.usage.len() + option.alias.len();
        if ln > max_option_len {
            max_option_len = ln;
        }
    }

    println!("\nAvailable options:");

    for option in get_options() {
        let this_len = option.usage.len() + option.alias.len();
        let len_diff = max_option_len - this_len;

        println!("  {}: {}{}: {}", option.alias, option.usage, " ".repeat(len_diff), option.description);
    }

    println!();
}

fn parse_option(args: Vec<String>) {
    let path = args[0].to_string();

    let mut file = File::open(&path).expect(format!("error: File {} not found", path).as_str());
    let metadata = fs::metadata(&path).expect("error: unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("error: buffer overflow");

    let vid = Video::from_data(&buffer);

    println!("Size: {}x{}", vid.get_width(), vid.get_height());
    println!("Frames: {}", vid.get_frame_amount());
}