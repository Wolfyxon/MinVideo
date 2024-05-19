mod min_video;
use min_video::Video;
use sdl2::rect::Point;

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
    imgproc
};

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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
            alias: "play",
            callback: play_option,
            usage: "<path> [--invert]",
            description: "[NOT IMPLEMENTED] Plays a video in a graphical window",
            minimum_args: 1
        },

        Option {
            alias: "play_text",
            callback: play_text_option,
            usage: "<path> [--invert]",
            description: "Plays a video in the terminal (some terminals might not display it correctly)",
            minimum_args: 1
        },

        Option {
            alias: "convert",
            callback: convert_option,
            usage: "<input path> [output path] [width] [height]",
            description: "Converts a standard video to the MinVideo format",
            minimum_args: 1
        },

        Option {
            alias: "invert",
            callback: invert_option,
            usage: "<input path> [output path (default: input_path)]",
            description: "Inverts a MinVideo video",
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
    println!("Buffer size: {}", vid.get_data().len());
}

fn play_text_option(args: Vec<String>) {
    let path = args[0].to_string();

    let mut invert = false;

    if args.len() > 1 {
        invert = args[1] == "--invert";
    }

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

                if invert {
                    print!("{}█", get_rgb_ansi(b, g, r));
                } else {
                    print!("{}█", get_rgb_ansi(r, g, b));
                }
                
            }

            println!();
        }
    }
}

fn play_option(args: Vec<String>) {
    let path = args[0].to_string();

    let mut invert = false;

    if args.len() > 1 {
        invert = args[1] == "--invert";
    }

    let mut file = File::open(&path).expect(format!("error: File {} not found", path).as_str());
    let metadata = fs::metadata(&path).expect("error: unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("error: buffer overflow");

    let mut vid = Video::from_data(&buffer);
    let w = vid.get_width();
    let h = vid.get_height();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("MinVideo Renderer", w, h)
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let frame_duration = Duration::from_secs_f64(1.0 / 30.0);
    let mut prev_frame = SystemTime::now();

    for frame_i in 0..vid.get_frame_amount() {
        // Quit request handling
        let mut quit = false;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => quit = true,
                _ => {}
            }
        }

        if quit { break; }

        // FPS limit
        let now = SystemTime::now();
        let elapsed = now.duration_since(prev_frame).unwrap_or_default();
    
        if elapsed < frame_duration {
            sleep(frame_duration - elapsed);
        }
        prev_frame = SystemTime::now();

        // Rendering

        let (windowW, windowH) = canvas.window().size();

        let frame = vid.get_frame(frame_i);

        for y in 0..h {
            for x in 0..w {
                let (r, g, b) = frame.get_color(x, y);
                
                if invert { canvas.set_draw_color(Color::RGB(b, g, r)); } 
                else      { canvas.set_draw_color(Color::RGB(r, g, b)); }

                canvas.set_scale(
                    windowW as f32 / w as f32, 
                    windowH as f32 / h as f32
                ).unwrap();

                canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
            }
        }

        canvas.present();
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

    let mut mv = min_video::Video::new(target_w as u32, target_h as u32);

    loop {

        let mut perc = ((current_frame + 1) as f64 / frame_count as f64 * 100.0) as usize;
        if perc > 100 {
            perc = 100;
        }

        let hashes = "#".repeat( perc );
        let dashes = "-".repeat( 100 - perc );

        print!("\r[{}{}] {}/{}", hashes, dashes, current_frame + 1,  frame_count);
        current_frame += 1;

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

        let mut mv_frame = min_video::Frame::new(target_w as u32, target_h as u32);

        for y in 0..frame.rows() {
            for x in 0..frame.cols() {
                let px = frame.at_2d::<opencv::core::Vec3b>(y, x).unwrap();

                // IMPORTANT: OpenCV uses the BGR format not RGB!!!
                let r = px[2];
                let g = px[1];
                let b = px[0];

                mv_frame.set_color(x as u32, y as u32, (r, g, b));
            }
        }

        mv.add_frame(&mv_frame);
    }

    output_file.write_all(&mv.get_data()).expect("Failed to write file");

    println!("\nDone");
}

fn invert_option(args: Vec<String>) {
    let input_path = args[0].to_string();
    let output_path = args.get(1).unwrap_or( &input_path );

    let mut file = File::open(&input_path).expect(format!("error: File {} not found", input_path).as_str());
    let metadata = fs::metadata(&input_path).expect("error: unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("error: buffer overflow");

    let mut output_file = fs::OpenOptions::new().write(true).create(true).open(output_path).expect("error: Cannot open output file");

    let mut vid = Video::from_data(&buffer);
    let w = vid.get_width();
    let h = vid.get_height();

    println!("Inverting {} and saving it as {}", input_path, output_path);

    let frame_count = vid.get_frame_amount();
    let mut current_frame = 0;

    for frame_i in 0..vid.get_frame_amount() {
        let mut perc = ((current_frame + 1) as f64 / frame_count as f64 * 100.0) as usize;
        if perc > 100 {
            perc = 100;
        }

        let hashes = "#".repeat( perc );
        let dashes = "-".repeat( 100 - perc );

        print!("\r[{}{}] {}/{}", hashes, dashes, current_frame + 1,  frame_count);
        current_frame += 1;

        let mut frame = vid.get_frame(frame_i);

        for y in 0..h {
            for x in 0..w {
                let (r, g, b) = frame.get_color(x, y);
                frame.set_color(x, y, (b, g, r));
            }
        }

        vid.put_frame(&frame, frame_i);
    }

    output_file.write_all(&vid.get_data()).expect("Failed to write file");
    println!("Inverting complete")
}