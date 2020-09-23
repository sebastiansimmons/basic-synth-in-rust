#[macro_use]
extern crate vst;

use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use rand::random;

#[derive(Default)]
struct Whisper;

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

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
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
