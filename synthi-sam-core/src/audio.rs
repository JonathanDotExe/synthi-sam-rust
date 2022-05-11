type AudioSample = f64;

struct AudioPort {
    channels: Box<[AudioSample]>,
}

/**
 * A connection point for sending audio from one component to another.
 * 
 * 
 */
impl AudioPort {

    pub fn new(channels: usize) -> AudioPort {
        return AudioPort {
            channels: vec![0.0; channels].into_boxed_slice()
        }
    }

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

}