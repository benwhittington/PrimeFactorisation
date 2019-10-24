use std::io;
use std::io::prelude::*; 

fn main() {

    control();

}

fn list_primes(n: u128) -> Vec<u128>{

    let mut primes: Vec<u128>=Vec::new();

    for i in 2..n {
        if is_prime(i){
            primes.push(i);
        }
    }
    
    return primes
}

fn check_prime(v: &Vec<u128>) -> bool {
    let mut out: bool=true;
    for val in v.iter(){
        out&=is_prime(*val);
    }

    return out;
}

fn get_command(command: &mut String){

    print!(">> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(command).expect("Read line failed");
}

fn mul_vec(v: &Vec<u128>) -> u128{
    let mut out: u128=1;

    for val in v.iter(){
        out*=val;
    }

    return out;
}

fn print_vec(out: &Vec<u128>){
    for val in out.iter(){
        print!("{} ",val);
    }
    println!("");
}

fn control(){
    let mut command=String::new();
    let mut n: u128;
    let mut out:Vec<u128>=[1].to_vec();
    let n_primes: u128=1000;
    let primes: Vec<u128>=list_primes(n_primes);

    loop {
        command.clear();
        get_command(&mut command);

        if command.is_empty() {continue}

        if command.trim()=="quit" {break}

        if command.trim()=="check" {
            println!("{}",mul_vec(&out));
            if check_prime(&out){
                println!("All factors prime")
            } else {
                println!("NOT ALL FACTORS PRIME");
            }
            continue;
        }

        n=command.trim().parse().expect("Not a Number");

        if factorise(n,&primes,&mut out){
            print!("Factorisation of {}: ",n);
            print_vec(&out);
        } else {
            println!("Failed to completely factorise {}",n);
        }



    }
}

fn factorise(mut n: u128, primes: &Vec<u128>, out: &mut Vec<u128>) -> bool{
    
    // last prime does not have to be in list of primes

    let mut n_root: u128=(n as f64).sqrt() as u128+1;
    let mut ok: bool;
    out.clear();
    while !is_prime_root(n, n_root) {
        ok=false;
        for prime in primes {
            if n%prime==0{
                out.push(*prime);
                n/=prime;
                n_root=(n as f64).sqrt() as u128+1;
                ok=true;
                break;
            }
        }
        if !ok {return false}
    }

    out.push(n);
    return is_prime_root(n, n_root);
}

fn is_prime_root(n: u128, n_root: u128) -> bool {

    if n==2 {return true}
    for i in 2..(n_root) {
        if n%i==0 {return false};
    }

    return true

}

fn is_prime(n: u128) -> bool {
    let n_root: u128=(n as f64).sqrt() as u128+1;
    if n==2 {return true}
    for i in 2..(n_root) {
        if n%i==0 {return false};
    }

    return true
}