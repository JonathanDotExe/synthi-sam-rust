use midir::{MidiInput, MidiInputConnection, Ignore};
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use lockfree::channel::spsc;
use synthi_sam_core::core::{midi::MidiMessage, audio::{ProcessingInfo, ProcessingMode, SampleInfo}};


pub trait AudioMidiProcessor {

    fn setup(&mut self, info: ProcessingInfo);

    fn process(&mut self, info: SampleInfo) -> (f64, f64);

    fn recieve_midi(&mut self, msg: MidiMessage, info: SampleInfo);

}

pub struct AudioMidiHandler {
    _midiconn: MidiInputConnection<()>,
    _stream: Box<dyn cpal::traits::StreamTrait>,
}

impl AudioMidiHandler {

    pub fn new<>(mut processor: Box<dyn AudioMidiProcessor + Send>)-> AudioMidiHandler {
        //Midi Queue
        let (mut sender, mut reciever) = spsc::create::<MidiMessage>();

        //Audio
        // Create audio pipeline
        let host = cpal::default_host();
        let device = host.default_output_device().expect("No device available!");
        let sample_rate = 48000;
        //TODO query configs
        /*let config = cpal::StreamConfig{
            channels: 2,
            buffer_size: cpal::BufferSize::Fixed(256),
            sample_rate: cpal::SampleRate(sample_rate),
        };*/
        let range = device.supported_output_configs().expect("Error!").next().expect("No config found!");
        println!("{} / {}", range.min_sample_rate().0, range.max_sample_rate().0);
        let channels = range.channels() as u128;
        let config = range.with_sample_rate(cpal::SampleRate(sample_rate)).config();
        let time_step: f64 = 1.0/(sample_rate as f64);

        let info = ProcessingInfo {sample_rate: sample_rate, time_step: time_step, processing_mode: ProcessingMode::Realtime};
        let mut sample_info = SampleInfo { sample_count: 0, time: 0.0, jitter: false };
        let mut curr_ch = channels;
        let mut curr_l: f64 = 0.0;
        let mut curr_r: f64 = 0.0;

        processor.setup(info);
        let stream = device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| { 
                for sample in data.iter_mut() {
                    if curr_ch > channels {
                        //Check midi
                        let mut msg = reciever.recv();
                        while msg.is_ok() {
                            let message = msg.unwrap();
                            println!("Midi Message {}", message.channel);
                            processor.recieve_midi(message, sample_info);
                            msg = reciever.recv();
                        }
                        //Process
                        (curr_l, curr_r) = processor.process(sample_info);
                        curr_ch = 0;
                        //Increment sample info
                        sample_info.sample_count += 1;
                        sample_info.time += info.time_step;
                    }
                    if curr_ch % 2 == 0 {
                        *sample = curr_l as f32;
                    }
                    else {
                        *sample = curr_r as f32;
                    }

                    curr_ch += 1;
                }
            },
            move |_err| {
                println!("Error while running audio thread!")
            },
        ).unwrap();
        stream.play().unwrap();
        //MIDI
        let mut midiin = MidiInput::new("App-In").unwrap();
        midiin.ignore(Ignore::None);
        let ports = midiin.ports();
        if ports.len() <= 0 {
            panic!("No MIDI port found!");
        }
        let port = &ports[0];
        println!("Using port {}!", midiin.port_name(&port).unwrap());

        let midiconn = midiin.connect(port, "App-In", move |_stamp, message, _| {
            println!("Sending mesage!");
            let msg = MidiMessage::new(message);
            if msg.is_ok() {
                sender.send(msg.unwrap()).unwrap();
            }
        }, ()).unwrap();

        return AudioMidiHandler {
            _midiconn: midiconn,
            _stream: Box::new(stream),
        };
    }

}