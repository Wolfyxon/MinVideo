// JavaScript MinVideo version for web browsers
// Accessed by importing the script, then using the functions and classes in other scripts

const minvideo = {};

minvideo.VIDEO_SIZE_BYTE_LENGTH = 8;
minvideo.VIDEO_MAX_DIMENSION = minvideo.VIDEO_SIZE_BYTE_LENGTH * 255;
minvideo.BYTES_BEFORE_FRAMES = minvideo.VIDEO_SIZE_BYTE_LENGTH * 2;

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
