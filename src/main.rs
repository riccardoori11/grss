use std::cmp::min;
use clap::Parser;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::thread;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Write;
use std::time::Duration;

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
    let mut count = 0;
    let mut position = 0;

    for line_content in reader.lines(){
        let line = line_content?;
        let new = min(position + line.len() as u64 + 1, totalsize);
        if line.contains(&args.pattern){

            count += 1;
        }
        position = new;;
        pb.set_position(new);
        thread::sleep(Duration::from_millis((5)));
    }


    println!("Search has been completed!!!, there are {} instances", count);
    Ok(())
}