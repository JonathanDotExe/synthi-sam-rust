use synthi_sam_core::{core::{device::{Device, DeviceInfo, NamedAudioPort, NamedMidiPort}, audio::{ProcessingInfo, SampleInfo}, midi::{MidiMessage, MidiMessageContent}}, dsp::{oscillator::{WaveForm, Oscillator, OscilatorConfig}, note_to_freq_transpose, note_to_freq}, util::voice::{VoiceManager, self}};


#[derive(Default)]
pub struct SynthVoice {
    pub osc1: Oscillator,
    pub osc2: Oscillator,
    pub freq: f64,
}

#[derive(Default, Copy, Clone)]
pub struct SynthPreset {
    osc1_waveform: WaveForm,
    osc2_waveform: WaveForm,
    detune: f64,
}

struct SynthProcessor {
    pub preset: SynthPreset,
    sample_rate: u32,
    time_step: f64,
}

impl voice::VoiceProcessor<SynthVoice> for SynthProcessor {

    fn process_voice(&mut self, voice: &mut voice::Voice<SynthVoice>, _info: SampleInfo) -> f64 {
        let osc1 = OscilatorConfig {waveform: self.preset.osc1_waveform, freq: voice.data.freq};
        let osc2 = OscilatorConfig {waveform: self.preset.osc2_waveform, freq: voice.data.freq * self.preset.detune};

        let sample = (voice.data.osc1.process(osc1, self.time_step) + voice.data.osc2.process(osc2, self.time_step)) * 0.5;

        return sample; //Mix both oscillators equally
    }

    fn voice_on(&mut self, voice: &mut voice::Voice<SynthVoice>, _info: SampleInfo) {
        voice.data.freq = note_to_freq(voice.note as f64);
    }

}

struct DemoDevice {
    info: DeviceInfo,
    output: NamedAudioPort,
    midiin: NamedMidiPort,

    voice_mgr: VoiceManager<SynthVoice>,
    proc: SynthProcessor
}

impl DemoDevice {

    fn new() -> DemoDevice {
        return DemoDevice {
            info: DeviceInfo {
                name: "Demo Synth",
                type_identifier: "synthi_sam_demo_synth"
            }, 
            output: NamedAudioPort::new("Mono Out", "mono_out", 1), 
            midiin: NamedMidiPort::new("MIDI In", "midi_in"),

            voice_mgr: VoiceManager::new(30),
            proc: SynthProcessor {
                preset: SynthPreset { //Simple fat saw patch
                    osc1_waveform: WaveForm::Saw,
                    osc2_waveform: WaveForm::Saw,
                    detune: note_to_freq_transpose(0.1),
                },
                sample_rate: 0,
                time_step: 0.0,
            }
        }
    }

}

impl Device for DemoDevice {

    fn info(&self) -> &DeviceInfo {
        return &self.info;
    }

    fn setup(&mut self, info: ProcessingInfo) {
        //Clear ports
        self.output.port.reset();
        self.midiin.port.reset();

        //Processor
        self.proc.sample_rate = info.sample_rate;
        self.proc.time_step = info.time_step;
        let i = SampleInfo {
            sample_count: 0,
            time: 0.0,
            jitter: false,
        };
        self.voice_mgr.reset(&mut self.proc, i);
    }

    fn process(&mut self, info: SampleInfo) {
        //Recieve MIDI
        while let Some(msg) = self.midiin.port.pop() {
            //Note on/off for voice manager
            match msg.message {
                MidiMessageContent::NoteOn(note) => self.voice_mgr.press_note(&mut self.proc, note.note, note.velocity, info),
                MidiMessageContent::NoteOff(note) => self.voice_mgr.release_note(&mut self.proc, note.note, info),
                _ => {},
            }
        }
        //Process voice mgr
        let sample = self.voice_mgr.process_voices(&mut self.proc, info);
        //Output
        self.output.port.take_input_mono(sample);
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
