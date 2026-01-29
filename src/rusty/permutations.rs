#![allow(unused)]
use std::io;

fn print_beautiful_permutation(n:usize) {
    if n==2 || n==3 {
        println!("NO SOLUTION");
        return;
    }
    if n==1 {
        println!("1");
        return;
    }
    //Even
    for i in (2..=n).step_by(2) {
     print!("{} ",i);
    }
    //Odd
    for i in (1..=n).step_by(2){
        print!("{} ",i);
    }
    println!();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n:usize = input.trim().parse().unwrap();
    print_beautiful_permutation(n);
}