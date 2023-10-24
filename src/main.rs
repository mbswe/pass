use cli_clipboard::{ClipboardContext, ClipboardProvider};
use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;

#[derive(Debug, Subcommand)]
enum Commands {
    #[command()]
    Pass(PassArgs)
}

#[derive(Parser, Debug)]
struct PassArgs {
    #[arg()]
    length: u8,

    #[clap(short, long, default_value_t = false)]
    special_characters: bool,
}

fn main() {
    let args = PassArgs::parse();

    let password = generate_password(args.length.into(), args.special_characters);
    
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(password.clone()).unwrap();
    
    println!("Generated password: {} and copied it to your clipboard.", password);
}

fn generate_password(length: usize, special_characters: bool) -> String {
    let mut charset = "abcdefghijklmnopqrstuvwxyz\
                       ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                       0123456789"
                       .chars()
                       .collect::<Vec<char>>();
    
    if special_characters {
        charset.extend("!@#$%^&*()[]{}:;<>,.?~".chars());
    }

    let password: String = (0..length)
        .map(|_| charset.choose(&mut rand::thread_rng()).unwrap().to_owned())
        .collect();

    password
}
