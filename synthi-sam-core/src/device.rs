use crate::audio::AudioPort;


pub struct NamedAudioPort {
    name: &'static str,
    pub port: AudioPort,
}

impl NamedAudioPort {
    
    pub fn new(name: &'static str, channels: usize) -> NamedAudioPort {
        return NamedAudioPort {
            name: name,
            port: AudioPort::new(channels),
        };
    }

    #[inline(always)]
    pub fn get_name(&self) -> &'static str {
        return self.name;
    }

}

pub struct DeviceInfo {

}

pub trait Device {
    fn info(&mut self) -> &mut DeviceInfo;

    fn process();

}

