use crate::audio::{self as audio};

#[derive(PartialEq)]
pub enum VoiceState {
    Incative,
    Pressed,
    Released
}

impl Default for VoiceState {
    fn default() -> Self {
        return VoiceState::Incative
    }
}

#[derive(Default)]
pub struct Voice<T> where T: Default{
    pub state: VoiceState,
    pub note: u32,
    pub velocity: u32,
    pub press_time: f64,
    pub release_time: f64,
    pub data: T,
}

pub struct VoiceManager<T> where T: Default{
    pub voices: Vec<Voice<T>>,
}

pub trait VoiceProcessor<T> where T: Default{

    /**
     * Called when a note was pressed
     */
    fn voice_on(&mut self, _voice: &mut Voice<T>, _info: audio::SampleInfo) {

    }

    /**
     * Process voices
     */
    fn process_voice(&mut self, voice: &mut Voice<T>, info: audio::SampleInfo) -> f64;

    /**
     * Checks if a not can be set invalid now
     */
    fn check_inactive(&mut self, voice: &Voice<T>, _info: audio::SampleInfo) -> bool {
        return voice.state != VoiceState::Pressed;
    }

    /**
     * Called when a note was released
     */
    fn voice_off(&mut self, _voice: &mut Voice<T>, _info: audio::SampleInfo) {

    }

}

impl<T> VoiceManager<T> where T: Default {
    //Init voice manager with a specific amount of polyphony
    pub fn new(size: usize) -> VoiceManager<T> {
        let mut mgr: VoiceManager<T> = VoiceManager {
            voices: Vec::new()
        };
        for _i in 0..size {
            let voice: Voice<T> = Voice::default();
            mgr.voices.push(voice);
        }
        return mgr;
    }

    pub fn reset<E: VoiceProcessor<T>>(&mut self, proc: &mut E, info: audio::SampleInfo) {
        for mut voice in self.voices.iter_mut() {
            voice.state = VoiceState::Incative;
            voice.release_time = info.time;
            proc.voice_off(&mut voice, info);
        }
    }

    fn find_next_slot(&mut self) -> usize {
        let mut released = false;
        let mut longest_index: usize = 0;
        let mut longest_time: f64 = f64::MAX;

        //TODO refactor, bad code when more states are added
        for i in 0..self.voices.len() {
            let voice: &Voice<T> = &self.voices[i];
            if voice.state == VoiceState::Incative {    // Note is free, use it
                return i;
            }
            if voice.state == VoiceState::Released {
                if released {  // Only counting released notes
                    if voice.release_time < longest_time {
                        longest_index = i;
                        longest_time = voice.release_time;
                    }
                }
                else {  // First released note, only count released notes from now on
                    longest_index = i;
                    longest_time = voice.release_time;
                    released = true;
                }
            }
            else if voice.press_time < longest_time{ // Check for pressed notes
                longest_index = i;
                longest_time = voice.press_time;
            }
        }

		return longest_index;
    }

    pub fn press_note<E: VoiceProcessor<T>>(&mut self, proc: &mut E, note: u32, velocity: u32, info: audio::SampleInfo) {
        let index = self.find_next_slot();
        self.voices[index].note = note;
        self.voices[index].velocity = velocity;
        self.voices[index].state = VoiceState::Pressed;
        self.voices[index].press_time = info.time;
        self.voices[index].release_time = 0.0;

        proc.voice_on(&mut self.voices[index], info);
    }

    pub fn release_note<E: VoiceProcessor<T>>(&mut self, proc: &mut E, note: u32, info: audio::SampleInfo) {
        for mut voice in self.voices.iter_mut() {
            if voice.note == note {     //Check if note is equal
                voice.state = VoiceState::Released;
                voice.release_time = info.time;
                proc.voice_off(&mut voice, info);
            }
        }
    }
  
    pub fn process_voices<E: VoiceProcessor<T>>(&mut self, proc: &mut E, info: audio::SampleInfo) -> f64 {
        let mut sample = 0.0;
        for mut voice in self.voices.iter_mut() {
            if voice.state != VoiceState::Incative {
                //Process sound
                sample += proc.process_voice(&mut voice, info);
                //Invalidate note
                if proc.check_inactive(&voice, info) {
                    voice.state = VoiceState::Incative;
                }
            }
        }
        return sample;
    }
}