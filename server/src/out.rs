use cpal::{Format, ChannelCount, OutputBuffer, Host, Device};
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::UnknownTypeOutputBuffer::{F32, I16, U16};
use cpal::StreamData::Output;
use crossbeam_channel::Receiver;
use crate::{Sample};

const LATENCY: u8 = 250;

pub struct Out {
    host: Host,
    device: Device,
    format: Format,
}

impl Out {
    pub fn init() -> Result<Self, String> {
        let host = cpal::default_host();
        match host.default_output_device() {
            Some(device) => {
                device.default_output_format()
                    .map_err(|e| format!("Failed to get default output format. {:?}", e))
                    .map(|format| Out { host, device, format })
            },
            None => Err("Failed to get default output device".to_string())
        }
    }

    pub fn buffer_size(&self) -> usize {
        self.sample_rate() as usize / LATENCY as usize
    }

    pub fn sample_rate(&self) -> f64 {
        *&self.format.sample_rate.0 as f64
    }

    pub fn loop_forever(&self, signal_stream: Receiver<Sample>) {
        let channels: ChannelCount = *&self.format.channels;
        let event_loop = &self.host.event_loop();
        let stream_id = event_loop.build_output_stream(&self.device, &self.format).unwrap();
        match event_loop.play_stream(stream_id.clone()) {
            Ok(result) => println!("Stream started {:?}", result),
            Err(err) => println!("{}", err)
        };

        event_loop.run(move |id, result| {
            let data = match result {
                Ok(data) => data,
                Err(err) => {
                    println!("Error occurred on stream {:?}: {}", id, err);
                    return;
                }
            };

            match data {
                Output { buffer: F32(buffer) } => feed_buffer(buffer, &signal_stream, channels as usize),
                Output { buffer: I16(buffer) } => feed_buffer(buffer, &signal_stream, channels as usize),
                Output { buffer: U16(buffer) } => feed_buffer(buffer, &signal_stream, channels as usize),
                _ => panic!("Unexpected buffer type.")
            }
        })
    }
}

fn feed_buffer<T: SampleFromF64>(mut buffer: OutputBuffer<'_, T>, sig_in: &Receiver<Sample>, channels: usize) {
    for buff_chunks in buffer.chunks_mut(channels) {
        match sig_in.recv() {
            Ok(sample) => {
                for channel in buff_chunks.iter_mut() {
                    *channel = T::from_f64(sample)
                }
            }
            _ => {
                panic!("Sample channel hang up?");
            }
        }
    }
}

trait SampleFromF64: cpal::Sample {
    fn from_f64(value: f64) -> Self;
}
impl SampleFromF64 for f32 {
    fn from_f64(value: f64) -> Self {
        value as f32
    }
}
impl SampleFromF64 for i16 {
    fn from_f64(value: f64) -> i16 {
        (value * f64::from(std::i16::MAX)) as i16
    }
}
impl SampleFromF64 for u16 {
    fn from_f64(value: f64) -> u16 {
        ((value * 0.5 + 0.5) * f64::from(std::u16::MAX)) as u16
    }
}