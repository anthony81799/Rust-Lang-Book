use std::{thread, time::Duration};
use std::collections::{HashMap, hash_map::Entry};
use std::hash::Hash;

fn _simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}

struct Cacher<T, K, V>
where
    T: Fn(K) -> V,
{
    calculation: T,
     value: HashMap<K, V>
}

impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Hash + Eq + Clone
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            value: HashMap::new()
        }
    }
    
    fn value(&mut self, arg: K) -> &V {
        match self.value.entry(arg.clone()) {
            Entry::Occupied(v) => v.into_mut(),
            Entry::Vacant(v) => v.insert((self.calculation)(arg))
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // closures are anonymous functions
    // types don't need type annotations
    // they can be determined by the compiler
    // will only work with type of first type passed in
    // closures can get information from thier environment
    // The traits that control this are Fn, FnOnce, or FnMut
    // Functions also implement these traits
    // They are associated with principles of ownership
    // Taking ownership, immutable references, and mutable references
    let mut cached_result = 
    Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num   
    });
    if intensity < 25 {
        println!("Today do {} pushups!",
        cached_result.value(intensity)
        );

        println!("Next do {} situps!",
        cached_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated");
        } else {
            println!("Today run for {} minutes!",
            cached_result.value(intensity)
            );
        }
    }
}
