extern crate cpal;
extern crate failure;
extern crate rand;

use std::sync::mpsc::{sync_channel, channel};
use std::thread;

use crate::out::Out;
use crate::synth::{Synth, Command };
use crate::osc_server::OscServer;

mod clock;
mod envelope;
mod out;
mod plot;
mod osc_server;
mod command_factory;
mod instrument;
mod synth;
mod signal;

fn main() {
    let out = Out::init().unwrap_or_else(|e| panic!(e));
    let (cmd_out, cmd_in) = channel::<Command>();
    let (sig_out, sig_in) = sync_channel::<f64>(out.buffer_size());
    let sample_rate = out.sample_rate();
    let synth = Synth::new(sample_rate);

    thread::spawn(move || out.loop_forever(sig_in));
    thread::spawn(move || synth.loop_forever(cmd_in, sig_out));

    plot();

    let osc_server = OscServer::new("127.0.0.1:38042");
    osc_server.listen(cmd_out);


}

// TODO: put plot in a different module

use gnuplot::{Figure, AxesCommon};
use gnuplot::Coordinate::Graph;
use gnuplot::PlotOption::Caption;
use crate::envelope::Envelope;

fn plot() {
    let envelope = Envelope::AR(0.05, 0.5, -4.);

    const SAMPLE_RATE: usize = 1000;

    let mut x_axis: [usize; SAMPLE_RATE as usize] = [0; SAMPLE_RATE as usize];
    let mut values: [f64; SAMPLE_RATE as usize] = [0.; SAMPLE_RATE as usize];
    for x in 0..SAMPLE_RATE  {
        x_axis[x] = x;
        values[x] = envelope.value_at(x as f64/SAMPLE_RATE as f64);
    }

    let mut fg = Figure::new();
    let axes = fg.axes2d()
        .set_title("A plot", &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .set_x_label("x", &[])
        .set_y_label("y^2", &[])
        .lines(
            x_axis.iter(),
            values.iter(),
            &[Caption("Parabola")],
        );

    fg.show();
}