#include <iostream>
#include <vector>
#include <numeric>
#include <fstream>
#include <cmath>
#include <cassert>

const int VIDEO_SIZE_BYTE_LENGTH = 8;
const int VIDEO_MAX_DIMENSION = VIDEO_SIZE_BYTE_LENGTH * 255;
const int BYTES_BEFORE_FRAMES = VIDEO_SIZE_BYTE_LENGTH * 2;

extern "C" __declspec(dllexport) std::vector<int> dimension_split(int dimension) {
    std::vector<int> res;

    if (dimension != 0) {
        int count = std::ceil(static_cast<double>(dimension) / 255);
        res = std::vector<int>(count, dimension / count);

        for (int i = 0; i < dimension % count; ++i) {
            res[i] += 1;
        }
    }

    while (res.size() < VIDEO_SIZE_BYTE_LENGTH) {
        res.push_back(0);
    }

    return res;
}

extern "C" __declspec(dllexport) std::pair<int, int> get_coords_at_idx(int frame_index, int width, int height) {
    int x = frame_index % width;
    int y = (frame_index / width) % height;

    return {x, y};
}

class __declspec(dllexport) Frame {
private:
    std::vector<std::vector<int>> pixels;
    int width;
    int height;

public:
    Frame(int width, int height) : width(width), height(height) {
        assert(width <= VIDEO_MAX_DIMENSION && height <= VIDEO_MAX_DIMENSION && "Width/height out of range");
        pixels = std::vector<std::vector<int>>(width * height, std::vector<int>(3, 0));
    }

    int get_area() const {
        return width * height;
    }

    int get_index(int x, int y) const {
        return y * width + x;
    }

    void set_color(int x, int y, int r, int g, int b) {
        assert(x < width && y < height && "Coordinates out of range");
        pixels[get_index(x, y)] = {r, g, b};
    }

    std::vector<int> get_color(int x, int y) const {
        if (x >= width || y >= height) {
            return {0, 0, 0};
        }

        return pixels[get_index(x, y)];
    }

    std::vector<int> get_data() const {
        std::vector<int> res;

        for (const auto& pixel : pixels) {
            res.insert(res.end(), pixel.begin(), pixel.end());
        }

        return res;
    }

    const std::vector<std::vector<int>>& get_rgb_data() const {
        return pixels;
    }
};

class __declspec(dllexport) Video {
private:
    std::vector<Frame> frames;
    int width;
    int height;

public:
    Video(int width, int height) : width(width), height(height) {
        assert(width <= VIDEO_MAX_DIMENSION && height <= VIDEO_MAX_DIMENSION && "Width/height out of range");
    }

    void add_frame(const Frame& frame) {
        frames.push_back(frame);
    }

    int get_area() const {
        return width * height;
    }

    std::vector<int> get_data() const {
        std::vector<int> data;

        auto width_splits = dimension_split(width);
        auto height_splits = dimension_split(height);

        data.insert(data.end(), width_splits.begin(), width_splits.end());
        data.insert(data.end(), height_splits.begin(), height_splits.end());

        for (const auto& frame : frames) {
            auto frame_data = frame.get_data();
            data.insert(data.end(), frame_data.begin(), frame_data.end());
        }

        return data;
    }

    std::vector<unsigned char> get_bytes() const {
        auto data = get_data();
        return std::vector<unsigned char>(data.begin(), data.end());
    }

    void save_file(const std::string& path) const {
        std::ofstream file(path, std::ios::binary);
        if (file.is_open()) {
            auto bytes = get_bytes();
            file.write(reinterpret_cast<const char*>(bytes.data()), bytes.size());
            file.close();
        } else {
            std::cerr << "Error opening file for writing: " << path << std::endl;
        }
    }

    static int get_width_from_data(const std::vector<int>& data) {
        return std::accumulate(data.begin(), data.begin() + VIDEO_SIZE_BYTE_LENGTH, 0);
    }

    static int get_height_from_data(const std::vector<int>& data) {
        return std::accumulate(data.begin() + VIDEO_SIZE_BYTE_LENGTH, data.begin() + VIDEO_SIZE_BYTE_LENGTH * 2, 0);
    }

    static int get_frame_amount_from_data(const std::vector<int>& data) {
        int w = get_width_from_data(data);
        int h = get_height_from_data(data);
        int pixel_amt = w * h * 3;

        return (data.size() - BYTES_BEFORE_FRAMES) / pixel_amt;
    }

    static Video from_data(const std::vector<int>& data) {
        int w = get_width_from_data(data);
        int h = get_height_from_data(data);
        Video vid(w, h);
        int frames = get_frame_amount_from_data(data);

        for (int frame_i = 0; frame_i < frames; ++frame_i) {
            Frame frame(w, h);

            int color_start_index = BYTES_BEFORE_FRAMES + frame_i * w * h * 3;
            std::vector<int> colors(data.begin() + color_start_index, data.begin() + color_start_index + w * h * 3);

            for (int i = 0; i < w * h; ++i) {
                auto [x, y] = get_coords_at_idx(i, w, h);
                int idx = i * 3;

                int b = colors[idx];
                int g = colors[idx + 1];
                int r = colors[idx + 2];

                frame.set_color(x, y, r, g, b);
            }

            vid.add_frame(frame);
        }

        return vid;
    }
};
