#include "minvideo.h"

int minvideo_get_idx_at_coords(int x, int y, int width) {
    return y * width + x;
}