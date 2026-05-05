/// UTF-8 BOM prefix bytes
const UTF8_BOM: &[u8] = &[0xEF, 0xBB, 0xBF];

/// Result of encoding detection and decoding
#[derive(Debug, Clone, PartialEq)]
pub struct DecodedFile {
    /// The decoded text content
    pub content: String,
    /// Detected encoding name (e.g. "UTF-8", "windows-1252")
    pub encoding: String,
    /// Whether a UTF-8 BOM was stripped during decoding
    pub bom_stripped: bool,
}

/// Strip UTF-8 BOM from the beginning of byte slice if present.
///
/// # Arguments
/// * `bytes` - The raw file bytes
///
/// # Returns
/// The byte slice with BOM removed if it was present, otherwise the original slice.
pub fn strip_bom(bytes: &[u8]) -> &[u8] {
    if bytes.starts_with(UTF8_BOM) {
        &bytes[UTF8_BOM.len()..]
    } else {
        bytes
    }
}

/// Detect encoding and decode bytes to a String.
///
/// This function attempts UTF-8 first, strips BOM if present, and falls back
/// to Windows-1252 (Latin-1) if UTF-8 decoding fails.
///
/// # Arguments
/// * `bytes` - The raw file bytes
///
/// # Returns
/// * `Ok(DecodedFile)` - Contains the decoded content, encoding name, and BOM flag
/// * `Err(String)` - Descriptive error message on failure
pub fn detect_and_decode(bytes: &[u8]) -> Result<DecodedFile, String> {
    let (bytes_after_bom, bom_stripped) = if bytes.starts_with(UTF8_BOM) {
        (bytes[UTF8_BOM.len()..].to_vec(), true)
    } else {
        (bytes.to_vec(), false)
    };

    // Attempt UTF-8 decode first
    if let Ok(s) = std::str::from_utf8(&bytes_after_bom) {
        return Ok(DecodedFile {
            content: s.to_string(),
            encoding: "UTF-8".to_string(),
            bom_stripped,
        });
    }

    // Fall back to windows-1252 via encoding_rs
    let decoder = encoding_rs::Encoding::for_label(b"windows-1252")
        .ok_or_else(|| "Invalid encoding label".to_string())?
        .new_decoder();

    // Prepare output buffer (max 4 bytes per input byte for valid windows-1252)
    let mut output = Vec::with_capacity(bytes_after_bom.len() * 2);
    let mut decoder = decoder;
    let mut total_read = 0usize;

    loop {
        let (result, read, _written, _was_input_consumed) =
            decoder.decode_to_utf8(&bytes_after_bom[total_read..], &mut output, true);

        total_read += read;

        match result {
            encoding_rs::CoderResult::InputEmpty => break,
            encoding_rs::CoderResult::OutputFull => {
                // Grow output and continue
                output.reserve(bytes_after_bom.len() / 2);
            }
        }

        if total_read >= bytes_after_bom.len() {
            break;
        }
    }

    let content = String::from_utf8(output)
        .map_err(|e| format!("Decoded bytes are not valid UTF-8: {}", e))?;

    Ok(DecodedFile {
        content,
        encoding: "windows-1252".to_string(),
        bom_stripped,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_utf8() {
        let input = b"Hello, World! \xE2\x99\x94"; // includes ♔
        let result = detect_and_decode(input).unwrap();
        assert_eq!(result.encoding, "UTF-8");
        assert!(!result.bom_stripped);
        assert_eq!(result.content, "Hello, World! ♔");
    }

    #[test]
    fn test_decode_utf8_with_bom() {
        let input = &[0xEF, 0xBB, 0xBF, b'H', b'i'];
        let result = detect_and_decode(input).unwrap();
        assert_eq!(result.encoding, "UTF-8");
        assert!(result.bom_stripped);
        assert_eq!(result.content, "Hi");
    }

    #[test]
    fn test_decode_latin1() {
        // Latin-1 encoded "café" where é is byte 0xE9
        let input = b"caf\xe9";
        let result = detect_and_decode(input).unwrap();
        assert_eq!(result.encoding, "windows-1252");
        assert!(!result.bom_stripped);
        assert_eq!(result.content, "café");
    }

    #[test]
    fn test_decode_empty() {
        let input = b"";
        let result = detect_and_decode(input).unwrap();
        assert_eq!(result.encoding, "UTF-8");
        assert!(!result.bom_stripped);
        assert_eq!(result.content, "");
    }

    #[test]
    fn test_strip_bom() {
        let with_bom = &[0xEF, 0xBB, 0xBF, b'A', b'B'];
        let without_bom = b"AB";

        assert_eq!(strip_bom(with_bom), without_bom);
        assert_eq!(strip_bom(without_bom), without_bom);
    }
}
