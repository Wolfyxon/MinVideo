import os, sys, time, shutil, glob
import threading
import queue
from pygame.locals import QUIT
os.environ["PYGAME_HIDE_SUPPORT_PROMPT"] = 'hide'
import pygame

# Initialize Pygame
pygame.init()

# Read initial file info
with open("race.minv", 'rb') as outf:
    width = int.from_bytes(outf.read(0x08), byteorder='little')
    outf.seek(0x08)
    height = int.from_bytes(outf.read(0x08), byteorder='little')
    outf.seek(0x10)
    dataClus = outf.read()  # Read all frame data

# Each frame has width*height*3 bytes (RGB)
frame_size = width * height * 3
num_frames = len(dataClus) // frame_size

# Create Pygame window
screen = pygame.display.set_mode((width, height))
pygame.display.set_caption("Frame Viewer")

# Set frame rate to 30 FPS
fps = 30
clock = pygame.time.Clock()

# Function to convert raw RGB data to a Pygame Surface
def rgb_to_surface(rgb_data, width, height):
    surface = pygame.image.fromstring(rgb_data, (width, height), 'RGB')
    return surface

# Queue to hold frames for rendering
frame_queue = queue.Queue(maxsize=1000)  # Limits the number of frames in queue

# Thread to read frames from the file
def frame_reader():
    current_frame = 0
    while current_frame < num_frames:
        start = current_frame * frame_size
        end = start + frame_size
        frame_data = dataClus[start:end]
        
        # Put frame data in the queue
        frame_queue.put(frame_data)

        # Move to the next frame
        current_frame = (current_frame + 1) % num_frames

# Thread to render frames
def frame_renderer():
    running = True
    while running:
        for event in pygame.event.get():
            if event.type == QUIT:
                running = False

        if not frame_queue.empty():
            frame_data = frame_queue.get()
            surface = rgb_to_surface(frame_data, width, height)
            
            # Display frame on the screen
            screen.blit(surface, (0, 0))
            pygame.display.flip()

        # Control frame rate (30 FPS)
        clock.tick(fps)

    pygame.quit()

# Create and start the threads
reader_thread = threading.Thread(target=frame_reader)
renderer_thread = threading.Thread(target=frame_renderer)

reader_thread.start()
renderer_thread.start()

# Wait for both threads to complete
reader_thread.join()
renderer_thread.join()
