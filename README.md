# MinVideo
A simple video format easy to adapt in limited environments.

## How it works
`16` bytes are reserved for the size. Each dimension is `8` bytes so the maximum video size is `2040x2040`. However this can be easily changed by modifying the code, but this will make the decoder not being able to properly parse videos made in the regular format.

After that there are the frames. Each frames consists of `width * height` bytes which hold the colors. 

Each color is `3` bytes: red, green, blue. Colors can hold numbers up to 255, example `[255, 0, 0]` is red.

I hope this will help you develop your own decoder or encoder following this format, for your project.

## Why?
It's a very raw format without any compression or complicated algorithms which makes it usable in example electronics emulators or games. You can make a circuit for rendering videos then encode the bytes to a ROM which the circuit will use.

You can also use it in game engines that don't support video playback. You would need to read a file's bytes or convert a string to bytes if file access is not possible. You can also use the data numbers, but it will negatively impact the file size.

# MinImage
- Simple, uncompressed images, no algorithm with pure `BGR/RGB` formatting.
- Generated from a MinVideo file.

```
Header Data is a total of 0x20 (32) Bytes at the beginning of the File.
Bytes 0x00 - 0x03 is the Name of The Header (mimg).
Bytes 0x04 - 0x05 Defines the Start of the Header Information.
Bytes 0x06 - 0x07 is the Mode which the file should be read.
- 0x01 is Raw BGR
- 0x02 is Raw RGB
- 0x03 is Raw ETC2_BGR
- 0x04 is Raw ETC2_RGB
Bytes 0x08 - 0x0B is the Width.
Bytes 0x0C - 0x0F is the Height.
Bytes 0x10 - 0x13 is the Width Checksum.
Bytes 0x14 - 0x17 is the Height Checksum.
Bytes 0x18 - 0x20 is the Defining the End of the Header Data.
```
