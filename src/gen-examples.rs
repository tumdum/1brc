use std::collections::HashSet;
use std::env;

#[cfg(feature = "random")]
use rand::{distributions::Alphanumeric, Rng};

const MIN_VALUE: i32 = -999; // inclusive
const MAX_VALUE: i32 = 999; // inclusive
const MIN_CITY_NAME_LEN: usize = 1;
const MAX_CITY_NAME_LEN: usize = 32; // inclusive

fn get_city_name() -> String {
    // Sampling printable UTF8 characters would be overly complex. Therefore,
    // only ASCII characters are sampled.
    let city_name_len =
        rand::thread_rng().gen_range(MIN_CITY_NAME_LEN..=MAX_CITY_NAME_LEN);
    let city_name_bytes = (0..city_name_len)
        .map(|_| rand::thread_rng().sample(Alphanumeric))
        .collect::<Vec<_>>();
    let city_name = String::from_utf8(city_name_bytes)
        .expect("Sampling ASCII characters must be valid UTF8");
    // Not all UTF8 characters use 4 bytes, so we need to truncate the string
    // to the actual length of the city name.
    city_name.to_string().chars().take(city_name_len).collect()
}

fn get_cities(nof_cities: u32) -> Vec<String> {
    let mut cities = HashSet::new();
    for _ in 0..nof_cities {
        loop {
            let city_name = get_city_name();
            if city_name.contains(';') {
                continue;
            }
            if cities.insert(city_name) {
                break;
            }
        }
    }
    cities.into_iter().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let max_nof_cities: u32 = args[1].parse().unwrap();
    let nof_rows: u32 = args[2].parse().unwrap();

    let cities = get_cities(max_nof_cities);
    for _ in 0..nof_rows {
        let city_idx = rand::thread_rng().gen_range(0..cities.len());
        let value = rand::thread_rng().gen_range(MIN_VALUE..=MAX_VALUE);
        let value = value as f64 / 10.0;
        println!("{};{value:.1}", cities[city_idx]);
    }
}
