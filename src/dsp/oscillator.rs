use rand::distributions::{Distribution, Normal};
use std::f32::consts::*;

/// Base oscillator types (including `Noise`) or custom oscillator
pub enum OscillatorType {
    /// Sine function
    Sine(SinOscillator),
    /// Square function of any duty cycle (ratio of positive signal over a whole cycle)
    Square { duty_cycle: f32 },
    /// Impulse function with any duty cycle (ratio of on signal over a whole cycle)
    Impulse { duty_cycle: f32 },
    /// Triangle wave with any duty cycle
    Triangle { duty_cycle: f32 },
    /// Noise function TODO: Filter after generating white noise, or generate any color of noise
    Noise,
    /// Other (custom) waveform. See struct [CustomWaveForm]
    Other { defining_function: CustomWaveForm },
}
/// Represents a custom waveform for a custom oscillator. The function can take [parameters_count]
/// parameters and can also be given a name.
pub struct CustomWaveForm {
    function: fn(&[f32], f32) -> f32,
    name: String,
    parameters_count: u32,
    parameters: OscillatorParameters,
}

pub struct OscillatorParameters {
    freq: f32,
    amplitude: f32,
    phase: f32,
}

pub struct SinOscillator {
    parameters: OscillatorParameters,
}

// pub fn eval_oscillator_time(oscillator: &Oscillator, t: f32) -> f32{
//     match oscillator.oscillator_type{
//         OscillatorType::Sine(osc) => {
//             osc.osc(t,oscillator.phase,oscillator.freq,oscillator.amplitude)
//         },
//         OscillatorType::Noise(osc) => {
//             osc.osc(t)
//         }
//     }
// }

// trait OscTrait {
//     type Parameter: Copy;
//
//     fn osc(&self, p: Self::Parameter) -> f32;
// }
//
// impl OscTrait for Oscillator<Sine, ()> {
//     fn osc(&self) -> f32 {
//         // ...
//     }
// }
//
// impl OscTrait for Oscillator<Square, ()>

pub fn osc_sin(t: f32, phase: f32, freq: f32, amplitude: f32) -> f32 {
    (freq * 2.0 * PI * t + phase * 2.0 * PI).sin() * amplitude
}

pub fn osc_square(t: f32, phase: f32, freq: f32, amplitude: f32, duty_cycle: f32) -> f32 {
    let period = 1.0 / freq;
    let time_relative = ((t + phase).rem_euclid(period)) / period;
    if time_relative < duty_cycle {
        amplitude
    } else {
        -amplitude
    }
}

pub fn osc_noise(amplitude: f32) -> f32 {
    let normal = Normal::new(0.0, 1.0);
    let v = normal.sample(&mut rand::thread_rng());
    v as f32 * amplitude
}

pub fn osc_triangle(t: f32, phase: f32, freq: f32, amplitude: f32, duty_cycle: f32) -> f32 {
    let period = 1.0 / freq;
    let time_relative = ((t + phase).rem_euclid(period)) / period;
    if (duty_cycle == 0.0) | (duty_cycle == 1.0) {
        ((duty_cycle - 0.5) * 2.0) * (-amplitude + amplitude * 2.0 * time_relative)
    } else if time_relative < duty_cycle {
        -amplitude + amplitude * 2.0 * (time_relative / duty_cycle)
    } else {
        amplitude - amplitude * 2.0 * ((1.0 - time_relative) / (1.0 - duty_cycle))
    }
}
