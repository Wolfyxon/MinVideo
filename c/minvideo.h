#ifndef MINVIDEO_H__
#define MINVIDEO_H__

extern const unsigned int VIDEO_SIZE_BYTE_LENGTH;
extern const unsigned int VIDEO_MAX_DIMENSION;
extern const unsigned int BYTES_BEFORE_FRAMES;

typedef struct {
    unsigned int data_len;
    unsigned int *data;
    unsigned int width;
    unsigned int height;
} MinVideoFrame;

typedef struct {
    unsigned int r;
    unsigned int g;
    unsigned int b;
} MinVideoColor;

extern void MinVideoFrame_set_color(MinVideoFrame* frame, unsigned int x, unsigned int y, MinVideoColor color);
extern MinVideoColor MinVideoFrame_get_color(MinVideoFrame* frame, unsigned int x, unsigned int y);

extern unsigned int minvideo_get_idx_at_coords(unsigned int x, unsigned int y, unsigned int width);

#endif