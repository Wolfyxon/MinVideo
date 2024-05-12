// JavaScript MinVideo version for web browsers
// Accessed by importing the script, then using the functions and classes in other scripts

const minvideo = {};

minvideo.VIDEO_SIZE_BYTE_LENGTH = 8;
minvideo.VIDEO_MAX_DIMENSION = minvideo.VIDEO_SIZE_BYTE_LENGTH * 255;
minvideo.BYTES_BEFORE_FRAMES = minvideo.VIDEO_SIZE_BYTE_LENGTH * 2;


////////////////// Frame //////////////////
/**
 * Video frame image class
 * @constructor
 * @param {number} width 
 * @param {number} height 
 * @param {boolean} noDataInit 
 */
minvideo.Frame = function(width, height, data) {
    if(width > minvideo.VIDEO_MAX_DIMENSION) throw "Maximum width exceeded: "+ VIDEO_MAX_DIMENSION;
    if(height > minvideo.VIDEO_MAX_DIMENSION) throw "Maximum height exceeded: "+ VIDEO_MAX_DIMENSION;
    
    this.width = width;
    this.height = height;

    if(!data) {
        this.data = [];
        const size = width * height * 3;

        for(let i = 0; i < size; i++) {
            this.data.push(0);
        }

    } else {
        this.data = data;
    }
}

/// Dynamic methods ///

minvideo.Frame.prototype = {
    /**
     * Sets a color at a position in the frame
     * @param {number} x 
     * @param {number} y 
     * @param {number} r 
     * @param {number} g 
     * @param {number} b 
     */
    setColor: function(x, y, r, g, b) {
        const begin = minvideo.getIdxAtCoords(x, y, this.width) * 3;

        this.data[begin + 0] = r;
        this.data[begin + 1] = g;
        this.data[begin + 2] = b;
    },

    /**
     * Returns a RGB color at the given position
     * @param {number} x 
     * @param {number} y 
     * @returns {{r: number, g: number, b: number}}
     */
    getColor: function(x, y) {
        const begin = minvideo.getIdxAtCoords(x, y, this.width) * 3;

        return {
            r: this.data[begin + 0],
            g: this.data[begin + 1],
            b: this.data[begin + 2]
        };
    }
}

////////////////// Video //////////////////

/**
 * Video class
 * @constructor
 * @param {number} width 
 * @param {number} height 
 * @param {boolean} noDataInit 
 */
minvideo.Video = function(width, height) {
    if(width > minvideo.VIDEO_MAX_DIMENSION) throw "Maximum width exceeded: "+ VIDEO_MAX_DIMENSION;
    if(height > minvideo.VIDEO_MAX_DIMENSION) throw "Maximum height exceeded: "+ VIDEO_MAX_DIMENSION;
    
    this.width = width;
    this.height = height;
    this.data = [];
}

/// Dynamic methods ///

minvideo.Video.prototype = {
    /**
     * Adds a frame to the video
     * @param {minvideo.Frame} frame 
     */
    addFrame: function(frame) {
        if( frame.width !== this.width ) throw "Frame width does not match the video width";
        if( frame.height !== this.height ) throw "Frame height does not match the video height";
        
        if(frame.data.length !== this.width * this.height * 3) throw "Invalid frame buffer. Data size must be " + this.width * this.height * 3;

        for(let i = 0; i < frame.data.length; i++) {
            this.data.push(frame.data[i]);
        }
    },

    /**
     * Gets the nth frame of the video
     * @param {number} index
     * @returns {minvideo.Frame} 
     */
    getFrame: function(index) {
        const begin = minvideo.BYTES_BEFORE_FRAMES + (this.width * this.height * 3) * index;
        const end = begin + (this.width * this.height * 3);

        return new minvideo.Frame( this.width, this.height, this.data.slice(begin, end) );
    },

    /**
     * Gets the amount of frames in the video
     * @returns {number}
     */
    getFrameAmount: function() {
        return minvideo.Video.getFrameAmountFromData(this.data);
    }
}

/// Static methods ///

/**
 * Creates a video from a buffer
 * @param {number[]} data 
 * @returns {minvideo.Video}
 */
minvideo.Video.fromData = function(data) {    
    const w = minvideo.Video.getWidthFromData(data);
    const h = minvideo.Video.getHeightFromData(data);

    const vid = new minvideo.Video(w, h);
    vid.data = data;

    return vid
}

/**
 * Calculates video width from a buffer
 * @param {number[]} data 
 * @returns {number}
 */
minvideo.Video.getWidthFromData = function(data) {
    let res = 0

    for(let i = 0; i < minvideo.VIDEO_SIZE_BYTE_LENGTH; i++) {
        res += data[i];
    }

    return res
}

/**
 * Calculates video height from a buffer
 * @param {number[]} data 
 * @returns {number}
 */
minvideo.Video.getHeightFromData = function(data) {
    let res = 0

    for(let i = minvideo.VIDEO_SIZE_BYTE_LENGTH; i < minvideo.VIDEO_SIZE_BYTE_LENGTH * 2; i++) {
        res += data[i];
    }
    
    return res
}

/**
 * Calculates the amount of frames from a buffer
 * @param {number[]} data 
 * @returns {number}
 */
minvideo.Video.getFrameAmountFromData = function(data) {
    const w = minvideo.Video.getWidthFromData(data);
    const h = minvideo.Video.getHeightFromData(data);

    return (data.length - minvideo.BYTES_BEFORE_FRAMES) / 3 / (w * h);
}


////////////////// Other functions //////////////////

/**
 * Splits a size dimension to store it in a buffer
 * @param {number} dimension
 * @returns {number[]} 
 */
minvideo.dimensionSplit = function(dimension){
    const res = [];

    if(dimension !== 0) {
        const count = Math.ceil(dimension / 255);

        for(let i = 0; i < count; i++ ) {
            res.push(dimension / count);
        }

        for(let i = 0; i < dimension % count; i++) {
            res[i] += 1;
        }
    }

    while(res.length < minvideo.VIDEO_SIZE_BYTE_LENGTH) {
        res.push(0);
    }

    return res;
}

/**
 * Calculates a position at the specified pixel index
 * @param {number} index 
 * @param {number} width 
 * @param {number} height 
 * @returns {{x: number, y: number}}
 */
minvideo.getCoordsAtIdx = function(index, width, height) {
    return {
        x: index % width,
        y: (index / width) % height
    }
}

/**
 * Calculates a pixel index at the specified position
 * @param {number} x 
 * @param {number} y 
 * @param {number} width 
 * @returns {number}
 */
minvideo.getIdxAtCoords = function(x, y, width) {
    return y * width + x
}
