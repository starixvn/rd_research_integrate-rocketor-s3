use std::{collections::HashMap, fs::File, io::BufReader};

pub async fn read_summary(
    file_path: &str,
) -> Result<HashMap<String, i32>, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(file_path).expect("Failed to open file"));

    let summary: HashMap<String, i32> = serde_json::from_reader(reader)?;

    Ok(summary)
}

// Example test to verify the function
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_read_summary() {
        // Create a temporary test file
        let test_json = r#"
{
  "AAAA": 1
}
        "#;
        std::fs::write("test_summary.json", test_json).unwrap();

        let result = read_summary("test_summary.json").await;
        assert!(result.is_ok());

        // Cleanup
        std::fs::remove_file("test_summary.json").unwrap();
    }
}
