mod min_video;
use min_video::Video;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::time::{Duration, SystemTime};
use std::thread::sleep;

const RECOMMENDED_WIDTH: i32 = 128;
const RECOMMENDED_HEIGHT: i32 = 96;

extern crate opencv;
use opencv::{
    prelude::*,
    videoio,
    highgui,
    imgproc
};

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
    return format!("\x1b[38;2;{};{};{}m", r, g, b);
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
        },

        Option {
            alias: "play_text",
            callback: play_text_option,
            usage: "<path>",
            description: "Plays a video in the terminal (some terminals might not display it correctly)",
            minimum_args: 1
        },

        Option {
            alias: "convert",
            callback: convert_option,
            usage: "<input path> [output path] [width] [height]",
            description: "Converts a standard video to the MinVideo format",
            minimum_args: 1
        }
    ];

}

fn help_option(_args: Vec<String>) {
    println!("MinVideo command line tool (Rust version)");
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

fn play_text_option(args: Vec<String>) {
    let path = args[0].to_string();

    let mut file = File::open(&path).expect(format!("error: File {} not found", path).as_str());
    let metadata = fs::metadata(&path).expect("error: unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("error: buffer overflow");

    let mut vid = Video::from_data(&buffer);
    let w = vid.get_width();
    let h = vid.get_height();

    let frame_duration = Duration::from_secs_f64(1.0 / 30.0);
    let mut prev_frame = SystemTime::now();

    for frame_i in 0..vid.get_frame_amount() {

        // FPS limit
        let now = SystemTime::now();
        let elapsed = now.duration_since(prev_frame).unwrap_or_default();
    
        if elapsed < frame_duration {
            sleep(frame_duration - elapsed);
        }
        prev_frame = SystemTime::now();

        // Rendering

        let frame = vid.get_frame(frame_i);

        print!("\x1B[2J\x1B[1;1H"); // Clear the terminal

        for y in 0..h {
            for x in 0..w {
                let (r, g, b) = frame.get_color(x, y);
                print!("{}█", get_rgb_ansi(r, g, b));
            }

            println!();
        }
    }
}

fn convert_option(args: Vec<String>) {
    let input_path = &args[0];
    let default_output_path = format!("{}.minv", input_path).to_string();
    let output_path = args.get(1).unwrap_or( &default_output_path );

    let mut output_file = fs::OpenOptions::new().write(true).create(true).open(output_path).expect("error: Cannot open output file");

    let mut cap = videoio::VideoCapture::from_file(&input_path, videoio::CAP_ANY).expect("error: Unable to open input file");
    
    let cap_w = cap.get(videoio::CAP_PROP_FRAME_WIDTH).unwrap() as i32;
    let cap_h = cap.get(videoio::CAP_PROP_FRAME_HEIGHT).unwrap() as i32;

    let target_w = args.get(2).map_or(RECOMMENDED_WIDTH, |s| s.parse().unwrap_or(RECOMMENDED_WIDTH));
    let target_h = args.get(3).map_or(RECOMMENDED_HEIGHT, |s| s.parse().unwrap_or(RECOMMENDED_HEIGHT));


    let frame_count = cap.get(videoio::CAP_PROP_FRAME_COUNT).unwrap();
    let mut current_frame = 0;

    println!("Converting and resizing video");
    println!("{}x{} -> {}x{}", cap_w, cap_h, target_w, target_h);

    let mut buf: Vec<u8> = Vec::new();

    buf.extend( min_video::dimension_split(target_w as u32) );
    buf.extend( min_video::dimension_split(target_h as u32) );
    

    loop {
        let mut frame = Mat::default();

        cap.read(&mut frame).unwrap();

        if frame.size().unwrap().width == 0 { break; } // No frames left to read

        if cap_w != target_w || cap_h != target_h {
            let mut resized_frame = Mat::default();

            imgproc::resize(&frame, &mut resized_frame, opencv::core::Size {
                width: target_w,
                height: target_h
            }, 0.0, 0.0, imgproc::INTER_LINEAR).unwrap();

            frame = resized_frame;
        }

        for y in 0..frame.rows() {
            for x in 0..frame.cols() {
                let px = frame.at_2d::<opencv::core::Vec3b>(y, x).unwrap();
                buf.extend([px[0], px[1], px[2]]);
            }
        }

        print!("\r{}/{}", current_frame + 1,  frame_count);
        current_frame += 1;
    }

    output_file.write_all(&buf).expect("Failed to write file");

    println!("\nDone");
}