fn main() {
    let message = get_welcome();
    print_welcome(message);
    primitive_aside();
}

fn print_welcome(text: &str) {
    println!("{}", text);
}

fn get_welcome() -> &'static str{
    let weclome = "hi";
    weclome
}



fn primitive_aside() {
    let custom_num = 98_000;
    let hex_num = 0xfa;
    let bin_num = 0b0010_1011;
    let byte_num = b'A';
    println!("{}", custom_num);
    println!("{}", hex_num);
    println!("{}", bin_num);
    println!("{}", byte_num);
}