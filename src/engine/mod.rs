pub mod audio_thread;
pub mod effects;
pub mod mixer;
pub mod sampler;
pub mod sequencer;
pub mod synth;

use crate::bridge::EngineHandle;
use crate::project::project::Project;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream};

pub struct AudioEngine {
    _stream: Stream,
}

impl AudioEngine {
    pub fn new(handle: EngineHandle, project: &Project) -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("No output device");
        let config = device.default_output_config().expect("No default config");
        log::info!(
            "Audio device: {}, sample rate: {}",
            device.name().unwrap_or_default(),
            config.sample_rate().0
        );

        let sample_rate = config.sample_rate().0 as f64;
        let bpm = project.bpm;
        let channels = config.channels() as usize;

        let stream = match config.sample_format() {
            SampleFormat::F32 => {
                let mut state = audio_thread::AudioState::new(sample_rate, bpm, channels, handle);
                device
                    .build_output_stream(
                        &config.into(),
                        move |data: &mut [f32], _| {
                            state.fill_buffer(data);
                        },
                        |err| log::error!("Audio stream error: {err}"),
                        None,
                    )
                    .expect("Failed to build stream")
            }
            _ => panic!("Unsupported sample format"),
        };

        stream.play().expect("Failed to start stream");
        Self { _stream: stream }
    }
}
