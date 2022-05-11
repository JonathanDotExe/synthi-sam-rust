use crate::util::get_default;

#[derive(PartialEq, Debug)]
pub struct NoteEvent {
    pub note: u8,
    pub velocity: f64,
}

#[derive(PartialEq, Debug)]
pub struct PolyphonicAftertouchEvent {
    pub note: u8,
    pub aftertouch: f64,
}

#[derive(PartialEq, Debug)]
pub struct MonophonicAftertouchEvent {
    pub aftertouch: f64,
}

#[derive(PartialEq, Debug)]
pub struct ControlChangeEvent {
    pub control: u8,
    pub value: f64,
}

#[derive(PartialEq, Debug)]
pub struct ProgramChangeEvent {
    pub program: u8,
}


#[derive(PartialEq, Debug)]
pub struct PitchBendEvent {
    pub pitch_bend: f64,
}

#[derive(PartialEq, Debug)]
pub struct SysExEvent {
    pub data: Vec<u8>,
}

#[derive(PartialEq, Debug)]
pub enum MidiMessageContent {
    NoteOff(NoteEvent),
    NoteOn(NoteEvent),
    PolyphonicAftertouch(PolyphonicAftertouchEvent),
    ControlChange(ControlChangeEvent),
    ProgramChange(ProgramChangeEvent),
    MonophonicAftertouch(MonophonicAftertouchEvent),
    PitchBend(PitchBendEvent),
    SysEx(SysExEvent)
}

#[derive(Debug)]
pub struct MidiMessage {
    pub channel: u8,
    pub message: MidiMessageContent,
}

impl MidiMessage {

    pub fn new(data: &[u8]) -> Result<MidiMessage, &'static str> {
        let message_type: MidiMessageContent;
        let channel: u8;

        //Message type
        if data.len() > 0 {
            match data[0] & 0xF0 {
                0x80 => message_type = MidiMessageContent::NoteOff(NoteEvent{ note: get_default(&data, 1, 0), velocity: (get_default(&data, 2, 0) as f64)/127.0} ),
                0x90 => {
                    let vel = (get_default(&data, 2, 0) as f64)/127.0;
                    if (vel > 0.0) {
                        message_type =  MidiMessageContent::NoteOn(NoteEvent{ note: get_default(&data, 1, 0), velocity: vel} );
                    }
                    else {
                        message_type =  MidiMessageContent::NoteOff(NoteEvent{ note: get_default(&data, 1, 0), velocity: vel} );
                    }
                },
                0xA0 => message_type = MidiMessageContent::PolyphonicAftertouch(PolyphonicAftertouchEvent{ note: get_default(&data, 1, 0), aftertouch: (get_default(&data, 2, 0) as f64)/127.0} ),
                0xB0 => message_type = MidiMessageContent::ControlChange(ControlChangeEvent{ control: get_default(&data, 1, 0), value: (get_default(&data, 2, 0) as f64)/127.0} ),
                0xC0 => message_type = MidiMessageContent::ProgramChange(ProgramChangeEvent{ program: get_default(&data, 1, 0) }),
                0xD0 => message_type = MidiMessageContent::MonophonicAftertouch(MonophonicAftertouchEvent{ aftertouch: (get_default(&data, 1, 0) as f64)/127.0} ),
                0xE0 => {
                    let first = (get_default(&data, 1, 0) & 0b0111_1111) as f64;
                    let second = (get_default(&data, 2, 0) & 0b0111_1111) as f64;
                    message_type = MidiMessageContent::PitchBend(PitchBendEvent{ pitch_bend: (first + second * 128.0)/8192.0 - 1.0} );
                },
                0xF0 => message_type = MidiMessageContent::SysEx(SysExEvent{data: data.to_vec()}),
                _ =>  return Err("Invalid message type!"),
            }
            channel = (data[0] & 0x0F);
        }
        else {
            return Err("Empty message!")
        }

        return Ok(MidiMessage {
            message: message_type,
            channel: channel,
        });
    }

}
