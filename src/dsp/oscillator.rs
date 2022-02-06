/// Base oscillator types (including `Noise`) or custom oscillator
pub enum OscillatorType {
    /// Sine function
    Sine,
    /// Square function of any duty cycle (ratio of positive signal over a whole cycle)
    Square{duty_cycle: f32},
    /// Impulse function with any duty cycle (ratio of on signal over a whole cycle)
    Impulse{duty_cycle: f32},
    /// Triangle wave with any duty cycle
    Triangle{duty_cycle: f32},
    /// Noise function TODO: Filter after generating white noise, or generate any color of noise
    Noise,
    /// Other (custom) waveform. See struct [CustomWaveForm]
    Other{defining_function: CustomWaveForm }
}

pub struct CustomWaveForm {
    function: fn(Vec<f32>) -> f32,
    name: String,
    parameters_count: u32
}

struct Oscillator{
    freq: f32,
    amplitude: f32,
    phase: f32,
    oscillator_type: OscillatorType
}
