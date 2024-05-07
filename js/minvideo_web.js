// JavaScript MinVideo version for web browsers
// Accessed by importing the script, then using the functions and classes in other scripts

const minvideo = {};

minvideo.VIDEO_SIZE_BYTE_LENGTH = 8;
minvideo.VIDEO_MAX_DIMENSION = minvideo.VIDEO_SIZE_BYTE_LENGTH * 255;
minvideo.BYTES_BEFORE_FRAMES = minvideo.VIDEO_SIZE_BYTE_LENGTH * 2;


/**
 * @constructor
 * @param {number} width 
 * @param {number} height 
 * @param {boolean} noDataInit 
 */
minvideo.Frame = function(width, height, data) {
    this.width = width;
    this.height = height;

    if(!data) {
        this.data = [];

        const size = width * height * 3
        let pix = [];

        for(let i = 0; i < size; i++) {
            pix.push(0);

            if(i % 3 == 0) {
                this.data.push(pix);
                pix = [];
            }

        }

    } else {
        this.data = data;
    }
}
minvideo.Frame.prototype = {
    /**
     * @param {number} x 
     * @param {number} y 
     * @param {number} r 
     * @param {number} g 
     * @param {number} b 
     */
    setColor: function(x, y, r, g, b) {
        const begin = minvideo.getIdxAtCoords(x, y, this.width) * 3;

        self.data[begin + 0] = r;
        self.data[begin + 1] = g;
        self.data[begin + 2] = b;
    },

    getColor: function(x, y) {
        const begin = minvideo.getIdxAtCoords(x, y, this.width) * 3;

        return {
            b: self.data[begin + 0],
            g: self.data[begin + 1],
            r: self.data[begin + 2]
        };
    }
}

/**
 * @constructor
 * @param {number} width 
 * @param {number} height 
 * @param {boolean} noDataInit 
 */
minvideo.Video = function(width, height, noDataInit) {
    this.width = width;
    this.height = height;
    this.data = [];

    if(!noDataInit) {
        const size = width * height * 3;
        
        for(let i = 0; i < size; i++) {
            this.data.push(0);
        }
    }
}

/**
 * @param {number[]} data 
 * @returns {minvideo.Video}
 */
minvideo.Video.fromData = function(data) {    
    const w = minvideo.Video.getWidthFromData(data);
    const h = minvideo.Video.getHeightFromData(data);

    const vid = new minvideo.Video(w, h, true);
    vid.data = data;

    return vid
}

/**
 * @param {number[]} data 
 * @returns {number}
 */
minvideo.Video.getWidthFromData = function(data) {
    let res = 0

    for(let i = 0; i < VIDEO_SIZE_BYTE_LENGTH; i++) {
        res += data[i];
    }

    return res
}

/**
 * @param {number[]} data 
 * @returns {number}
 */
minvideo.Video.getHeightFromData = function(data) {
    let res = 0

    for(let i = VIDEO_SIZE_BYTE_LENGTH; i < VIDEO_SIZE_BYTE_LENGTH * 2; i++) {
        res += data[i];
    }
    
    return res
}


/**
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
 * @param {number} x 
 * @param {number} y 
 * @param {number} width 
 * @returns {number}
 */
minvideo.getIdxAtCoords = function(x, y, width) {
    return y * width + x
}
