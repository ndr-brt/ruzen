use crate::synth::ugen::{UGen, ValueAt, Range};

pub trait Envelope: ValueAt {
    fn duration(&self) -> f64;
}

impl dyn Envelope {
    pub fn ar(attack: f64, release: f64, curve: f64) -> UGen<AR> {
        UGen {
            signal: AR { attack, release, curve },
            range: Range { low: 0., high: 1. }
        }
    }

    pub fn line(start: f64, end: f64, duration: f64) -> UGen<Line> {
        UGen {
            signal: Line { start, end, duration },
            range: Range { low: start, high: end }
        }
    }
}

pub struct AR {
    attack: f64,
    release: f64,
    curve: f64,
}

impl Envelope for UGen<AR> {
    fn duration(&self) -> f64 {
        self.signal.attack + self.signal.release
    }
}

impl ValueAt for AR {
    fn value_at(&self, clock: f64) -> f64 {
        if clock <= self.attack {
            let x = clock / self.attack;
            if self.curve >= 0. { x.powf(self.curve + 1.) } else { x.powf(-1. / (self.curve - 1.)) }
        } else if clock <= self.attack + self.release {
            let x = (clock - self.attack) / self.release;
            if self.curve >= 0. { 1. - x.powf(self.curve + 1.) } else { 1. - x.powf(-1. / (self.curve - 1.)) }
        } else {
            0.
        }
    }
}

pub struct Line {
    start: f64,
    end: f64,
    duration: f64
}

impl Envelope for UGen<Line> {
    fn duration(&self) -> f64 {
        self.signal.duration
    }
}

impl ValueAt for Line {
    fn value_at(&self, clock: f64) -> f64 {
        (self.start + (clock * (self.end - self.start) / self.duration))
    }
}


#[cfg(test)]
mod tests {
    use crate::synth::ugen::{ValueAt };
    use crate::synth::ugen::envelope::Envelope;

    #[test]
    fn ar_envelope() {
        let envelope = Envelope::ar(1.0, 1.0, 0.);
        assert_eq!(0.0, envelope.value_at(0.0));
        assert_eq!(0.5, envelope.value_at(0.5));
        assert_eq!(1.0, envelope.value_at(1.0));
        assert_eq!(0.5, envelope.value_at(1.5));
        assert_eq!(0.0, envelope.value_at(2.0));
    }

}