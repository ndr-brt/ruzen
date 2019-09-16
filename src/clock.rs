pub type Hz = f64;

#[derive(Clone,Copy)]
pub(crate) struct Clock {
    sample_rate: Hz,
    clock: f64,
}

impl Clock {

    pub fn new(sample_rate: Hz) -> Clock {
        Clock{ sample_rate, clock: 0. }
    }

    pub fn tick(&mut self) -> f64 {
        self.clock += 1.0;
        self.get()
    }

    pub fn get(&self) -> f64 {
        self.clock / self.sample_rate
    }

}