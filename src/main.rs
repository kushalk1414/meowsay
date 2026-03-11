// use std::env::args;
use::colored::Colorize;
use clap::Parser;


fn main() {
    let message_struct = Message::parse();
    message_struct.print_cat();
}

#[derive(Parser)]
struct Message {
    #[clap(default_value = "Meow!")]
    message: String,
    #[clap(short = 'd', long = "dead")]
    dead: bool
}


impl Message {
    fn print_cat(&self) {
    
        let eye = if self.dead {"x"} else {"o"};
    
        println!("{:?}", self.message);
        println!(" \\");
        println!("
        /\\_/\\");
        println!("
        ( {eye} {eye} )", eye=eye.red().bold());
        println!("
        =( I )=");
    }

//     fn new(str: String) -> Message{ 
//         Message {
//         message: str
//     }}
}
