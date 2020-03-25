use crate::state::ugen::ValueAt;
use rand::Rng;

pub struct WhiteNoise { }

impl WhiteNoise {
    pub fn default() -> Self {
        WhiteNoise { }
    }
}

impl ValueAt for WhiteNoise {
    fn value_at(&self, _clock: f64) -> f64 {
        rand::thread_rng().gen_range(-1., 1.)
    }
}