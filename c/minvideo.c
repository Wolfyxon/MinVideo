#include "minvideo.h"

unsigned int minvideo_get_idx_at_coords(unsigned int x, unsigned int y, unsigned int width) {
    return y * width + x;
}