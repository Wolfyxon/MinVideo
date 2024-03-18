# MinVideo
A simple video format easy to adapt in limited environments.

## How it works
`16` bytes are reserved for the size. Each dimension is `8` bytes so the maximum video size is `2040x2040`. However this can be easily changed by modifying the code, but this will make the decoder not being able to properly parse videos made in the regular format.

After that there are the frames. Each frames consists of `width * height * 3` bytes which hold the colors. 

Each color is `3` bytes: red, green, blue. Colors can hold numbers up to 255, example `[255, 0, 0]` is red.

I hope this will help you develop your own decoder or encoder following this format, for your project.

## Why?
It's a very raw format without any compression or complicated algorithms which makes it usable in example electronics or games. You can make a circuit for rendering videos then encode the bytes to a ROM which the circuit will use.

You can also use it in game engines that don't support video playback. You would need to read a file's bytes or convert a string to bytes if file access is not possible. You can also use the data numbers, but it will negatively impact the file size.

## Related projects
[MinVideo Tools](https://github.com/Cracko298/MinVideo-Tools) by Cracko298
