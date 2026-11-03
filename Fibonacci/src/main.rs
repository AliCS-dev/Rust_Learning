use std::io;
fn main() {
   println!("Enter number:");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    
    let n: u32 = n.trim().parse().expect("Please type a number!");
    let result = fibonacci(n);
    println!("{}", result);
}
fn fibonacci(n: u32) -> u32 {
    if n==0{return 0;}
    if n==1{return 1;}
    fibonacci(n - 1) + fibonacci(n - 2)
}


