#[macro_use]
extern crate vst;

use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::api::Events;
use rand::random;

#[derive(Default)]
struct Whisper {
    notes: u8,
}

impl Plugin for Whisper {
    fn get_info(&self) -> Info {
        Info {
            name: "whispter_vst".to_string(),

            vendor: "Sebastian Simmons".to_string(),

            unique_id: 1337,

            // Set to 0 since this is a synth
            inputs: 0,

            outputs: 2,

            category: Category::Synth,

            // Fill the rest with default values
            ..Default::default()
        }
    }

    fn process_events(&mut self, events: &Events) {
        // not all events are midi
        for event in events.events() {
            match event {
                Event::Midi(ev) => {
                    match ev.data[0]{
                        // Note on
                        144 => self.notes += 1u8,

                        // Note off
                        128 => self.notes -= 1u8,

                        _ => (),
                    }

                },
                // ignore all other events
                _ => (),
            }
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        // returns if no midi note is being pressed
        if self.notes == 0 { return }

        let (_, mut output_buffer) = buffer.split();

        for output_channel in output_buffer.into_iter() {
            // Let's iterate over every sample in our channel.
            for output_sample in output_channel {
                // For every sample, we want to add a random value from
                // -1.0 to 1.0.
                *output_sample = (random::<f32>() - 0.5f32) * 2f32;
            }
        }

    }
}

plugin_main!(Whisper);
