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

pub fn string_memory() -> String {
   let temp = String::from("a string put a ptr, capacity, and lenght on the stack. the value is on the heap");
   // moving non-primitive type causes the a new stack frame to be allocated for the 
   // content. the old stack content, in this case temp, is no longer valid and cannot be addressed
   // Rust's mv'ing semantics prevents double freeing. That is, allowing cp'ing of ptrs on the stack, which is the representation of
   // of a complex type, leads to multiple stack frames referencing the same heap location.
   // Mv'ing have strong ownership properties that map from the stack to the heap, makind the double free problem go away.
   let output = temp;
   output
}


fn extend_string(mut s: String) -> String {
    s.push_str(" and more...");
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_welcome_works() {
        let result = get_welcome();
        assert_eq!(result, "hi");
    }

    #[test]
    fn return_mved_value() {
        let msg= String::from("Hello");
        // we can get the value of an immutable var 
        // 1. by shadowing
        let msg = extend_string(msg);
        assert_eq!(msg, "Hello and more...");
        // 2. with a new variable
        let orig = String::from("Hello");
        let updated = extend_string(orig);
        assert_eq!(updated, "Hello and more...");
        // or by using mutables in the first place
        let mut mutant  = String::from("Hello");
        mutant = extend_string(mutant);
        assert_eq!(mutant, "Hello and more...");

    }

    #[test]
    fn mutable_borrow() {
        let mut msg =String::from("hello");
        // mutable_borrow intended to mut the value 'hello'
        // need to declare it as a mutable reference
        let mutable_borrower = &mut msg;
        mutable_borrower.push_str(" my friend");
        assert_eq!(mutable_borrower,"hello my friend");
        assert_eq!(msg, "hello my friend");

    }
}
