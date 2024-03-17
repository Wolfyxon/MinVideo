pub const VIDEO_SIZE_BYTE_LENGTH: u32 = 8;
pub const VIDEO_MAX_DIMENSION: u32 = VIDEO_SIZE_BYTE_LENGTH * 255;
pub const BYTES_BEFORE_FRAMES: u32 = VIDEO_SIZE_BYTE_LENGTH * 2;

// NOTE: Do not just translate Python code, this should be remade from ground up to use less memory and be faster (the Python version too).


// Frame

pub struct Frame {
    data: Vec<u8>,
    width: u32,
    height: u32
} 

impl Frame {

    // Constructor

    pub fn new(width: u32, height: u32) -> Self {
        Frame {
            data: Vec::new(),
            width: width,
            height: height
        }
    }

    // Dynamic functions

    pub fn set_color(&mut self, x: u32, y: u32, rgb: (u8, u8, u8)) {
        assert!(x <= self.width, "X out of range");
        assert!(y <= self.height, "Y out of range");

        let begin = (x * y) as usize;
        let (r, g, b) = rgb;

        assert!(begin + 2 < self.data.len(), "End color index out of range, this isn't supposed to happen!");

        self.data[begin] = r;
        self.data[begin + 1] = g;
        self.data[begin + 2] = b;
        
    }

}

// Video class

pub struct Video {
    data: Vec<u8>,
    width: u32,
    height: u32
}

impl Video {

    // Constructor

    pub fn new(width: u32, height: u32) -> Self {
        Video {
            data: Vec::new(),
            width: width,
            height: height
        }
    }

    // Dynamic functions

    pub fn add_frame(self, frame: &Frame) {
        assert!(frame.width == self.width, "Frame width not equal to video width");
        assert!(frame.height == self.height, "Frame height not equal to video height")
    }

    // Static functions

    fn get_width_from_data(data: &Vec<u8>) -> u32 {
        let mut res: u32 = 0;

        for i in data.iter().take(VIDEO_SIZE_BYTE_LENGTH as usize) {
            res += *i as u32;
        }

        return res;
    }

    fn get_height_from_data(data: &Vec<u8>) -> u32 {
        let mut res: u32 = 0;

        for i in data.iter().skip(VIDEO_SIZE_BYTE_LENGTH as usize).take(VIDEO_SIZE_BYTE_LENGTH as usize) {
            res += *i as u32;
        }

        return res;
    }

    fn get_frame_amount_from_data(data: &Vec<u8>) -> usize {
        let w = Video::get_width_from_data(data);
        let h = Video::get_height_from_data(data);
        
        return (data.len() - BYTES_BEFORE_FRAMES as usize) / 3 / (w * h) as usize;
    }
}

// Functions

pub fn dimension_split(dimension: u32) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    if dimension != 0 {
        let count = (dimension as f64).ceil() as u32;

        for _ in 1..count {
            res.push( (dimension / count) as u8 );
        }

        for i in 1 .. dimension % count {
            res[i as usize] += 1;
        }
    }

    return res;
} 

pub fn get_coords_at_idx(index: u32, width: u32, height: u32) -> (u32, u32) {
    let x = index % width;
    let y = (index / width) % height;

    return (x, y)
}