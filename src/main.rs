
/// A program that checks if the number is odd or even
fn main() {
    let mut args = std::env::args();
    if args.len() > 2 {
        eprintln!("\x1b[1m\x1b[31mFatal. \x1b[39mToo many arguments.");
        eprintln!("USAGE: is-odd [number]\x1b[0m");
        std::process::exit(1);
    }
    if args.len() < 2 {
        eprintln!("\x1b[1m\x1b[31mFatal. \x1b[39mToo few arguments.");
        eprintln!("USAGE: is-odd [number]\x1b[0m");
        std::process::exit(1);
    }
    let arg = &args.nth(1).unwrap();
    let odd = is_odd(arg.parse::<u128>().unwrap());
    let str = if odd { "odd" } else { "even" };
    println!("{} is {}", arg, str);
}

fn is_odd(num: u128) -> bool {
    num % 2 == 1
}