use metronome::MyPlugin;
use nih_plug::prelude::nih_export_standalone;

fn main() {
    nih_export_standalone::<MyPlugin>();
}
