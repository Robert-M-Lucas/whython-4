
pub fn create_error<T>(error: String, line: usize) -> Result<T, String> {
    Err(format!("Line {}: {}", line + 1, error))
}