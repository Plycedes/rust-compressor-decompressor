use std::io;
mod comp;
mod decom;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error");

    match input.trim() {
        "comp" => comp::compress(),
        "decom" => decom::decomp(),
        _ => println!("Invalid Input"),
    }
}