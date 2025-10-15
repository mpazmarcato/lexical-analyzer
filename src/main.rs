mod iteradores;
mod analisador;
use iteradores::*;
use analisador::*;

fn main() {
    let mut entrada = "450 + 20";
    let mut offset = 0;

    loop {
        match proximo(entrada, offset) {
            Ok((pos, token, resto, novo_offset)) => {
                println!("Token: {}, posição: {}", token, pos);
                entrada = resto;
                offset = novo_offset;
            }
            Err(None) => {
                println!("Fim da análise");
                break;
            }
            Err(Some(pos)) => {
                println!("Erro na posição {}", pos);
                break;
            }
        }
    }
}