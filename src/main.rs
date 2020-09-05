//! # Audio_noise_gen
//!
//! `Audio_noise_gen` is a console program to generate a noise WAV file with n seconds. The noise can be white noise, pink noise or brown noise.
//! 
//! ## Info
//! Author:  Joao Nuno Carvalho <br>
//! Date:    2020.09.05         <br>
//! License: Apache-2           <br>
//! 
//! ## References
//! [How to Generate Noise with the Web Audio API](https://noisehack.com/generate-noise-web-audio-api/)
//!
//! ## To compile the final program inside cargo
//! cargo build --release
//! 
//! ## To execute the program directly do
//! Usage: "audio_noise_gen [white|pink|brown] duration_in_sec filename.wav" <br>
//! <br>
//! ex: audio_noise_gen brown 10 brown_noise.wav <br>
//! 
//! ## To execute the program inside cargo
//! cargo run --release brown 10 brown_noise.wav
//! 
//! ## To generate the docs inside cargo
//! cargo doc <br>
//! cargo doc --open <br>

use std::env;
use std::process;
use rand::Rng;
use rand::rngs::ThreadRng;
use hound;

/// Size of the buffer that will be generated each time.
const BUF_SIZE: usize = 4096; 
/// Usage: "audio_noise_gen [white|pink|brown] duration_in_sec filename.wav"
static USAGE: &str = "   Usage: \"audio_noise_gen [white|pink|brown] duration_in_sec filename.wav\" .";

fn main() {
    println!("Audio noise generator...");
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(& args);
    let mut noise_buf = NoiseBuffer::new(BUF_SIZE);
    write_wav_to_file(&cfg, & mut noise_buf);
    println!("...ended generating WAV noise file.");
}

/// Configuration structure to parse the command line options.  
struct Config {
    noise_type: NoiseType,
    duration_sec: u32,
    filename: String,
}

impl Config {
    /// Constructor - Is were the parsing is made.
    /// It exists if an error occurs.
    fn new(args: &[String]) -> Config {
        if args.len() < 4 {
            println!(" Invalid insufficient parameters...");
            println!("{}", USAGE);
            process::exit(1)
        }
        let noise_type = match args[1].to_lowercase().as_str() {
            "white" => NoiseType::White,
            "pink"  => NoiseType::Pink,
            "brown" => NoiseType::Brown,
            _       => {
                println!(" Invalid noise type...");
                println!("{}", USAGE);
                process::exit(1)
            },
        };
        let duration_sec = match args[2].parse::<u32>(){
            Ok(n)  => n,
            Err(_e) => {
                println!(" Invalid duration in seconds...");
                println!("{}", USAGE);
                process::exit(1)
            } 
        };
        let filename = args[3].clone();
        Config { noise_type, duration_sec, filename }
    }
}

/// Enum with the different noise types available.
enum NoiseType {
    White,
    Pink,
    Brown,
}

/// Function that writes to file the WAV and that generates is small
/// buffer chucks the data to be saved in the file. 
/// The file can be of any size because it will not use almost any memory. 
/// Although the program is really fast.
fn write_wav_to_file(config: &Config, noise_buf: & mut NoiseBuffer) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(&config.filename, spec).unwrap_or_else(|err| {
        println!(" Error opening file for writing...\n {}", err);
        println!("{}", USAGE);
        process::exit(1);
    });
    
    let noise_buf_num = (spec.sample_rate * config.duration_sec as u32) 
                        / BUF_SIZE as u32; 

    for _ in 0..noise_buf_num {
        match config.noise_type {
            NoiseType::White => noise_buf.fill_with_white_noise(),
            NoiseType::Pink  => noise_buf.fill_with_pink_noise(),
            NoiseType::Brown => noise_buf.fill_with_brown_noise(),
        }
        for sample in &noise_buf.buf {
            let amplitude = i16::MAX as f32;
            writer.write_sample((sample * amplitude) as i16).unwrap_or_else(|err| {
                println!(" Error writing to file ...\n {}", err);
                println!("{}", USAGE);
                process::exit(1);
            });
        }
    }
    writer.finalize().unwrap_or_else(|err| {
        println!(" Error closing written file ...\n {}", err);
        println!("{}", USAGE);
        process::exit(1);
    });
}

/// The Buffer and the methods that fill the reusable
/// buffer with the different types of noise.
#[derive(Debug)]
struct NoiseBuffer {
    buf: Vec<f32>,
    rng: ThreadRng,
}

impl NoiseBuffer {
    /// Constructor
    fn new(buf_size: usize) -> Self {
        NoiseBuffer {
            buf: vec![0.0; buf_size],
            rng: rand::thread_rng(),
        }
    }

    /// Fills the buffer with white noise, when the buffer is 
    /// used a new buffer should be filled so that the pattern
    /// is not repeated. The process is really fast. 
    fn fill_with_white_noise(& mut self) {
        for i in 0..BUF_SIZE {
            // Generates a float between 0 and 1,
            // then scale it from 0 to 2,
            // then shift between -1 and 1.
            self.buf[i as usize] = self.rng.gen::<f32>() * 2.0 - 1.0; 
        }
    } 
    
    /// Fills the buffer with pink noise, when the buffer is 
    /// used a new buffer should be filled so that the pattern
    /// is not repeated. The process is really fast.
    fn fill_with_pink_noise(& mut self){
        // Array from 0 to 6 initialized to zeros.
        let mut b: [f32; 7] = [0.0; 7];
        for i in 0..BUF_SIZE {
            let white = self.rng.gen::<f32>() * 2.0 - 1.0;
            b[0] = 0.99886 * b[0] + white * 0.0555179;
            b[1] = 0.99332 * b[1] + white * 0.0750759;
            b[2] = 0.96900 * b[2] + white * 0.1538520;
            b[3] = 0.86650 * b[3] + white * 0.3104856;
            b[4] = 0.55000 * b[4] + white * 0.5329522;
            b[5] = -0.7616 * b[5] - white * 0.0168980;
            self.buf[i as usize] = b[0] + b[1] + b[2] + b[3] + b[4] + b[5] + b[6] + white * 0.5362;
            self.buf[i as usize] *= 0.11; // (roughly) compensate for gain
            b[6] = white * 0.115926;
        }
    }

    /// Fills the buffer with brown noise, when the buffer is 
    /// used a new buffer should be filled so that the pattern
    /// is not repeated. The process is really fast.
    /// This one will make remind you of a waterfall.
    fn fill_with_brown_noise(& mut self) {
        let mut last_out: f32 = 0.0;
        for i in 0..BUF_SIZE {
            let white = self.rng.gen::<f32>() * 2.0 - 1.0;
            self.buf[i as usize] = (last_out + (0.02 * white)) / 1.02;
            last_out = self.buf[i as usize];
            self.buf[i as usize] *= 3.5; // (roughly) compensate for gain
        }
    }

}

