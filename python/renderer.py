import os, sys, time, shutil, glob
from tkinter import filedialog
import threading
import queue
from pygame.locals import QUIT
os.environ["PYGAME_HIDE_SUPPORT_PROMPT"] = 'hide'
import pygame
pygame.init()

video_file = filedialog.askopenfilename(
    defaultextension=".minv",
    filetypes=[("MinVideo files", "*.minv;*.miv;*.minvideo;*.mvid")],
    initialdir=os.getcwd(),
    title="Load MinVideo File"
)

with open(video_file, 'rb') as outf:
    width = int.from_bytes(outf.read(0x06), byteorder='little')
    outf.seek(0x06)
    height = int.from_bytes(outf.read(0x06), byteorder='little')
    outf.seek(0x0C)
    fps = int.from_bytes(outf.read(0x04), byteorder='little')
    outf.seek(0x10)
    dataClus = outf.read()

frame_size = width * height * 3
num_frames = len(dataClus) // frame_size
screen = pygame.display.set_mode((width, height))
pygame.display.set_caption("MinVideo Player")

clock = pygame.time.Clock()
def rgb_to_surface(rgb_data, width, height):
    surface = pygame.image.fromstring(rgb_data, (width, height), 'RGB')
    return surface

if fps >= 40:
    frame_queue = queue.Queue(maxsize=20)
elif fps >= 20 and fps < 40:
    frame_queue = queue.Queue(maxsize=10)
else:
    frame_queue = queue.Queue(maxsize=5)

def frame_reader():
    current_frame = 0
    while current_frame < num_frames:
        start = current_frame * frame_size
        end = start + frame_size
        frame_data = dataClus[start:end]
        frame_queue.put(frame_data)
        current_frame = (current_frame + 1) % num_frames

def frame_renderer():
    running = True
    while running:
        for event in pygame.event.get():
            if event.type == QUIT:
                running = False

        if not frame_queue.empty():
            frame_data = frame_queue.get()
            surface = rgb_to_surface(frame_data, width, height)
            
            screen.blit(surface, (0, 0))
            pygame.display.flip()

        clock.tick(fps)

    pygame.quit()

reader_thread = threading.Thread(target=frame_reader)
renderer_thread = threading.Thread(target=frame_renderer)

reader_thread.start()
renderer_thread.start()
reader_thread.join()
renderer_thread.join()