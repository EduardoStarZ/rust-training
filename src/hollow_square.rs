use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("cannot read line");
    let number : i32 = input.trim().parse::<i32>().unwrap();

    for x in 1..number+1{
        for y in 1..number+1{
            if x == 1 || x == number || y == 1 || y == number {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}