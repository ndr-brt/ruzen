extern crate cpal;
extern crate failure;

use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::{Format, OutputBuffer, Device, Host, ChannelCount};
use std::thread;
use std::sync::mpsc::{Receiver, sync_channel};
use cpal::StreamData::Output;
use cpal::UnknownTypeOutputBuffer::{F32, I16, U16};
use std::f64::consts::PI;

const LATENCY: u8 = 250;

pub type Hz = f64;

fn main() {
    let out = Out::init().unwrap_or_else(|e| panic!(e));
    let (_sig_out, sig_in) = sync_channel::<f64>(out.buffer_size());
    let mut clock = Clock::new(out.sample_rate());

    thread::spawn(move || out.loop_forever(sig_in));

    thread::spawn(move || {
        loop {
            clock.tick();
            let signal = (clock.get() * 440.0 * 2.0 * PI).sin();
            let result = _sig_out.send(signal * 200000.0);
            match result {
                Ok(_data) => (),
                Err(err) => println!("{}", err)
            }
        };
    });

    loop { }
}

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

    pub fn loop_forever(&self, sig_in: Receiver<f64>) {
        let channels: ChannelCount = *&self.format.channels;
        let event_loop = &self.host.event_loop();
        let stream_id = event_loop.build_output_stream(&self.device, &self.format).unwrap();
        match event_loop.play_stream(stream_id.clone()) {
            Ok(result) => println!("{:?}", result),
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
                Output { buffer: F32(buffer) } => feed_buffer(buffer, &sig_in, channels as usize),
                Output { buffer: I16(buffer) } => feed_buffer(buffer, &sig_in, channels as usize),
                Output { buffer: U16(buffer) } => feed_buffer(buffer, &sig_in, channels as usize),
                _ => panic!("Unexpected buffer type.")
            }
        })
    }
}

fn feed_buffer<T: SampleFromF64>(mut buffer: OutputBuffer<'_, T>, sig_in: &Receiver<f64>, channels: usize) {
    for buff_chunks in buffer.chunks_mut(channels) {
        match sig_in.recv() {
            Ok(sample) =>
                for out in buff_chunks.iter_mut() {
                    *out = T::from_f64(sample);
                },
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

struct Clock {
    sample_rate: Hz,
    clock: f64,
}
impl Clock {

    fn new(sample_rate: Hz) -> Clock {
        Clock{ sample_rate, clock: 0. }
    }

    fn tick(&mut self) -> f64 {
        self.clock += 1.0;
        self.get()
    }

    fn get(&self) -> f64 {
        self.clock / self.sample_rate
    }

}