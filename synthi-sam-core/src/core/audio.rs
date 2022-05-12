type AudioSample = f64;

#[derive(Copy, Clone)]
pub enum ProcessingMode {
    Realtime,   //Must respond in real time
    Offline,    //Processing is offline (e.g. rendering in a DAW)
}

impl Default for ProcessingMode {
    fn default() -> Self {
        return ProcessingMode::Realtime;
    }
}

#[derive(Copy, Clone, Default)]
pub struct ProcessingInfo {
    pub sample_rate: u32,
    pub time_step: f64,
    pub processing_mode: ProcessingMode,
}

#[derive(Copy, Clone, Default)]
pub struct SampleInfo {
    pub sample_count: u64,
    pub time: f64,
    pub jitter: bool, //Indicates wether the sample function is called at a fixed (false) or variable (true) pace
}

pub struct AudioPort {
    channels: Box<[AudioSample]>,
}

/// A connection point for sending audio from one component to another holding one sample for each channel at a time.
impl AudioPort {

    /// Creates an AudioPort with the specified amount of channels
    pub fn new(channels: usize) -> AudioPort {
        return AudioPort {
            channels: vec![0.0; channels].into_boxed_slice()
        }
    }


    /// Loads the sample from the channels and spreads them across it's own channels
    /// 
    /// If the input has the same amount of channels as the port the content will be copied
    /// If the input has no channel, the entire port will be filled with 0
    /// If the input is mono, the entire port will be filled with the value
    /// If the input has more or less channels, the input samples will be spread across the port channels via modulo
    ///      An input of [1, 2, 3, 4] to a port with 3 channels will yield [5, 2, 3]
    ///      An input of [1, 2, 3, 4] to a port with 5 channels will yield [1, 2, 1, 2, 1]
    pub fn take_input(&mut self, sample: &[AudioSample]) {
        match sample.len() {
            0 => self.channels.fill(0.0), //Empty input, clear channel
            1 => self.channels.fill(sample[0]), //Mono input, fill channel
            _ => {
                if self.channels.len() < sample.len() { //Less channels than input => restarting at the end
                    self.channels.fill(0.0); //Clear
                    for (i, s) in sample.iter().enumerate() {
                        self.channels[i % self.channels.len()] += s;
                    }
                }
                else if self.channels.len() > sample.len() { //More channels than input => restarting at the end
                    for (i, s) in self.channels.iter_mut().enumerate() {
                        *s += sample[i % sample.len()];
                    }
                }
                else {
                    self.channels.copy_from_slice(sample); //Same size, copy
                }
            }
        }
    }

    /// Takes input from another port
    #[inline(always)]
    pub fn take_input_from_port(&mut self, port: &AudioPort) {
        self.take_input(&port.channels);
    }

    /// Returns the current sample in all channels
    #[inline(always)]
    pub fn channels(&self) -> &[AudioSample] {
        return &self.channels;
    }

    /// Fills all channels with 0
    #[inline(always)]
    pub fn reset(&mut self) {
        self.channels.fill(0.0);
    }

}