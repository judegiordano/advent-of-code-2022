use crate::helpers;

pub fn run() -> anyhow::Result<()> {
    let content = helpers::read_inputs_txt("day1")?;
    for a in content.lines() {
        println!("{:#?}", a);
    }
    Ok(())
}
