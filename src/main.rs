mod dsp;
mod math;

use rodio::{Decoder, OutputStream, Sink, Source};
use std::f32::consts::PI;
use std::fs::File;
use std::io::BufReader;
use std::ops::Rem;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct AnyWave {
    freq: f32,
    num_sample: usize,
    sampling_rate: u32,
    master: f32,
    instant_phase: f32,
}

impl AnyWave {
    #[inline]
    pub fn new(freq: f32, master: f32) -> AnyWave {
        AnyWave {
            freq,
            num_sample: 0,
            sampling_rate: 44100,
            master,
            instant_phase: 0.0,
        }
    }
}

impl Iterator for AnyWave {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        let k: f32 = 200.0;
        self.num_sample = self.num_sample.wrapping_add(1);
        let t = self.num_sample as f32 / self.sampling_rate as f32;
        let modulator = k * dsp::oscillator::osc_triangle(t, 0.0, 2.0, 1.0, 0.5);
        self.instant_phase += 2.0 * PI * self.freq / self.sampling_rate as f32
            + 2.0 * modulator / self.sampling_rate as f32;
        let value = (self.instant_phase).sin();
        println!("{}", value);
        Some(value * self.master)
    }
}

impl Source for AnyWave {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]

    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        self.sampling_rate
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

fn main() {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    //  let file = BufReader::new(File::open("example.ogg").unwrap());
    // Decode that sound file into a source
    //  let source = Decoder::new(file).unwrap();
    let source = AnyWave::new(1000.0, 0.5);
    // Play the sound directly on the device
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(source);
    sink.sleep_until_end();
}
