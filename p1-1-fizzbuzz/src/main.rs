fn main() {
    for i in 1..101 {
        println!("{}", fizzbuzz(i));
    }
}
fn fizzbuzz(n: u32) -> String {
    let result: String = if n.is_multiple_of(15) {
        "FizzBuzz".to_string()
    } else if n.is_multiple_of(3) {
        "Fizz".to_string()
    } else if n.is_multiple_of(5) {
        "Buzz".to_string()
    } else {
        n.to_string()
    };
    result
}
