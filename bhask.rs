use std::{f32}; // to use float

fn main(){

println!("type A value (?x^2) : ");
let mut input = String::new();//                         string to recieve input;
std::io::stdin().read_line(&mut input).unwrap();//       input
let a: f32 = input.trim().parse().unwrap();//            input to float32

println!("type B value (?x) : ");
let mut input = String::new();
std::io::stdin().read_line(&mut input).unwrap();
let b : f32 = input.trim().parse().unwrap();

println!("type C value (?) :");
let mut input = String::new();
std::io::stdin().read_line(&mut input).unwrap();
let c : f32 = input.trim().parse().unwrap();

let delta : f32 = (b.powf(2f32)) - (4f32*a*c);

let x1 : f32 = ((b*-1f32) + (delta.sqrt())) / (2f32*a);
let x2 : f32 = ((b*-1f32) - (delta.sqrt())) / (2f32*a);

println!("\n-------\ndelta = {}\nx1 = {}\nx2 = {} ",delta,x1,x2);

let xv : f32 = (b*-1f32) / (2f32*a);
let yv : f32 = (delta*-1f32) / (4f32*a);

println!("max/min point : {} , {} ",xv,yv);

}