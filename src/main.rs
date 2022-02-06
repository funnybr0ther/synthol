mod math;
mod dsp;

use std::any::Any;
use std::f32::consts::PI;
use std::fs::File;
use std::io::BufReader;
use std::ops::Rem;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink, Source};

#[derive(Clone, Debug)]
pub struct AnyWave {
        freq: f32,
        num_sample: usize,
        sampling_rate: u32
}

impl AnyWave {
        /// The frequency of the sine.
        #[inline]
        pub fn new(freq: f32) -> AnyWave {
                AnyWave {
                        freq: freq,
                        num_sample: 0,
                        sampling_rate: 44100
                }
        }
}

impl Iterator for AnyWave {
        type Item = f32;

        #[inline]
        fn next(&mut self) -> Option<f32> {
                let k:f32 = 150.0;
                let fm_hz = 2.0;
                self.num_sample = self.num_sample.wrapping_add(1);
                let carrier = 2.0 * PI * self.num_sample as f32 * self.freq / self.sampling_rate as f32;
                let modulator = k * (2.0 * PI * self.num_sample as f32 * fm_hz / self.sampling_rate as f32).sin();
                let value = (carrier + modulator).cos()*0.3;
                Some(value)
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
        let source = AnyWave::new(1000.0);
    // Play the sound directly on the device
        let sink = Sink::try_new(&stream_handle).unwrap();

        sink.append(source);
        sink.sleep_until_end();
}
