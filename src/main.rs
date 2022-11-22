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

    #[test]
    fn reference_rules() {
        // the rules of references are 
        //1. At any time either
        //   ONE mutable reference ,or  
        //   ANY number of immutable
        // This enables one writer, many readers and no contention amongst them
        //2. References must be valid

        let mut input = String::from("this may be mutated");
        let reader_1 = &input;
        let reader_2= &input;
        let reader_3= reader_1;

        // any number of readers are ok
        assert_eq!(input.as_str(), reader_1.as_str());
        assert_eq!(input, *reader_2);
        assert_eq!(input,*reader_3);

        // since input is mutable, we can define a mutable reference, as long
        // as there are no readers in scope
        // the reader are out of scope after the asserts above

        let mutation = String::from(", and so it was");
        let expected =   input.clone() + mutation.as_str();
        let changes_value =&mut input;
        changes_value.push_str(&mutation);
        assert_eq!(*changes_value, expected);

        // attempting to have a reader(an immutable reference) to input with not 
        // compile. in the context of this single threaded program the reason it is not obvious 
        // but if you image async/ multithreaded, then it's clear the this pattern, in general
        // could cause data races and unprectability results for the reader

        // Error: cannot borrow input as immutable because borrowed as mutable
        /* 
        let violates_rule_1_mixing_mut_and_immut = &input;
        assert_eq!(*changes_value, expected);
*/

        // similarly multiple writers would lead to unpredictable results in async context
        // Error: cannot borrow input as mutable more than once at a time
        /* 
        let too_many_writers = &mut input;
        assert_eq!(*changes_value, *too_many_writers);
        */
    }

    #[test]
    fn string_slice() {
        let s = String::from("hello");
        let slc = &s[2..4];

        assert_eq!(slc.len(), 2);

        let inclusive = &s[2..=4];
        assert_eq!(inclusive.len(), 3);

        // like above, we can move `s` as long as a
        // borrow is still in scope
        fn take(v: String) {println!("{}",v)}
        /* will fail to compile b/c of borrow scoping 
        take(s);    
        take(slc.to_string());
        */
        // this is ok b/c borrowers are out of scope before moving
        // the original variable
        take(slc.to_string());
        take(s);
    }

    enum PersonalIdentifier {
        Passport(String),
        DriverLicense(u32,u32)
    }

    struct Person {
        first: String,
        last: String,
        age: u32,
        id: PersonalIdentifier
    }


    impl Person {
        fn full_name(&self) -> String{
           format!("{} {}", self.first, self.last)
        }

        fn update_age(&mut self, new_age: u32) {
            self.age = new_age
        }

        fn new() -> Person {
            Person { 
                first: "".to_string(), 
                last: "".to_string(), 
                age: 0,
                id: PersonalIdentifier::Passport("P11234".to_string())
             }
        }

        fn from(first: String, last: String, age: u32, id: PersonalIdentifier) -> Person {
            Person { first: first, last: last, age: age, id: id}
        }
    }

    fn check_id(id: PersonalIdentifier) {
        let result = match id {
            PersonalIdentifier::Passport(x) => {
                println!("passport id: {}", x);
                x
            },
            PersonalIdentifier::DriverLicense(y, z) => {
                println!("drivers license: {}.{}", y,z);
                y
            }
        };
        println!("got result {}", result)
    }
    #[test]
    fn structs() {

        let mut p = Person{
            first:"john".to_string(),
            last:String::from("doe"),
            age: 100,
            id: PersonalIdentifier::DriverLicense(21,47)
        };

        assert_eq!(p.age, 100);
     
        assert_eq!(p.full_name(), "john doe");
        // p must be declare as mut, of course, to apply this method
        // it is interesting that the methods are scoped to the declaration
        // of the instance (mutable vs immutable)
        p.update_age(100);
        assert_eq!( p.age, 100);

        let new_born = Person::new();
        assert_eq!(new_born.age, 0);

        let p2 = Person::from(p.first, p.last, 42,  PersonalIdentifier::DriverLicense(37, 65));
       
        assert_eq!(p2.age, 42);
        check_id(p2.id)
    }
}
