use rand::distributions::{Uniform, Exp, Alphanumeric};
use std::collections::HashMap;
use rand::prelude::*;

#[derive(Default)]
pub struct UserData {
    name: String,
    username: String,
    age: String,
    email: String,
    bio: String,
}

use super::*;

#[test]
pub fn evo_algo(gens: i32, rate: f32, verbose: bool) {
    let start = "Hello, world!".to_string();
    let targ ="Goodbye all!!".to_string();
    let evo = Evol::new()
        .with_data(start)
        .with_rate(rate)
        .with_target(targ);
    evo.evolve(gens, verbose);
}

#[derive(Default, Clone)]
pub struct Evol {
    pub gen: i32,
    pub mut_rate: f32,
    pub data: String,
    pub target: String,
    pub rng: ThreadRng,
    pub str_hist: Vec<String>,
    pub fit_hist: Vec<f32>,
    pub mut_hist: Vec<HashMap<i32, char>>,
}

impl Evol {

    pub fn new() -> Self { 
        Self { 
            gen: 0, mut_rate: 0.05, rng: thread_rng(), ..Self::default() } }

    pub fn with_data(mut self, data: String) -> Self { 
        let mut str_hist = Vec::new();
        str_hist.push(data.clone());
        self.data = data; 
        self 
    }

    pub fn evolve(mut self, gens: i32, verbose: bool) {
        let mut last = self.clone();
        let mut next = last.clone();
        for i in 1..100 {
            last = next.clone();
            next = last.step();
            if next.get_fitness() < last.get_fitness() {
                next = last;
            } else if next.get_fitness() > last.get_fitness() {
                // get rng seed or something, make dist metric based on bits, etc.
            }
            //println!("{} {}", last.data, last.target);
            //self = last;
        }
    }

    pub fn with_rate(mut self, rate: f32) -> Self { self.mut_rate = rate; self }

    pub fn with_target(mut self, target: String) -> Self { self.target = target; self }

    pub fn step(&mut self) -> Self {
        let mut str_hist = self.clone().str_hist;
        let mut fit_hist = self.clone().fit_hist;
        let mut mut_hist = self.clone().mut_hist;
        let (new_str, mutation) 
            = self.mutate(Some(self.str_hist.last().unwrap_or(&self.data).to_owned()));
        let new_fit = self.get_fitness();
        str_hist.push(new_str);
        fit_hist.push(new_fit);
        mut_hist.push(mutation);
        match self.str_hist.last() {
            Some(string) => Self { data: string.to_owned(), gen: self.gen + 1, str_hist, fit_hist, mut_hist, ..self.to_owned() },
            None => Self { data: self.mutate(Some(self.data.to_owned())).0, gen: self.gen + 1, str_hist, fit_hist, mut_hist, ..self.to_owned() }
        }
        //Self { gen: self.gen+1, ..last };
    }

    fn mutate(&mut self, string: Option<String>) -> (String, HashMap<i32, char>) {
        let mut rng = thread_rng();
        let mut mutations: HashMap<i32, char> = HashMap::new();
        let last = match string {
            Some(string) => string,
            None => self.clone().data,
        };
        let out: String = last.chars().into_iter().enumerate().map(|(i, chr)|
            if rng.gen_bool(self.mut_rate as f64) {
                mutations.insert(i as i32, chr);
                self.clone().rand_string(1).remove(0)
            } else { chr }
        ).collect();

        println!("From mutate:{}, fitness: {}", out, self.get_fitness());
        (out, mutations)
    }

    fn gen_target(&mut self, len: usize) -> Self { 
        self.clone().target = self.clone().rand_string(len); 
        self.to_owned() 
    }

    fn gen_data(&mut self, len: usize) -> Self { 
        self.data = self.clone().rand_string(len); 
        let mut str_hist = Vec::new();
        str_hist.push(self.clone().data);
        self.to_owned()
    }

    fn rand_string(mut self, len: usize) -> String {
        let mut out = String::with_capacity(len);
        let rn: Vec<char> = self.rng.sample_iter(Alphanumeric).take(len).collect();
        out.extend(rn);
        out
    }

    fn get_fitness(&self) -> f32 {
        let fit = self.data.chars().zip(self.target.chars())
            .filter(|&(c1, c2)| c1.eq(&c2)).count() as f32;
        fit
    }

    fn push_fitness(&self) -> Self {
        let mut fit_hist = self.clone().fit_hist;
        let fitness = self.get_fitness();
        fit_hist.push(fitness);
        Self { fit_hist, ..self.to_owned() }
    }
}



