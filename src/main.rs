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
    dead: bool,
    #[clap(short = 'f', long = "file")]
    inputfile: Option<std::path::PathBuf>
}


impl Message {
    fn print_cat(&self) {
        if self.message.to_lowercase() == "woof" {
                eprintln!("A cat shouldn't bark like a dog.")
            }
        
          else {
                let eye = if self.dead {"x"} else {"o"};
                match &self.inputfile {
                    Some(path) => {
                        let cat_template = std::fs::read_to_string(path)
                        .expect(&format!("error reading the file at {:?}", path));

                        let eye = format!("{}", eye.red().bold());
                        let cat_picture = cat_template.replace("eye",
                        &eye);
                        println!(
                        "{}",
                        self.message.bright_yellow().underline().on_purple()
                        );
                        println!("{}", &cat_picture);
                    }

                    None => {
                        println!("{:?}", self.message);
                        println!(" \\");
                        println!("
                        /\\_/\\");
                        println!("
                        ( {eye} {eye} )", eye=eye.red().bold());
                        println!("
                        =( I )=");
                    }
            }
        }
    }

}
