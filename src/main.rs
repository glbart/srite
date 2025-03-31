use std::io::{self, Write};

fn main() -> std::io::Result<()> {
    loop {
        print!("db > ");
        io::stdout().flush()?;

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Couldn't parse stdin");

        let command = buf.trim();

        if command == ".exit" {
            print!("Lucky srite!");
            return Ok(());
        } else {
            println!("Unknown type srite");
        }
    }
}
