/// Example: "really long" -> "really l…"
pub fn truncate_text(text: &str, length: usize) -> String {
    if text.len() < length {
        return text.to_string();
    }

    format!("{}…", &text[..length-1].trim())
}
