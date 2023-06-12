use core::{find_pattern_start, parse_signature};

fn main() {
    let signature = "FF??D0??FF";
    let pattern = parse_signature(signature);

    println!("{:?}", pattern);

    let data: Vec<u8> = vec![0xFF, 0x12, 0xD0, 0x34, 0xFF];

    println!("{:?}", find_pattern_start(&data, &pattern));
    println!("Hello, world");
}
