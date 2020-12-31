use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, _app: &mut AppBuilder) {
        println!("SamplePlugin running...")
    }
}