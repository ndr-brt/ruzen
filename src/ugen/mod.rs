use std::ops::{Add, Mul};

pub mod generator;
pub mod envelope;

pub trait ValueAt {
    fn value_at(&self, clock: f64) -> f64;
}

pub struct UGen<T> where T: ValueAt {
    parameters: T,
}

pub trait Range<T> where T: ValueAt {
    type Output;

    fn range(self, low: f64, high: f64) -> Self::Output;
}

impl<T> ValueAt for UGen<T> where T: ValueAt {
    fn value_at(&self, clock: f64) -> f64 {
        self.parameters.value_at(clock)
    }
}

pub struct Summed<T, O> where T: ValueAt, O: ValueAt {
    first: UGen<T>,
    second: UGen<O>,
}

impl<T, O> ValueAt for Summed<T, O> where T: ValueAt, O: ValueAt {
    fn value_at(&self, clock: f64) -> f64 {
        self.first.value_at(clock) + self.second.value_at(clock)
    }
}

pub struct Multiplied<T, O> where T: ValueAt, O: ValueAt {
    first: UGen<T>,
    second: UGen<O>,
}

impl<T, O> ValueAt for Multiplied<T, O> where T: ValueAt, O: ValueAt {
    fn value_at(&self, clock: f64) -> f64 {
        self.first.value_at(clock) * self.second.value_at(clock)
    }
}

impl<T: 'static, O: 'static> Add<UGen<O>> for UGen<T> where T: ValueAt, O: ValueAt {
    type Output = UGen<Summed<T, O>>;

    fn add(self, other: UGen<O>) -> Self::Output {
        UGen {
            parameters: Summed { first: self, second: other },
        }
    }
}

impl<T: 'static, O: 'static> Mul<UGen<O>> for UGen<T> where T: ValueAt, O: ValueAt {
    type Output = UGen<Multiplied<T, O>>;

    fn mul(self, other: UGen<O>) -> Self::Output {
        UGen {
            parameters: Multiplied { first: self, second: other },
        }
    }
}

pub struct Constant<T> {
    value: T,
}

impl ValueAt for Constant<f64> {
    fn value_at(&self, _clock: f64) -> f64 {
        self.value
    }
}

impl From<f64> for UGen<Constant<f64>> {
    fn from(value: f64) -> Self {
        UGen {
            parameters: Constant { value },
        }
    }
}

pub struct Ranged<T> where T: ValueAt {
    signal: UGen<T>,
    low: f64,
    high: f64,
}

impl<T: 'static> Range<UGen<T>> for UGen<T> where T: ValueAt {
    type Output = UGen<Ranged<T>>;

    fn range(self, from: f64, to: f64) -> Self::Output {
        UGen {
            parameters: Ranged { signal: self, low: from, high: to }
        }
    }
}

impl<T> ValueAt for Ranged<T> where T: ValueAt {
    fn value_at(&self, clock: f64) -> f64 {
        let source_central_point = (1. + (-1.))/2.;
        let dest_central_point = (self.low + self.high)/2.;
        let add = dest_central_point - source_central_point;

        let source_amp = (1. - (- 1.));
        let dest_amp = (self.high - self.low);
        let mul = dest_amp / source_amp;

        (self.signal.value_at(clock) * mul) + add
    }
}


#[cfg(test)]
mod tests {
    use crate::ugen::{ValueAt, Range};
    use crate::ugen::generator::Sine;
    use crate::ugen::envelope::Envelope;
    use std::f64::consts::PI;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn range_on_a_generator() {
        let sine = Sine::new(1., 0.);

        let range = sine.range(0., 10.);

        assert_approx_eq!(range.value_at(0.), 5.);
        assert_approx_eq!(range.value_at(0.25), 10.);
        assert_approx_eq!(range.value_at(0.5), 5.);
        assert_approx_eq!(range.value_at(0.75), 0.);
        assert_approx_eq!(range.value_at(1.), 5.);
    }

/*    #[test]
    fn range_on_an_envelope() {
        let envelope = Envelope::ar(1., 1., 0.);

        let range = envelope.range(-5., 5.);

        assert_approx_eq!(range.value_at(0.), -5.);
        assert_approx_eq!(range.value_at(1.), 5.);
        assert_approx_eq!(range.value_at(2.), -5.);
    }*/

}