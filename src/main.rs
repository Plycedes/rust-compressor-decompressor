use std::io;
mod comp;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");

    match input.trim() {
        "comp" => comp::compress(),
        _ => println!("Invalid Input"),
    }
}