pub mod day_1;
pub mod helpers;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let start = std::time::Instant::now();
    day_1::run()?;
    println!("operation complete in: {:#?}", start.elapsed());
    Ok(())
}
