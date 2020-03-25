use std::collections::HashMap;
use rosc::OscType;
use crate::state::ugen::{UGen};

#[derive(Debug, Clone)]
pub struct Parameters {
    data: HashMap<String, UGen<f64>>
}

pub trait GetParameter {
    fn get(&self, key: &str, default: UGen<f64>) -> UGen<f64>;
}

impl From<Vec<OscType>> for Parameters {
    fn from(args: Vec<OscType>) -> Self {
        let param_list: Vec<String> = args.iter()
            .map(|t| t.to_owned())
            .map(|t| t.string())
            .map(|t| t.unwrap())
            .collect();

        let mut data = HashMap::<String, UGen<f64>>::new();
        let mut index = 0;
        while index < param_list.len() {
            let key: String = param_list.get(index).unwrap().clone();
            let value = param_list.get(index + 1).unwrap().clone();
            data.insert(key, UGen::from(value.parse::<f64>().unwrap()));
            index += 2;
        }

        Parameters { data }
    }
}

impl GetParameter for Parameters {
    fn get(&self, key: &str, default: UGen<f64>) -> UGen<f64> {
        match self.data.get(key) {
            Some(val) => *val,
            None => default
        }
    }
}