use sha2::{Digest, Sha256};

pub fn generate_sha256(str: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(str.as_bytes());
    hasher
        .finalize()
        .to_vec()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_sha256() {
        let str = "AAAA-0";
        let sha256 = generate_sha256(str);
        assert_eq!(
            sha256,
            "53e5fe7054fb5c4867e0277e8867c0c08f9ca8be43fdac0d3b6c6ea5c7d656a2"
        );
    }
}
