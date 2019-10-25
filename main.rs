use std::io;
use std::io::prelude::*; 
use colored::*;

fn main() {

    control();

}

fn list_primes(n: u128) -> Vec<u128>{

    // generates list of n primes

    let mut primes: Vec<u128>=Vec::with_capacity(1000);
    let mut i: u128=2;

    while primes.len()<n as usize{
        if is_prime(i){
            primes.push(i);
        }
        i+=1;
    }
    
    return primes
}

fn check_prime(v: &Vec<u128>) -> bool {

    // Check if all values in prime factorisation are prime

    let mut out: bool=true;
    for val in v.iter(){
        out&=is_prime(*val);
    }

    return out;
}

fn get_command(command: &mut String){

    // Get command from user

    print!(">> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(command).expect("Read line failed");
}

fn mul_vec(v: &Vec<u128>) -> u128{

    // multiplies elements of vector together.
    // used to compute number from factorisation

    let mut out: u128=1;

    for val in v.iter(){
        out*=val;
    }

    return out;
}

fn help() {
    println!("{}","  Prime Factor Calculator".green().bold());
    println!("{}","  Commands:".green().bold());
    println!("{}","      quit           : exit calculator".green().bold());
    println!("{}","      <unsigned int> : factorises number".green().bold());
    println!("{}","      check          : checks factorisation was correct".green().bold());
}

fn print_vec(out: &Vec<u128>){
    // Just prints out elems of vector
    for val in out.iter(){
        print!("{}",format!("{} ",val).green().bold());
    }
    println!("");
}

fn control(){

    // main control loop

    let mut command=String::new();
    let mut n: u128;
    let mut out:Vec<u128>=Vec::new();
    let n_primes: u128=1000;
    let primes: Vec<u128>=list_primes(n_primes);

    loop {
        command.clear();
        get_command(&mut command);

        if command.is_empty() {continue}

        if command.trim()=="quit" {break}

        if command.trim()=="check" {
            if out.is_empty(){
                println!("{}","    Factorise a number first".yellow().bold())
            } else {
                println!("    {}",format!("Product of factors: {}",mul_vec(&out)).green().bold());
                if check_prime(&out){
                    println!("{}","    All factors prime".green().bold());
                } else {
                    println!("{}","    NOT ALL FACTORS PRIME".red().bold());
                }
            }
            continue;
        }

        if command.trim()=="help" {
            help();
            continue;
        }

        if command.trim().parse::<u128>().is_ok(){
            n=command.trim().parse().expect("    Not a Number");
        } else {
            println!("{}","    Command not recognised".red().bold());
            continue;
        }

        if factorise(n,&primes,&mut out){
            if out.len()==1{
                println!("{}",format!("    {} is prime",n).green().bold());
            } else {
                print!("{}",format!("    Factorisation of {}: ",n).green().bold());
                print_vec(&out);
            }
        } else {
            println!("{}",format!("    Failed to completely factorise {}",n).red().bold());
        }
    }
}

fn factorise(mut n: u128, primes: &Vec<u128>, out: &mut Vec<u128>) -> bool{
    
    // Last prime does not have to be in list of primes. 
    // Freaked me out when it first realised.

    if n<2 {return false}

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