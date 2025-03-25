use clap::Parser;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Write;

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli{

    pattern: String,
    path: std::path::PathBuf,

}



// Try to optimize with BufReader
fn main() -> io::Result<()> {



    let args = Cli::parse();
    let totalsize = std::fs::metadata(&args.path)?.len();
    let pb = ProgressBar::new(totalsize);
    pb.set_style(
    ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} (ETA: {eta_precise})",
    )
    .unwrap()
    .progress_chars("#>-"),);
    let f = File::open(&args.path)?;
    let reader = BufReader::new(f);    

    let mut position = 0;

    for line_content in reader.lines(){

        let line = line_content?;
        position +=  line.len() as u64 + 1;
        if line.contains(&args.pattern){


            println!("{}", line);
        }

        pb.set_position(position);

    }

    io::stdout().flush()?;

    println!("Search has been completed!!!");
    Ok(())
}


