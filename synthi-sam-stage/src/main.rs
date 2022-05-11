use synthi_sam_core::{audio::{AudioPort, ProcessingInfo}, device::{NamedMidiPort, DeviceInfo, NamedAudioPort}};

struct DemoDevice {
    info: DeviceInfo,
    output: NamedAudioPort,
    midiin: NamedMidiPort,
    processing_info: ProcessingInfo,
}

impl Device {

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

    fn recieve_midi(&mut self, msg: MidiMessage, info: SampleInfo, port: usize) {

    }

    fn audio_input_port(&mut self, index: usize) -> Result<&mut NamedAudioPort, &'static str> {
        return None;
    }

    fn audio_output_port(&mut self, index: usize) -> Result<&mut NamedAudioPort, &'static str> {
        return match index {
            0 => Some(&self.output),
            _ => None,
        }
    }

    fn midi_input_port(&mut self, index: usize) -> Result<&mut NamedMidiPort, &'static str> {
        return match index {
            0 => Some(&self.midiin),
            _ => None,
        }
    }

    fn midi_output_port(&mut self, index: usize) -> Result<&mut NamedMidiPort, &'static str> {
        return None
    }

}


fn main() {
    println!("Hello, world!");
}
