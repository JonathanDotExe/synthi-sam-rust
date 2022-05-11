use std::fmt::Display;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WaveForm {
    Sine,
    Saw,
    Square,
}

impl Default for WaveForm {
    fn default() -> Self {
        return WaveForm::Sine;
    }
}

impl Display for WaveForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            WaveForm::Sine => "Sine",
            WaveForm::Square => "Square",
            WaveForm::Saw => "Saw",
        })
    }
}

impl WaveForm {
    fn synthesize(&self, phase: f64) -> f64{
        let f: f64;
        match self {
            WaveForm::Sine => f = (phase * (std::f64::consts::PI as f64) * 2.0).sin(),
            WaveForm::Square => {
                if phase < 0.5 {
                    f = 1.0;
                }
                else {
                    f = -1.0;
                }   
            },
            WaveForm::Saw => f = phase * 2.0 - 1.0,
        }
        return f;
    }
}

#[derive(Default)]
pub struct OscilatorConfig {
    pub waveform: WaveForm,
    pub freq: f64,
}

#[derive(Default)]
pub struct Oscillator {
    phase: f64,
}

impl Oscillator {

    pub fn process(&mut self, osc: OscilatorConfig, time_step: f64) -> f64 {
        self.phase += time_step * osc.freq;
        //Modulo
        while self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        //Synthesize
        return osc.waveform.synthesize(self.phase);
    }

}