use io::AudioMidiProcessor;
use synth::DemoDevice;
use synthi_sam_core::core::device::Device;

mod synth;
mod io;

struct DemoProcessor {
    synth: DemoDevice,
}

impl AudioMidiProcessor for DemoProcessor {
    fn setup(&mut self, info: synthi_sam_core::core::audio::ProcessingInfo) {
        self.synth.setup(info);
    }

    fn process(&mut self, info: synthi_sam_core::core::audio::SampleInfo) -> (f64, f64) {
        self.synth.process(info);
        return match self.synth.audio_output_port(0) {
            Some(port) => (port.port.channels()[0], port.port.channels()[1]),
            _ => (0.0, 0.0)
        }
    }

    fn recieve_midi(&mut self, msg: synthi_sam_core::core::midi::MidiMessage, _info: synthi_sam_core::core::audio::SampleInfo) {
        match self.synth.midi_input_port(0) {
            Some(port) => port.port.queue(msg),
            None => {}
        }
    }
}

fn main() {
    //Audio
    let synth: Box<DemoProcessor> = Box::new(DemoProcessor { synth: DemoDevice::new() });
    let mut _handler = io::AudioMidiHandler::new(synth);
}
