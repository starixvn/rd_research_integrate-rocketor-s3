pub fn parse_m3u8_segments(m3u8_content: &str) -> Vec<String> {
    let mut segments = Vec::new();

    for line in m3u8_content.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        segments.push(line.trim().to_string());
    }

    segments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_m3u8_segments() {
        let m3u8_content = r#"
#EXTM3U
#EXT-X-VERSION:3
#EXT-X-TARGETDURATION:3
#EXT-X-MEDIA-SEQUENCE:0
#EXT-X-KEY:METHOD=AES-128,URI="encryption.key",IV=0x00000000000000000000000000000000
#EXTINF:3.000000,
0aae33c06bd5a4d66fca48c47f57ab8f5983482fa88b3742da5818143cd69a13_720p0.ts
#EXTINF:3.000000,
0aae33c06bd5a4d66fca48c47f57ab8f5983482fa88b3742da5818143cd69a13_720p1.ts
#EXTINF:3.000000,
0aae33c06bd5a4d66fca48c47f57ab8f5983482fa88b3742da5818143cd69a13_720p2.ts
#EXTINF:3.000000,
0aae33c06bd5a4d66fca48c47f57ab8f5983482fa88b3742da5818143cd69a13_720p3.ts
#EXT-X-ENDLIST"#;
        let segments = parse_m3u8_segments(m3u8_content);
        assert_eq!(segments.len(), 4);
        assert_eq!(
            segments[0],
            "0aae33c06bd5a4d66fca48c47f57ab8f5983482fa88b3742da5818143cd69a13_720p0.ts"
        );
        assert_eq!(
            segments[1],
            "0aae33c06bd5a4d66fca48c47f57ab8f5983482fa88b3742da5818143cd69a13_720p1.ts"
        );
        assert_eq!(
            segments[2],
            "0aae33c06bd5a4d66fca48c47f57ab8f5983482fa88b3742da5818143cd69a13_720p2.ts"
        );
        assert_eq!(
            segments[3],
            "0aae33c06bd5a4d66fca48c47f57ab8f5983482fa88b3742da5818143cd69a13_720p3.ts"
        );
    }
}
