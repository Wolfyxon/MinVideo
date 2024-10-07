import cv2, os, threading
from PIL import Image
from tkinter import filedialog
from concurrent.futures import ThreadPoolExecutor
from tqdm import tqdm

def save_frame(frame, count, output_folder):
    pil_image = Image.fromarray(cv2.cvtColor(frame, cv2.COLOR_BGR2RGB))
    frame_filename = f"{output_folder}\\frame_{count}.png"
    pil_image.save(frame_filename)

def extract_frames(video_path, output_folder, every_n_frame=1):
    video = cv2.VideoCapture(video_path)
    total_frames = int(video.get(cv2.CAP_PROP_FRAME_COUNT))
    success, frame = video.read()
    count = 0
    frame_count = 0
    if not os.path.exists(output_folder):
        os.makedirs(output_folder)
    
    with ThreadPoolExecutor() as executor:
        futures = []
        with tqdm(total=total_frames // every_n_frame, desc="Extracting Frames") as pbar:
            while success:
                if frame_count % every_n_frame == 0:
                    futures.append(executor.submit(save_frame, frame, count, output_folder))
                    count += 1
                    pbar.update(1)
            
                success, frame = video.read()
                frame_count += 1
        
        for future in futures:
            future.result()

    video.release()

def getFrameList(outdir):
    fileList0 = os.listdir(outdir)
    list0 = []
    for file in fileList0:
        if "frame_" in file and ".png" in file:
            list0.append(file)
        
    list0.sort(key=lambda x: int(x.split('_')[1].split('.')[0]))
    return list0

def process_single_image(image_path, output_file):
    try:
        with Image.open(f".\\extracted_frames\\{image_path}") as img:
            img = img.convert('RGB')
            width, height = img.size
            for y in range(height):
                for x in range(width):
                    r, g, b = img.getpixel((x, y))
                    output_file.write(bytes([r, g, b]))
    except Exception as e:
        print(f"Error processing {image_path}: {e}")

# Function to process images with threading and progress bar
def process_images(image_files, output_file):
    with open(output_file, 'ab+') as out:
        with ThreadPoolExecutor(max_workers=8) as executor:
            # Initialize a thread lock for writing to the file
            lock = threading.Lock()

            # Function that will handle threading and writing
            def threaded_process(image_path):
                # Process each image and write output in a thread-safe way
                with lock:
                    process_single_image(image_path, out)

            # Create a progress bar
            with tqdm(total=len(image_files), desc="Processing Images") as progress_bar:
                # Submit tasks to the ThreadPoolExecutor
                futures = [executor.submit(threaded_process, img) for img in image_files]
                
                # Wait for each thread to complete and update the progress bar
                for _ in futures:
                    _.result()  # To catch any exception in threads
                    progress_bar.update(1)

def getVideoDemensions(vf):
    video = cv2.VideoCapture(vf)
    x = video.get(cv2.CAP_PROP_FRAME_WIDTH)
    y = video.get(cv2.CAP_PROP_FRAME_HEIGHT)
    fps = video.get(cv2.CAP_PROP_FPS)
    video.release()
    return x, y, fps

video_file = filedialog.askopenfilename(
    defaultextension=".mp4",
    filetypes=[("Video files", "*.mp4;*.avi;*.mov;*.wmv")],
    initialdir=os.getcwd(),
    title="Load Video File"
)

minvFile = f"{video_file.replace('.mp4', '.minv')}"
number1, number2, framespersecond = getVideoDemensions(video_file)

with open(minvFile, 'wb') as f:
    byte1 = int(number1).to_bytes(6, 'little')
    f.write(byte1)
    byte2 = int(number2).to_bytes(6, 'little')
    f.write(byte2)
    print(framespersecond)
    byte3 = int(framespersecond).to_bytes(4, 'little')
    f.write(byte3)


output_directory = ".\\extracted_frames"
extract_frames(video_file, output_directory, every_n_frame=1)
list0 = getFrameList(output_directory)
process_images(list0, f"{minvFile}")
