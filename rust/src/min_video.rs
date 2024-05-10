pub const VIDEO_SIZE_BYTE_LENGTH: u32 = 8;
pub const VIDEO_MAX_DIMENSION: u32 = VIDEO_SIZE_BYTE_LENGTH * 255;
pub const BYTES_BEFORE_FRAMES: u32 = VIDEO_SIZE_BYTE_LENGTH * 2;

// Frame

pub struct Frame {
    data: Vec<u8>,
    width: u32,
    height: u32
} 

impl Frame {

    // Constructor

    pub fn new(width: u32, height: u32) -> Self {
        assert!(width > 0, "Width must be greater than 0");
        assert!(height > 0, "Height must be greater than 0");
        assert!(width <= VIDEO_MAX_DIMENSION, "Width exceeds the maximum width");
        assert!(height <= VIDEO_MAX_DIMENSION, "Width exceeds the maximum height");

        return Frame::from_data(width, height, vec![0].repeat( (width * height * 3) as usize));
    }

    pub fn from_data(width: u32, height: u32, data: Vec<u8>) -> Self {
        assert!(data.len() >= BYTES_BEFORE_FRAMES as usize, "Data too short - does not include width and height");
        assert!(data.len() == (width * height * 3) as usize, "Data length does not match width * height * 3");

        Frame {
            data: data,
            width: width,
            height: height
        }
    }

    // Dynamic functions

    pub fn set_color(&mut self, x: u32, y: u32, rgb: (u8, u8, u8)) {
        assert!(x <= self.width, "X out of range");
        assert!(y <= self.height, "Y out of range");

        let begin = get_idx_at_coords(x, y, self.width) as usize * 3;
        let (r, g, b) = rgb;

        assert!(begin + 2 < self.data.len(), "End color index out of range, this isn't supposed to happen!");

        self.data[begin + 0] = r;
        self.data[begin + 1] = g;
        self.data[begin + 2] = b;
        
    }

    pub fn get_color(&self, x: u32, y: u32) -> (u8, u8, u8) {
        assert!(x <= self.width, "X out of range");
        assert!(y <= self.height, "Y out of range");

        let begin = get_idx_at_coords(x, y, self.width) as usize * 3;

        // TEMPORARY FIX
        // TODO: Find the cause why the colors are inverted
        let b = self.data[begin + 0];
        let g = self.data[begin + 1];
        let r = self.data[begin + 2];

        return (r, g, b);
    }

    pub fn get_data(&self) -> &Vec<u8> {
        return &self.data;
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
        assert!(width > 0, "Width must be greater than 0");
        assert!(height > 0, "Height must be greater than 0");
        
        Video {
            data: dimension_split(width).iter().chain(dimension_split(height).iter()).cloned().collect(),
            width: width,
            height: height
        }
    }

    /// Constructs a video from a data buffer
    pub fn from_data(data: &Vec<u8>) -> Self {
        assert!(Video::is_data_valid(data), "Not a valid MinVideo data");

        let w = Video::get_width_from_data(data);
        let h = Video::get_height_from_data(data);

        assert!(w > 0, "Data invalid, width is 0");
        assert!(h > 0, "Data invalid, width is 0");
        
        Video {
            data: data.to_vec(),
            width: w,
            height: h
        }
    }

    // Dynamic functions

    /// Adds a frame to the video
    pub fn add_frame(&mut self, frame: &Frame) {
        assert!(frame.width == self.width, "Frame width not equal to video width");
        assert!(frame.height == self.height, "Frame height not equal to video height");

        self.data.extend(frame.data.iter());
    }

    /// Returns the frame at the specified index
    pub fn get_frame(&mut self, index: usize) -> Frame {
        let begin = BYTES_BEFORE_FRAMES + (self.width * self.height * 3) * index as u32;
        let end = begin + (self.width * self.height * 3);

        let frame_data = self.data[begin as usize..end as usize].to_vec();
        return Frame::from_data(self.width, self.height, frame_data);
    }

    /// Returns the video's data buffer
    pub fn get_data(&self) -> Vec<u8> {
        return self.data.clone();
    }

    /// Returns the amount of frames in the video
    pub fn get_frame_amount(&self) -> usize {
        return Video::get_frame_amount_from_data(&self.data);
    }

    /// Returns the video width
    pub fn get_width(&self) -> u32 {
        return self.width;
    }

    /// Returns the video height
    pub fn get_height(&self) -> u32 {
        return self.height;
    }

    // Static functions

    /// Checks if the specified data can be used to construct a video
    pub fn is_data_valid(data: &Vec<u8>) -> bool {
        return data.len() >= BYTES_BEFORE_FRAMES as usize;
    }

    /// Calculates the width from a video data buffer
    pub fn get_width_from_data(data: &Vec<u8>) -> u32 {
        let mut res: u32 = 0;

        for i in data.iter().take(VIDEO_SIZE_BYTE_LENGTH as usize) {
            res += *i as u32;
        }

        return res;
    }

    /// Calculates the height from a video data buffer
    pub fn get_height_from_data(data: &Vec<u8>) -> u32 {
        let mut res: u32 = 0;

        for i in data.iter().skip(VIDEO_SIZE_BYTE_LENGTH as usize).take(VIDEO_SIZE_BYTE_LENGTH as usize) {
            res += *i as u32;
        }

        return res;
    }

    /// Calculates the amount of frames from a video buffer
    pub fn get_frame_amount_from_data(data: &Vec<u8>) -> usize {
        let w = Video::get_width_from_data(data);
        let h = Video::get_height_from_data(data);
        
        return (data.len() - BYTES_BEFORE_FRAMES as usize) / 3 / (w * h) as usize;
    }
}

// Functions

pub fn dimension_split(dimension: u32) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::new();

    if dimension != 0 {
        let count = (dimension as f64 / 255.0).ceil() as u32;

        for _ in 0..count {
            res.push( (dimension / count) as u8 );
        }

        for i in 0.. dimension % count {
            res[i as usize] += 1;
        }
    }

    while res.len() < VIDEO_SIZE_BYTE_LENGTH as usize {
        res.push(0);
    }

    return res;
} 

pub fn get_coords_at_idx(index: u32, width: u32, height: u32) -> (u32, u32) {
    let x = index % width;
    let y = (index / width) % height;

    return (x, y)
}

pub fn get_idx_at_coords(x: u32, y: u32, width: u32) -> u32 {
    return y * width + x;
}