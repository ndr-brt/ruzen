use std::ops::{Add, Mul};

pub mod generator;
pub mod envelope;

pub trait ValueAt {
    fn value_at(&self, clock: f64) -> f64;
}

pub struct UGen<T> where T: ValueAt {
    parameters: T,
    range: Range,
}

pub struct Range {
    low: f64,
    high: f64,
}

impl Range {
    fn pp_amplitude(&self) -> f64 {
        self.high - self.low
    }

    fn is_bipolar(&self) -> bool {
        (self.low < 0. && self.high >= 0.) || (self.low >= 0. && self.high < 0.)
    }
}

pub trait SignalRange<T> where T: ValueAt {
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
            range: Range { low: -1., high: 1.} // TODO: not correct!
        }
    }
}

impl<T: 'static, O: 'static> Mul<UGen<O>> for UGen<T> where T: ValueAt, O: ValueAt {
    type Output = UGen<Multiplied<T, O>>;

    fn mul(self, other: UGen<O>) -> Self::Output {
        UGen {
            parameters: Multiplied { first: self, second: other },
            range: Range { low: -1., high: 1.} // TODO: not correct!
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
            range: Range { low: value, high: value }
        }
    }
}

pub struct Ranged<T> where T: ValueAt {
    signal: UGen<T>,
    range: Range,
}

impl<T: 'static> SignalRange<UGen<T>> for UGen<T> where T: ValueAt {
    type Output = UGen<Ranged<T>>;

    fn range(self, low: f64, high: f64) -> Self::Output {
        UGen {
            parameters: Ranged { signal: self, range: Range { low, high } },
            range: Range { low, high },
        }
    }
}

impl<T> ValueAt for Ranged<T> where T: ValueAt {
    fn value_at(&self, clock: f64) -> f64 {
        let (ratio, offset) = if self.signal.range.is_bipolar() {
            let ratio = self.range.pp_amplitude() * 0.5;
            (ratio, ratio + self.range.low)
        } else {
            (self.range.pp_amplitude(), self.range.low)
        };

        (self.signal.value_at(clock) * ratio) + offset
    }
}

#[cfg(test)]
mod tests {
    use crate::ugen::{ValueAt, SignalRange, UGen};
    use crate::ugen::generator::{Generator};
    use crate::ugen::envelope::Envelope;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn range_on_a_generator() {
        let sine = Generator::sine().frequency(UGen::from(1.));

        let range = sine.range(0., 10.);

        assert_approx_eq!(range.value_at(0.), 5.);
        assert_approx_eq!(range.value_at(0.25), 10.);
        assert_approx_eq!(range.value_at(0.5), 5.);
        assert_approx_eq!(range.value_at(0.75), 0.);
        assert_approx_eq!(range.value_at(1.), 5.);
    }

    #[test]
    fn range_on_an_envelope() {
        let envelope = Envelope::ar(1., 1., 0.);

        let range = envelope.range(-5., 5.);

        assert_approx_eq!(range.value_at(0.), -5.);
        assert_approx_eq!(range.value_at(1.), 5.);
        assert_approx_eq!(range.value_at(2.), -5.);
    }

    #[test]
    fn multiply_an_ugen() {
        let envelope = Envelope::ar(1., 1., 0.);

        let multed = envelope * UGen::from(2.);

        assert_approx_eq!(multed.value_at(0.), 0.);
        assert_approx_eq!(multed.value_at(1.), 2.);
        assert_approx_eq!(multed.value_at(2.), 0.);
    }

}