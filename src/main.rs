use bstr::{BStr, ByteSlice};
use memmap::MmapOptions;
use rustc_hash::FxHashMap as HashMap;
use std::{fmt::Display, fs::File};

use rayon::prelude::*;

#[derive(Debug)]
struct State {
    min: f64,
    max: f64,
    count: u64,
    sum: f64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            min: f64::MAX,
            max: f64::MIN,
            count: 0,
            sum: 0.0,
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let avg = self.sum / (self.count as f64);
        write!(f, "{:.1}/{avg:.1}/{:.1}", self.min, self.max)
    }
}

impl State {
    fn update(&mut self, v: f64) {
        self.min = self.min.min(v);
        self.max = self.max.max(v);
        self.count += 1;
        self.sum += v;
    }

    fn merge(&mut self, other: &Self) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self.count += other.count;
        self.sum += other.sum;
    }
}

fn make_map<'a>(i: impl Iterator<Item = &'a [u8]>) -> HashMap<&'a BStr, State> {
    let mut state: HashMap<&'a BStr, State> = Default::default();
    for line in i {
        let (name, value) = line.split_once_str(&[b';']).unwrap();
        let value = fast_float::parse(value).unwrap();
        state.entry(name.into()).or_default().update(value);
    }
    state
}

fn solve_for_part((start, end): (usize, usize), mem: &[u8]) -> HashMap<&BStr, State> {
    make_map((&mem[start..end]).lines())
}

fn merge<'a>(a: &mut HashMap<&'a BStr, State>, b: &HashMap<&'a BStr, State>) {
    for (k, v) in b {
        a.entry(k).or_default().merge(v);
    }
}

fn main() {
    let cores: usize = std::thread::available_parallelism().unwrap().into();
    let path = match std::env::args().skip(1).next() {
        Some(path) => path,
        None => "measurements.txt".to_owned(),
    };
    let file = File::open(path).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };

    let chunk_size = mmap.len() / cores;
    let mut chunks: Vec<(usize, usize)> = vec![];
    let mut start = 0;
    for _ in 0..cores {
        let end = (start + chunk_size).min(mmap.len());
        let next_new_line = match memchr::memchr(b'\n', &mmap[end..]) {
            Some(v) => v,
            None => {
                assert_eq!(end, mmap.len());
                0
            }
        };
        let end = end + next_new_line;
        chunks.push((start, end));
        start = end + 1;
    }
    let parts: Vec<_> = chunks
        .par_iter()
        .map(|r| solve_for_part(*r, &mmap))
        .collect();

    let state: HashMap<&BStr, State> = parts.into_iter().fold(Default::default(), |mut a, b| {
        merge(&mut a, &b);
        a
    });

    let mut all: Vec<_> = state.into_iter().collect();
    all.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    print!("{{");
    for (i, (name, state)) in all.into_iter().enumerate() {
        if i == 0 {
            print!("{name}={state}");
        } else {
            print!(", {name}={state}");
        }
    }
    println!("}}");
}
