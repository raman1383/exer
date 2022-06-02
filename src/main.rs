extern crate rusty_machine as rm;

use std::process::ExitCode;

fn main() -> ExitCode {
    let x = 1;
    let x = x + 2;
    let x = x * 2;
    println!("Value of x: {}", x);

    ExitCode::SUCCESS
}
