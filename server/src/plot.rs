use gnuplot::{Figure, AxesCommon};
use gnuplot::Coordinate::Graph;
use gnuplot::PlotOption::Caption;
use crate::ugen::{ValueAt, UGen};

pub(crate) trait Plot {
    fn plot(self);
}

impl<T> Plot for UGen<T> where T: ValueAt {
    fn plot(self) {
        const SAMPLE_RATE: usize = 1000;

        let mut x_axis: [usize; SAMPLE_RATE as usize] = [0; SAMPLE_RATE as usize];
        let mut values: [f64; SAMPLE_RATE as usize] = [0.; SAMPLE_RATE as usize];
        for x in 0..SAMPLE_RATE  {
            x_axis[x] = x;
            values[x] = self.value_at(x as f64/SAMPLE_RATE as f64);
        }

        let mut fg = Figure::new();
        fg.axes2d()
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
}