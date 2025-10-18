mod iteradores;
mod analisador;
use iteradores::*;
use analisador::*;

fn main() {
    let exemplos = [
        "450 + 20",
        "450     +     20",
        "450+20",
        "0+-0",
        "0 +++",
        "10+a",
        "10 + 20a",
    ];

    for mut entrada in exemplos {
        println!("{}", entrada);
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
        println!();
    }
}