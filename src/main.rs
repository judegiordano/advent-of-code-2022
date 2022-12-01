use clap::Parser;

use crate::helpers::read_inputs_txt;

pub mod day_1;
pub mod helpers;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// input file name no extension
    #[arg(short, long)]
    path: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let Args { path } = Args::parse();
    let content = read_inputs_txt(&path)?;
    let start = std::time::Instant::now();
    day_1::run(&content)?;
    println!("operation complete in: {:#?}", start.elapsed());
    Ok(())
}
