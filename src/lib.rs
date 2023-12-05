#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let _content = find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<(), Box<dyn std::error::Error>> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        };
    };

    Ok(())
}