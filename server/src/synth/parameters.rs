use std::collections::HashMap;
use rosc::OscType;

#[derive(Debug)]
pub struct Parameters {
    data: HashMap<String, OscType>
}

impl From<Vec<OscType>> for Parameters {
    fn from(args: Vec<OscType>) -> Self {
        let param_list: Vec<String> = args.iter()
            .map(|t| t.to_owned())
            .map(|t| t.string())
            .map(|t| t.unwrap())
            .collect();

        let mut data = HashMap::<String, OscType>::new();
        let mut index = 0;
        while index < param_list.len() {
            let key: String = param_list.get(index).unwrap().clone();
            let value = param_list.get(index + 1).unwrap().clone();
            data.insert(key, OscType::Double(value.parse::<f64>().unwrap()));
            index += 2;
        }

        Parameters { data }
    }
}

impl Parameters {
    pub fn get(&self, key: &str) -> Option<&OscType> {
        self.data.get(key)
    }

}