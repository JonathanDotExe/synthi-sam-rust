use synthi_sam_core::core::{device::{Device, DeviceInfo, NamedAudioPort, NamedMidiPort}, audio::{ProcessingInfo, SampleInfo}, midi::MidiMessage};

struct DemoDevice {
    info: DeviceInfo,
    output: NamedAudioPort,
    midiin: NamedMidiPort,
    processing_info: ProcessingInfo,
}

impl DemoDevice {

    fn new() -> DemoDevice {
        return DemoDevice {
            info: DeviceInfo {
                name: "Demo Synth",
                type_identifier: "synthi_sam_demo_synth"
            }, 
            output: NamedAudioPort::new("Stereo Out", "stereo_out", 2), 
            midiin: NamedMidiPort::new("MIDI In", "midi_in"),
            processing_info: ProcessingInfo::default(),
        }
    }

}

impl Device for DemoDevice {

    fn info(&self) -> &DeviceInfo {
        return &self.info;
    }

    fn setup(&mut self, info: ProcessingInfo) {
        self.processing_info = info;
        self.output.port.reset();
    }

    fn process(&mut self, info: SampleInfo) {

    }

    
    fn audio_input_port(&mut self, _: usize) -> Option<&mut NamedAudioPort> {
        return None;
    }

    fn audio_output_port(&mut self, index: usize) -> Option<&mut NamedAudioPort> {
        return match index {
            0 => Some(&mut self.output),
            _ => None,
        }
    }

    fn midi_input_port(&mut self, index: usize) -> Option<&mut NamedMidiPort> {
        return match index {
            0 => Some(&mut self.midiin),
            _ => None,
        }
    }

    fn midi_output_port(&mut self, _: usize) -> Option<&mut NamedMidiPort> {
        return None
    }

}


fn main() {
    println!("Hello, world!");
}
