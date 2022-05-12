use super::{audio::{AudioPort, ProcessingInfo, SampleInfo}, midi::{MidiMessage, MidiPort}};

pub struct NamedAudioPort {
    name: &'static str,
    identifier: &'static str,
    pub port: AudioPort,
}

impl NamedAudioPort {
    
    pub fn new(name: &'static str, identifier: &'static str, channels: usize) -> NamedAudioPort {
        return NamedAudioPort {
            name: name,
            identifier: identifier,
            port: AudioPort::new(channels),
        };
    }

    #[inline(always)]
    pub fn get_name(&self) -> &'static str {
        return self.name;
    }

    #[inline(always)]
    pub fn get_identifier(&self) -> &'static str {
        return self.identifier;
    }

}

pub struct NamedMidiPort {
    name: &'static str,
    identifier: &'static str,
    pub port: MidiPort,
}

impl NamedMidiPort {
    
    pub fn new(name: &'static str, identifier: &'static str) -> NamedMidiPort {
        return NamedMidiPort {
            name: name,
            identifier: identifier,
            port: MidiPort::new(),
        };
    }

    #[inline(always)]
    pub fn get_name(&self) -> &'static str {
        return self.name;
    }

    #[inline(always)]
    pub fn get_identifier(&self) -> &'static str {
        return self.identifier;
    }

}

pub struct DeviceInfo {
    pub name: &'static str,
    pub type_identifier: &'static str,
}

pub trait Device {

    fn info(&self) -> &DeviceInfo;

    fn setup(&mut self, info: ProcessingInfo);

    fn process(&mut self, info: SampleInfo);


    fn audio_input_port(&mut self, index: usize) -> Option<&mut NamedAudioPort>;

    fn audio_output_port(&mut self, index: usize) -> Option<&mut NamedAudioPort>;

    fn midi_input_port(&mut self, index: usize) -> Option<&mut NamedMidiPort>;

    fn midi_output_port(&mut self, index: usize) -> Option<&mut NamedMidiPort>;

}
