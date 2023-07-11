pub fn assembling_error(line_number: usize, line: &str, error: &str) -> String {
    
    format!("Error in line: {line_number}\n\t{line}\n\t{error}")
}