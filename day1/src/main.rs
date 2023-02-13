use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::{Result, anyhow};
use std::cmp::max;

fn main() -> Result<()> {
    let file = File::open(env::args().nth(1).ok_or(anyhow!("missing path"))?)?;
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    let mut max_calories_required: u64 = 0;
    let mut elf_sum : u64 = 0;
    loop {
        match reader.read_line(&mut buf) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let calories_per_elf = buf.trim();
                match calories_per_elf.len() {
                    0 => {
                        max_calories_required = max(elf_sum, max_calories_required);
                        elf_sum = 0;
                    }
                    ,
                    _ => {
                        if let Ok(caloris) = calories_per_elf.parse::<u64>() {
                            elf_sum += caloris;
                        }
                    }
                }
            },
            Err(e) => {
                panic!("{e}");
            }
        }
        buf.clear();
    }

    println!("max calories required: {}", max_calories_required);
    Ok(())
}
