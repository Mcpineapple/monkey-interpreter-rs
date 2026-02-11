mod lexer;
mod repl;
mod token;
use whoami;

fn main() {
    println!(
        "Hello {}! This is the Monkey programming language!\n",
        whoami::username().unwrap()
    );
    println!("Feel free to type commands\n");
    repl::start();
}
