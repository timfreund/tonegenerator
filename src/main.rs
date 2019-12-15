// extern crate crossbeam_channel;
extern crate hex;
extern crate jack;

// use crossbeam_channel::bounded;
use std::{thread, time};


fn midi_to_frequency(midi_note: u8) -> f64 {
    let midi_f = midi_note as f64;
    return 2_f64.powf((midi_f - 69.0) / 12.0) * 440.0;
}

fn main() {
    let(jack_client, _status) = jack::Client::new("tone_generator", jack::ClientOptions::NO_START_SERVER).unwrap();
    let mut audio_out = jack_client.register_port("audio_out", jack::AudioOut::default()).unwrap();
    let midi_in = jack_client.register_port("midi_in", jack::MidiIn::default()).unwrap();
    println!("{}", midi_to_frequency(67));

    let mut frequency = midi_to_frequency(67);
    let sample_rate = jack_client.sample_rate();
    let frame_t = 1.0 / sample_rate as f64;
    let mut time = 0.0;

    let jack_callback = jack::ClosureProcessHandler::new(
        move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
            let out = audio_out.as_mut_slice(ps);

            for v in out.iter_mut() {
                let x = frequency * time * 2.0 * std::f64::consts::PI;
                let y = x.sin();
                *v = y as f32;
                time += frame_t;
            }

            let midi_data = midi_in.iter(ps);
            for raw_midi in midi_data {
                println!("{}", hex::encode(raw_midi.bytes));
                let midi_note = raw_midi.bytes[1];
                frequency = midi_to_frequency(midi_note);
                println!("{}", frequency);
            }
            jack::Control::Continue
        },
    );

    let active_jack_client = jack_client.activate_async((), jack_callback).unwrap();

    let sleep_period = time::Duration::from_millis(500);
    loop {
        thread::sleep(sleep_period);
    }
}
