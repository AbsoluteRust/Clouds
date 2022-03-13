use clearscreen::*;
use std::thread;

fn main() {
    loop { 
        swaprain();
    }

}

fn cloud() {
    println!("          .-~~~-.");
    println!("  .- ~ ~-(       )_ _");
    println!(" /                     ~ -.");
    println!(r#"|                           \"#);
    println!(r#" \                         .'"#);
    println!("   ~- . _____________ . -~");
}

fn raindrops01() {
    
    println!("   ~- . _____________ . -~");
    println!("      |   |   |   |   ");
    println!("        |   |   |   | ");
    println!("      |   |   |   |   ");
    println!("        |   |   |   | ");
    println!("      |   |   |   |   ");
    println!("        |   |   |   | ");
    println!("      |   |   |   |   ");
}

fn raindrops02() {
    
    println!("   ~- . _____________ . -~");
    println!("        |   |   |   | ");
    println!("      |   |   |   |   ");
    println!("        |   |   |   | ");
    println!("      |   |   |   |   ");
    println!("        |   |   |   | ");
    println!("      |   |   |   |   ");
    println!("        |   |   |   | ");
}

fn swaprain() {
    cloud();
    raindrops01();
    thread::sleep_ms(250);
    clearscreen::clear().expect("failed to clear screen");
    cloud();
    raindrops02();
    thread::sleep_ms(250);
    clearscreen::clear().expect("failed to clear screen");

}