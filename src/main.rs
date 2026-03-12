// use std::env::args;
use::colored::Colorize;
use clap::{Parser};
use anyhow::{Context, Result};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let message_struct = Message::parse();
    message_struct.print_cat()?;
    Ok(())

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
    fn print_cat(&self) -> Result<String, Box<dyn std::error::Error>> {
        if self.message.to_lowercase() == "woof" {
                Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "A cat shouldn't bark like a dog.").into())
            }
        
          else {
                let eye = if self.dead {"x"} else {"o"};
                match &self.inputfile {
                    Some(path) => {
                        let cat_template = std::fs::read_to_string(path).with_context(
                            || format!("Could not read file {:?}", path)
                            )?;
                        let eye = format!("{}", eye.red().bold());
                        let cat_picture = cat_template.replace("eye",
                        &eye);
                        println!(
                        "{}",
                        self.message.bright_yellow().underline().on_purple()
                        );
                        println!("{}", &cat_picture);
                        Ok(cat_picture)
                    }

                    None => {
                        let cat_base_template = " \\
                            /\\_/\\
                            ( {eye} {eye} )
                            =( I )=".replace("eye",
                        &eye);
                        Ok(cat_base_template)
                    }
            }
        }
    }

}
