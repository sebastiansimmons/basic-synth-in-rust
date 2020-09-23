#[macro_use]
extern crate vst;

use vst::plugin::{Info, Plugin};

#[derive(Default)]
struct Whisper;

impl Plugin for Whisper_vst {
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
}

plugin_main!(Whispter_vst);
