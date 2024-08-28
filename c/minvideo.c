#include "minvideo.h"

const unsigned int VIDEO_SIZE_BYTE_LENGTH = 0;
const unsigned int VIDEO_MAX_DIMENSION = VIDEO_SIZE_BYTE_LENGTH * 255;
const unsigned int BYTES_BEFORE_FRAMES = VIDEO_SIZE_BYTE_LENGTH * 2;

MinVideoColor MinVideoFrame_get_color(MinVideoFrame* frame, unsigned int x, unsigned int y) {
    unsigned int begin = minvideo_get_idx_at_coords(x, y, frame->width) * 3;

    MinVideoColor color;

    color.r = frame->data[begin + 0];
    color.g = frame->data[begin + 1];
    color.b = frame->data[begin + 2];
    
    return color;
 }

void MinVideoFrame_set_color(MinVideoFrame* frame, unsigned int x, unsigned int y, MinVideoColor color) {
    unsigned int begin = minvideo_get_idx_at_coords(x, y, frame->width) * 3;
    
    frame->data[begin + 0] = color.r;
    frame->data[begin + 1] = color.g;
    frame->data[begin + 2] = color.b;
}

unsigned int minvideo_get_idx_at_coords(unsigned int x, unsigned int y, unsigned int width) {
    return y * width + x;
}