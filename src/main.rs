mod iteradores;
mod analisador;
use analisador::próximo;
use std::io::{self, Write};

fn main() {
    loop {
        print!("Digite uma expressão (ou pressione Enter para sair): ");
        io::stdout().flush().unwrap();

        let mut expressão = String::new();
        io::stdin().read_line(&mut expressão).unwrap();

        let expressão = expressão.trim();
        if expressão.is_empty() {
            break;
        }
        println!("\nAnalisando: '{}'", expressão);

        let mut índice = 0;
        let mut texto = expressão;
        let mut resultado = Vec::new();
        loop {
            match próximo(texto, índice) {
                Ok((pos, token, resto, novo_índice)) => {
                    resultado.push(format!("(\"{}\", {})", token, pos));
                    texto = resto;
                    índice = novo_índice;

                    if texto.is_empty() {
                        break;
                    }
                }
                Err(None) => {
                    break;
                }
                Err(Some(pos)) => {
                    resultado.push(format!("Erro na posição {}", pos));
                    break;
                }
            }
        }

        if !resultado.is_empty() {
            let output = resultado.join(" ");
            println!("{}", output);
        } else {
            println!("Nenhum token encontrado.");
        }
        println!();
    }
}