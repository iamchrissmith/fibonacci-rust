use std::io;

fn fibonacci(n: i64) -> i64 {
    if n < 2 {
        return 1;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn get_user_input(question: &str) -> String {
    println!("{}", question);

    let mut answer = String::new();

    io::stdin().read_line(&mut answer)
        .expect("Failed to read line");

    answer
}


fn main() {
    let question = "Which fibonacci number would you like?";

    let number = get_user_input(question);

    let number: i64 = number.trim().parse().unwrap();

    println!("You number is {}", fibonacci(number));
}
