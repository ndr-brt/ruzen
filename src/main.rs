extern crate cpal;
extern crate failure;
extern crate rand;

use std::f64::consts::PI;
use std::sync::mpsc::{Receiver, sync_channel, SyncSender};
use std::thread;

use cpal::{ChannelCount, Device, Format, Host, OutputBuffer};
use cpal::StreamData::Output;
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use cpal::UnknownTypeOutputBuffer::{F32, I16, U16};
use rand::Rng;

use crate::clock::Clock;
use crate::clock::Hz;
use crate::envelope::Envelope;

const LATENCY: u8 = 250;

mod clock;
mod envelope;

fn main() {
    let out = Out::init().unwrap_or_else(|e| panic!(e));
    let (sig_out, sig_in) = sync_channel::<f64>(out.buffer_size());
    let sample_rate = out.sample_rate();

    thread::spawn(move || out.loop_forever(sig_in));
    thread::spawn(move || play(sample_rate, sig_out));

    loop { }
}

fn play(sample_rate: Hz, sig_out: SyncSender<f64>) -> () {
    let mut rng = rand::thread_rng();
    loop {
        let frequency: f64 = rng.gen_range(220.0, 440.0);
        println!("Frequency {}", frequency);
        let mut sine = Sine::new(sample_rate, frequency).envelope(Envelope::new(1., 1.));
        while !sine.is_finished() {

            let result = sig_out.send(sine.signal());
            match result {
                Ok(_data) => (),
                Err(err) => println!("{}", err)
            }
        };
    }

}

#[derive(Clone,Copy)]
pub struct Sine{
    frequency: f64,
    clock: Clock,
    envelope: Envelope,
}

impl Sine {
    pub fn new(sample_rate: Hz, frequency: Hz) -> Sine {
        Sine {
            frequency,
            clock: Clock::new(sample_rate),
            envelope: Envelope::new(0.0, 0.0)
        }
    }

    pub fn signal(&mut self) -> f64 {
        self.clock.tick();
        let signal = (self.clock.get() * self.frequency * 2.0 * PI).sin();
        if self.envelope.is_valid() {
            self.envelope.apply(self.clock.get(), signal)
        } else {
            signal
        }
    }

    pub fn envelope(&mut self, envelope: Envelope) -> Self {
        self.envelope = envelope;
        self.clone()
    }

    pub fn is_finished(&self) -> bool {
        self.envelope.is_valid() && self.envelope.duration() < self.clock.get()
    }
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
