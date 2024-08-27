#ifndef MINVIDEO_H__
#define MINVIDEO_H__

struct MinVideoFrame {
    int data_len;
    int *data;
    int width;
    int height;
};

extern int minvideo_get_idx_at_coords(int x, int y, int width);

#endif