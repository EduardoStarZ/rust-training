use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("cannot read line");
    let number : i32 = input.trim().parse::<i32>().unwrap();

    for _ in 1..number+1 {
        for _ in 1..number+1 {
            print!("*");
        }
        println!("");
    }
}