# Audio noise WAV generator in Rust
This is a fast console program written in Rust.

## Description
`Audio_noise_gen` is a console program to generate a noise WAV file with n seconds. The noise can be white noise, pink noise or brown noise. This program was developed and tested on Linux. <br>
See the output example files: <br>
- **brown_noise.wav** - Sounds like a nice low pitch **Waterfall sound**. 
- **pink_noise.wav** - Sounds like a **heavy rain day**.
- **white_noise.wav** - Sounds like **static on an old CRT TV** without the antenna plugged in. 


## References
This reference is in Javascript for the Web Audio API, but it as a good description of the algorithms to generate each of the different kind of noise types. I ported them to Rust and developed the rest of my program. <br>
[How to Generate Noise with the Web Audio API](https://noisehack.com/generate-noise-web-audio-api/)

# How to

## To compile the final program inside cargo
```
cargo build --release
```

## To execute the program directly do
```
Usage: "audio_noise_gen [white|pink|brown] duration_in_sec filename.wav"

ex: audio_noise_gen brown 10 brown_noise.wav
```

## To execute the program inside cargo
```
cargo run --release brown 10 brown_noise.wav
```
 
## To generate the docs inside cargo
```
cargo doc
cargo doc --open
```

## License
Apache-2

## Have fun!
Best regards, <br>
Joao Nuno Carvalho <br>