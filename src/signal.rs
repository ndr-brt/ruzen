use std::f64::consts::PI;

trait Signal {
    fn value_at(&self, clock: f64) -> f64;
}

struct Sine {
    frequency: f64,
    phase: f64
}
impl Signal for Sine {
    fn value_at(&self, clock: f64) -> f64 {
        ((clock + self.phase) * self.frequency * 2.0 * PI).sin()
    }
}

struct Saw {
    frequency: f64,
    phase: f64
}
impl Signal for Saw {
    fn value_at(&self, clock: f64) -> f64 {
        (((clock + self.phase) * self.frequency) % 1.)
    }
}