
pub fn split_patch<E: serde::de::Error>(s: &str) -> Result<(u16, &str), E> {
    if s.is_empty() {
        return Err(E::custom("Empty".to_string()));
    }
    let patch: u16;
    let other: &str;
    if let Some(pos) = s.chars().position(|c| !c.is_ascii_digit()) {
        let ps = &s[..pos];
        other = &s[pos..];
        if ps.is_empty() {
            return Err(E::custom("Invalid format".to_string()));
        }
        patch = ps.parse::<u16>().map_err(E::custom)?;
    } else {
        patch = s.parse::<u16>().map_err(E::custom)?;
        other = "";
    }
    Ok((patch, other))
}
