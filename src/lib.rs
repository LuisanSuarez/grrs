pub fn find_matches(result: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in result.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}
