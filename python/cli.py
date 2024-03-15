import min_video
import cv2
import sys
import os

os.environ['PYGAME_HIDE_SUPPORT_PROMPT'] = "hide"

import pygame

RECOMMENDED_WIDTH = 128
RECOMMENDED_HEIGHT = 96

def get_min_video_from_mp4(input_path: str, width: int = -1, height: int = -1) -> min_video.Video:
    cap = cv2.VideoCapture(input_path)

    assert cap.isOpened(), "Failed to open: " + input_path

    w  = int(cap.get(3))
    h = int(cap.get(4))

    if width != -1:
        w = width
    
    if height != -1:
        h = height

    vid = min_video.Video(w, h)

    while True:
        ret, frame = cap.read()

        if not ret:
            break

        if width != -1 and height != -1:
            frame = cv2.resize(frame, (width, height))

        m_frame = min_video.Frame(w, h)

        for y in range(h):
            for x in range(w):
                r = frame[y, x, 0]
                g = frame[y, x, 1]
                b = frame[y, x, 2]

                m_frame.set_color(x, y, r, g, b)

        vid.add_frame(m_frame)

    return vid

def show_help():
    print("MinVideo command line tool")
    print("Source: https://github.com/Wolfyxon/MinVideo")
    print()
    print("Available options:")
    print("    help                                               : Shows this text")
    print("    play <path>                                        : Plays video in the MinVideo")
    print("    convert <input file> <output file> [width] [height]: Converts a MP4 video to the MinVideo format.")
    print()
    print("NOTE: Videos are resized to 128x96 as it's a optimal size. Use -1 -1 to use the original size.")
    

if __name__ == "__main__":
    if len(sys.argv) <= 1:
        show_help()
        exit()

    match sys.argv[1]:
        case "help":
            show_help()

        case "convert":
            if len(sys.argv) < 4:
                print("At least 3 arguments are required: convert <input file> <output file> [width] [height]")
                exit(1)

            w = RECOMMENDED_WIDTH
            h = RECOMMENDED_HEIGHT

            if len(sys.argv) >= 5:
                w = int(sys.argv[4])
            
            if len(sys.argv) >= 6:
                h = int(sys.argv[5])

            in_path = sys.argv[2]
            out_path = sys.argv[3]

            print("Converting standart video: " + in_path)
            print("To MinVideo: " + out_path)
            print("Using size: " + str(w) + "x" + str(h))

            vid = get_min_video_from_mp4( sys.argv[2], w, h )
            vid.save_file( sys.argv[3] )

            print("Done")

        case "play":
            if len(sys.argv) < 3:
                print("Video path is required")
                exit(1)

            

        case _:
            print("Unknown option: " + sys.argv[1])
            print("")
            show_help()
            