use crate::iteradores::StrExt;

pub fn próximo(entrada: &str, índice_atual: usize) -> Result<(usize, &str, &str, usize), Option<usize>> {
    let mut iter = entrada.meus_char_indices();
    let mut início_byte = 0;
    let mut início_char = 0;
    let mut ch_opcional = None;

    while let Some((byte_index, char_index, ch)) = iter.next() {
        if !ch.is_whitespace() && ch != '🦀' {
            início_byte = byte_index;
            início_char = char_index;
            ch_opcional = Some(ch);
            break;
        }
    }

    let ch = match ch_opcional {
        Some(c) => c,
        None => return Err(None),
    };

    let posição = índice_atual + início_char + 1;
    if ch.is_ascii_digit() {
        let mut fim_byte = início_byte + ch.len_utf8();
        let mut contagem_chars = 1;

        while let Some((byte_idx, _, next_ch)) = iter.next() {
            if next_ch.is_ascii_digit() {
                fim_byte = byte_idx + next_ch.len_utf8();
                contagem_chars += 1;
            } else {
                break;
            }
        }

        let token = &entrada[início_byte..fim_byte];
        let resto = &entrada[fim_byte..];
        let novo_índice = índice_atual + início_char + contagem_chars;
        return Ok((posição, token, resto, novo_índice));
    }

    if "+-*/".contains(ch) || ch == '🐧' {
        let fim_byte = início_byte + ch.len_utf8();
        let token = &entrada[início_byte..fim_byte];
        let resto = &entrada[fim_byte..];
        let novo_índice = índice_atual + início_char + 1;

        return Ok((posição, token, resto, novo_índice));
    }
    Err(Some(posição))
}