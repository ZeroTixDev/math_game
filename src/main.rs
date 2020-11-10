use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::time::Instant;
const QUESTIONS: f64 = 10.0;
enum Operation {
    Add,
    Sub,
    Mult,
}

fn calculate_answer(number1: i32, number2: i32, operation: Operation) -> i32 {
    match operation {
        Operation::Add => number1 + number2,
        Operation::Sub => number1 - number2,
        Operation::Mult => number1 * number2,
    }
}
fn create_new_operation() -> Operation {
    let value = rand::thread_rng().gen_range(1, 4); // 1 -> 3
    match value {
        1 => Operation::Add,
        2 => Operation::Sub,
        3 => Operation::Mult,
        _ => Operation::Add,
    }
}
fn main() {
    let mut question_count: u32 = 0;
    let mut correct_answers: u32 = 0;
    let mut incorrect_answers: u32 = 0;
    let before = Instant::now();
    println!(
        "Hello, this is a math game. There is {} questions you have to finish. Good luck!",
        QUESTIONS
    );
    loop {
        let number1 = rand::thread_rng().gen_range(1, 14);
        let number2 = rand::thread_rng().gen_range(1, 14);
        let operation: Operation = create_new_operation();
        let operation_string = match operation {
            Operation::Add => String::from("+"),
            Operation::Sub => String::from("-"),
            Operation::Mult => String::from("*"),
        };
        let answer = calculate_answer(number1, number2, operation);
        println!("{}{}{}", number1, operation_string, number2);
        // println!("answer is {}", answer);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong!");
                question_count += 1;
                incorrect_answers += 1;
                if question_count >= QUESTIONS as u32 {
                    break;
                } else {
                    continue;
                }
            }
        };
        match guess.cmp(&answer) {
            Ordering::Less => {
                println!("Wrong!");
                question_count += 1;
                incorrect_answers += 1;
                if question_count >= QUESTIONS as u32 {
                    break;
                } else {
                    continue;
                }
            }
            Ordering::Greater => {
                println!("Wrong!");
                question_count += 1;
                incorrect_answers += 1;
                if question_count >= QUESTIONS as u32 {
                    break;
                } else {
                    continue;
                }
            }
            Ordering::Equal => {
                question_count += 1;
                correct_answers += 1;
                if question_count >= QUESTIONS as u32 {
                    break;
                } else {
                    continue;
                }
            }
        }
    }
    let accuracy: f64 = correct_answers as f64 / QUESTIONS * 100.0;
    println!(
        "Accuracy: {}/{} ({}%), Correct Answers: {}, Incorrect Answers: {}, Time: {:?} seconds",
        correct_answers,
        QUESTIONS,
        accuracy,
        correct_answers,
        incorrect_answers,
        before.elapsed().as_secs()
    )
}
