pub fn read_inputs_txt(file_name: &str) -> anyhow::Result<String> {
    let path = format!("inputs/{file_name}.txt");
    Ok(std::fs::read_to_string(path)?)
}
