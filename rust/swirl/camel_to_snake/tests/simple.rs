use camel_to_snake::camel_to_snake;


pub fn main() {
    let camel_to_snake!(oneTwoThree): u8 = 123;
    println!("one_two_three: {}", one_two_three);
}
