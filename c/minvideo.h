#ifndef MINVIDEO_H__
#define MINVIDEO_H__

struct MinVideoFrame {
    unsigned int data_len;
    unsigned int *data;
    unsigned int width;
    unsigned int height;
};

struct MinVideoColor {
    unsigned int r;
    unsigned int g;
    unsigned int b;
};

extern unsigned int minvideo_get_idx_at_coords(unsigned int x, unsigned int y, unsigned int width);

#endif