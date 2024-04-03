import math

VIDEO_SIZE_BYTE_LENGTH = 8
VIDEO_MAX_DIMENSION = VIDEO_SIZE_BYTE_LENGTH * 255
BYTES_BEFORE_FRAMES = VIDEO_SIZE_BYTE_LENGTH * 2

def dimension_split(dimension: int) -> list[int]:
    res = []

    if dimension != 0:
        count = math.ceil(dimension / 255)
        res = [dimension // count] * count

        for i in range(dimension % count):
            res[i] += 1

    while len(res) < VIDEO_SIZE_BYTE_LENGTH:
        res.append(0)

    return res

def get_coords_at_idx(frame_index: int, width: int, height: int) -> tuple[int, int]:
    x = frame_index % width
    y = (frame_index // width) % height

    return x, y


class Frame:
    def __init__(self, width, height) -> None:
        assert width <= VIDEO_MAX_DIMENSION, "Width cannot be greater than " + str(VIDEO_MAX_DIMENSION)
        assert height <= VIDEO_MAX_DIMENSION, "Height cannot be greater than " + str(VIDEO_MAX_DIMENSION)
        
        self.pixels: list[ list[int] ] = []
        self.width = width
        self.height = height

        self.pixels = [None for _ in range(width * height)]

    def get_index(self, x: int, y: int) -> int:
        return y * self.width + x

    def set_color(self, x: int, y: int, r: int, g: int, b: int):
        idx = self.get_index(x, y)
        
        assert x <= self.width, "X out of range"
        assert y <= self.height, "Y out of range"

        px = [r, g, b]
        self.pixels[idx] = px

    def get_color(self, x: int, y: int) -> list[int]:
        idx = self.get_index(x, y)

        if idx >= len(self.pixels):
            return

        col = self.pixels[idx]
        
        if not col:
            return [0, 0, 0]

        return col

    def get_data(self) -> list[int]:
        res = []

        for i in self.pixels:
            for c in i:
                res.append(c)

        return res

class Video:
    def __init__(self, width: int, height: int) -> None:
        assert width <= VIDEO_MAX_DIMENSION, "Width cannot be greater than " + str(VIDEO_MAX_DIMENSION)
        assert height <= VIDEO_MAX_DIMENSION, "Height cannot be greater than " + str(VIDEO_MAX_DIMENSION)

        self.frames: list[Frame] = []
        self.width = width
        self.height = height

    def add_frame(self, frame: Frame):
        self.frames.append(frame)

    def get_data(self) -> list[int]:
        data = []

        data.extend(dimension_split(self.width))
        data.extend(dimension_split(self.height))

        for frame in self.frames:
            for color in frame.get_data():
                data.append(color)

        return data

    def get_bytes(self) -> bytes:
        return bytes(self.get_data())

    def save_file(self, path: str):
        with open(path, "wb") as file:
            file.write(self.get_bytes())

    @staticmethod
    def get_width_from_data(data: list[int]) -> int:
        return sum(data[:VIDEO_SIZE_BYTE_LENGTH])

    @staticmethod
    def get_height_from_data(data: list[int]) -> int:
        return sum(data[VIDEO_SIZE_BYTE_LENGTH:VIDEO_SIZE_BYTE_LENGTH * 2])

    @staticmethod
    def get_frame_amount_from_data(data: list[int]) -> int:
        w = Video.get_width_from_data(data)
        h = Video.get_height_from_data(data)

        return (len(data) - VIDEO_SIZE_BYTE_LENGTH * 2) // 3 // (w * h)

    @staticmethod
    def from_data(data: list[int]):
        data_len = len(data)
        w = Video.get_width_from_data(data)
        h = Video.get_height_from_data(data)
        pixel_amt = w * h * 3
        vid = Video(w, h)
        frames = (data_len - BYTES_BEFORE_FRAMES) // pixel_amt

        for frame_i in range(frames):
            frame = Frame(w, h)

            color_start_index = BYTES_BEFORE_FRAMES + frame_i * pixel_amt
            colors = data[color_start_index:color_start_index + pixel_amt]

            for i in range(pixel_amt // 3):
                x, y = get_coords_at_idx(i, w, h)

                b = colors[i * 3]
                g = colors[i * 3 + 1]
                r = colors[i * 3 + 2]

                frame.set_color(x, y, r, g, b)

            vid.add_frame(frame)

        return vid
    
    @staticmethod
    def foreach_frame(data: list[int], callback):
        data_len = len(data)
        w = Video.get_width_from_data(data)
        h = Video.get_height_from_data(data)
        pixel_amt = w * h * 3

        frames = (data_len - BYTES_BEFORE_FRAMES) // pixel_amt

        for frame_i in range(frames):
            frame = Frame(w, h)

            color_start_index = BYTES_BEFORE_FRAMES + frame_i * pixel_amt
            colors = data[color_start_index:color_start_index + pixel_amt]

            for i in range(pixel_amt // 3):
                x, y = get_coords_at_idx(i, w, h)

                b = colors[i * 3]
                g = colors[i * 3 + 1]
                r = colors[i * 3 + 2]

                frame.set_color(x, y, r, g, b)

            callback(frame)

    @staticmethod
    def from_file(path: str):
        with open(path, "rb") as file:
            return Video.from_data(file.read())


if __name__ == "__main__":
    print("This script is to be used a library! Refer to 'cli.py'")
    print("Use `import min_video` to access its classes and functions")