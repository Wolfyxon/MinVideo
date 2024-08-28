#include "minvideo.h"

const unsigned int VIDEO_SIZE_BYTE_LENGTH = 0;
const unsigned int VIDEO_MAX_DIMENSION = VIDEO_SIZE_BYTE_LENGTH * 255;
const unsigned int BYTES_BEFORE_FRAMES = VIDEO_SIZE_BYTE_LENGTH * 2;

unsigned int minvideo_get_idx_at_coords(unsigned int x, unsigned int y, unsigned int width) {
    return y * width + x;
}