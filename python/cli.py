import time
import warnings
import sys
import os
import cv2

warnings.simplefilter("ignore")
os.environ['PYGAME_HIDE_SUPPORT_PROMPT'] = "hide"
import pygame

import min_video

RECOMMENDED_WIDTH = 128
RECOMMENDED_HEIGHT = 96

def get_min_video_from_mp4(input_path: str, width: int = -1, height: int = -1) -> min_video.Video:
    cap = cv2.VideoCapture(input_path)

    assert cap.isOpened(), "Failed to open: " + input_path

    w = int(cap.get(3))
    h = int(cap.get(4))

    if width != -1:
        w = width
    
    if height != -1:
        h = height

    vid = min_video.Video(w, h)

    total_frames = int(cap.get(cv2.CAP_PROP_FRAME_COUNT))
    progress_width = 50

    print("Processing frames:")

    start_time = time.time()  # Start time for calculating elapsed time

    for i in range(total_frames):
        ret, frame = cap.read()

        if not ret:
            break

        if width != -1 and height != -1:
            frame = cv2.resize(frame, (width, height))

        m_frame = min_video.Frame(w, h)

        for y in range(h):
            for x in range(w):
                r = frame[y, x, 2]
                g = frame[y, x, 1]
                b = frame[y, x, 0]

                m_frame.set_color(x, y, r, g, b)

        vid.add_frame(m_frame)

        progress = int((i / total_frames) * progress_width)
        percent = int((i / total_frames) * 100)
        if percent == 99:
            percent = 100

        elapsed_time = time.time() - start_time
        estimated_time = (elapsed_time / (i + 1)) * (total_frames - i - 1)

        print('\r', '[' + '#' * progress + '-' * (progress_width - progress) + f'] {percent}% '
              f'| Estimated: {estimated_time:.2f}s', end='')

    print('\n')
    return vid



def show_help():
    print("MinVideo command line tool (Python version)")
    print("Source: https://github.com/Wolfyxon/MinVideo")
    print()
    print("Available options:")
    print("    help                                               : Shows this text")
    print("    play <path>                                        : Plays video in the MinVideo")
    print("    parse <path>                                       : Shows video data")
    print("    convert <input file> [output file] [width] [height]: Converts a MP4 video to the MinVideo format.")
    print()
    print("NOTE: Videos are resized to 128x96 as it's a optimal size. Use -1 -1 to use the original size.")
    
def parse_option():
    if len(sys.argv) < 3:
        print("Video path is required")
        exit(1)

    path = sys.argv[2]

    print("Reading: " + path + "...")

    with open(path, "rb") as file:
        data = file.read()

        w = min_video.Video.get_width_from_data(data)
        h = min_video.Video.get_height_from_data(data)
        frames = min_video.Video.get_frame_amount_from_data(data)

        print("Size: " + str(w) + "x" + str(h))
        print("Frames: " + str(frames))

def convert_option():
    if len(sys.argv) < 3:
        print("At least 1 argument is required: convert <input file> [output file] [width] [height]")
        exit(1)

    w = RECOMMENDED_WIDTH
    h = RECOMMENDED_HEIGHT

    if len(sys.argv) >= 5:
        w = int(sys.argv[4])

    if len(sys.argv) >= 6:
        h = int(sys.argv[5])

    in_path = sys.argv[2]
    out_path = ""

    if len(sys.argv) > 3:
        out_path = sys.argv[3]
    else:
        file, ext = os.path.splitext(in_path)
        out_path = file + ".minv"

    print("Converting standard video: " + in_path)
    print("To MinVideo: " + out_path)
    print("Using size: " + str(w) + "x" + str(h) + "\n")

    tm = time.time()

    vid = get_min_video_from_mp4(in_path, w, h)
    total_frames = len(vid.frames)

    vid.save_file(out_path)

    print("\nDone")
    print(str(w) + "x" + str(h) + " " + str(total_frames) + " frames")
    print("Conversion took " + str(time.time() - tm) + " seconds")

def play_option():
    if len(sys.argv) < 3:
        print("Video path is required")
        exit(1)

    path = sys.argv[2]

    if not os.path.isfile(path):
        print("File not found: " + path)
        exit(-1)

    print("Reading file: " + path + "...")

    tm = time.time()

    data = open(path, "rb").read()

    width = min_video.Video.get_width_from_data(data)
    height = min_video.Video.get_height_from_data(data)
    frames = min_video.Video.get_frame_amount_from_data(data)

    print("File read")
    print("Reading took " + str(time.time() - tm) + " seconds")
    print(str(width) + "x" + str(height) + " " + str( frames ) + " frames")


    pygame.init()
    screen = pygame.display.set_mode((width, height),pygame.HWSURFACE|pygame.DOUBLEBUF|pygame.RESIZABLE)
    pygame.display.set_caption("MinVideo renderer")

    surface = pygame.Surface((width, height))

    while True:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                print("Playback stopped, quitting")
                pygame.quit()
                sys.exit()
        

        def render(frame):
            for y in range(height):
                for x in range(width):
                    rgb = frame.get_color(x, y)
                    surface.set_at((x, y), rgb)
            
            screen.blit(pygame.transform.scale(surface, (screen.get_width(), screen.get_height())), (0, 0))
            pygame.display.flip()

        min_video.Video.foreach_frame(data, render)

if __name__ == "__main__":
    if len(sys.argv) <= 1:
        show_help()
        exit()

    match sys.argv[1]:
        case "help":
            show_help()

        case "convert":
            convert_option()

        case "play":
            play_option()

        case "parse":
            parse_option()

        case _:
            print("Unknown option: " + sys.argv[1])
            print("")
            show_help()
            