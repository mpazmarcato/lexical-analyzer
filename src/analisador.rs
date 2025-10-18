use crate::iteradores::StrExt;
pub fn next(_entrada: &str, index: usize) -> Result<(usize, &str, &str, usize), Option<usize>> {
    let mut iter = StrExt::char_indices(_entrada);

    let mut start_byte = 0;
    let mut ch_opt = None;

    for (i, c) in iter.by_ref() {
        if !c.is_whitespace() && c != '🦀' {
            start_byte = i;
            ch_opt = Some(c);
            break;
        }
    }

    let ch = match ch_opt {
        Some(c) => c,
        None => return Err(None),
    };

    let start_position = index + _entrada[..start_byte].chars().count() + 1;

    if ch.is_ascii_digit() {
        let mut end_byte = start_byte + ch.len_utf8();

        for (_, c) in _entrada[end_byte..].char_indices() {
            if c.is_ascii_digit() {
                end_byte += c.len_utf8();
            } else {
                let token = &_entrada[start_byte..end_byte];
                let resto = &_entrada[end_byte..];
                let novo_index = index + _entrada[..end_byte].chars().count();
                return Ok((start_position, token, resto, novo_index));
            }
        }

        let token = &_entrada[start_byte..end_byte];
        let novo_index = index + _entrada[..end_byte].chars().count();
        return Ok((start_position, token, "", novo_index));
    }

    if "+-*/🐧".contains(ch) {
        let end_byte = start_byte + ch.len_utf8();
        let token = &_entrada[start_byte..end_byte];
        let resto = &_entrada[end_byte..];
        let novo_index = index + _entrada[..end_byte].chars().count();
        return Ok((start_position, token, resto, novo_index));
    }

    Err(Some(start_position))
}