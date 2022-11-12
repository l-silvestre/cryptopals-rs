mod exercises;
mod helpers;
use std::io;

fn main() {

  loop {
    println!("Please Choose the exercise to run: (1 to 10)");
    println!("type '0' to exit");
    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("Choice: {guess}");

    match guess {
      1 => exercises::ex1(),
      2 => exercises::ex2(),
      3 => exercises::ex3(),
      4 => exercises::ex4(),
      5 => exercises::ex5(),
      6 => exercises::ex6(),
      7 => exercises::ex7(),
      8 => exercises::ex8(),
      9 => exercises::ex9(),
      10 => exercises::ex10(),
      0 => break,
      _ => println!("exercise not solved yet, pick a number between 1 and 10"),
    }
  }
}
