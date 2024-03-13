use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;
use std::collections::BTreeMap;

struct Stats {
    min: f64,
    max: f64,
    sum: f64,
    count: f64,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            sum: Default::default(),
            count: Default::default(),
        }
    }
}

fn main() {
    let mut data = BTreeMap::<String, Stats>::new();

    let path = "measurements.txt";
    let f = File::open(path).unwrap();
    let f = BufReader::new(f);

    for line in f.lines().flatten().take(1000000) {
        // Hamburg;21.4

        if let Some((city, temp)) = line.split_once(';') {
            let temp: f64 = temp.parse().unwrap();

            let city_data = data
                .entry(city.to_string())
                .or_default();

            city_data.min = temp.min(city_data.min);
            city_data.max = temp.max(city_data.max);
            city_data.sum += temp;
            city_data.count += 1.0;
        } else {
            continue;
        };
    }

    // let mut data = data.into_iter().collect::<Vec<_>>();
    // data.sort_unstable_by(|city_a, city_b| {
    //     (city_a.0).cmp(&(city_b.0))
    // });

    for (city, stats) in data.into_iter() {
        let avg = if stats.count == 0.0 {
            0.0
        } else {
            stats.sum / stats.count
        };

        println!("{city}: {}/{}/{avg}", stats.min, stats.max)
    }
}