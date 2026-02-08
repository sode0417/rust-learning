fn main() {
    for i in 1..101 {
        println!("{}", fizzbuzz(i));
    }
}
fn fizzbuzz(n: u32) -> String {
    let result =
        if n.is_multiple_of(15) {"FizzBuzz"}
        else if n.is_multiple_of(3) {"Fizz"}
        else if n.is_multiple_of(5) {"Buzz"}
        else {&n.to_string()};
    result.to_string()
}
