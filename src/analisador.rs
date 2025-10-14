use super::iteradores::*;

pub fn proximo(_entrada: &str) -> Result<(usize, &str, &str), Option<usize>> {
    let entrada = _entrada.trim_start_matches(|c: char| c.is_whitespace() || c == '🦀');

    if entrada.len() == 0 {
        return Err(None);
    }

    let mut iter = entrada.char_indices();

    let (start_byte, ch) = match iter.next() {
        Some(v) => v,
        None => return Err(None),
    };

    let start_position = entrada[..start_byte].chars().count() + 1;

    if ch.is_ascii_digit() {
        let mut end_byte = start_byte + ch.len_utf8();

        for (i, c) in entrada[end_byte..].char_indices() {
            if c.is_ascii_digit() {
                end_byte += ch.len_utf8();
            } else {
                let token = &entrada[start_byte..end_byte];
                let resto = &entrada[end_byte..];
                return Ok((start_position, token, resto));
            }
        }

        let token = &entrada[start_byte..end_byte];
        let resto = "";
        return Ok((start_position, token, resto));
    }

    if "+-*/🐧".contains(ch) {
        let end_byte = start_byte + ch.len_utf8();
        let token = &entrada[start_byte..end_byte];
        let resto = &entrada[end_byte..];
        return Ok((start_position, token, resto));
    }

    Err(Some(start_position))
}