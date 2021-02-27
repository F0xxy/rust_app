use core::f32;

use rand::prelude::*;

fn main() {
    let rdn_boolean: bool = random::<bool>();
    let _lol: Vec<i32> = vec![1, 2, 3, 4];
    let pi: f32 = std::f32::consts::PI;
    println!("hola mundo, {},{},{}", rdn_boolean, pi, rdn_boolean);
}
