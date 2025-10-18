mod iteradores;
mod analisador;
use analisador::*;
use std::io::{self, Write};

fn main() {
    loop {
        print!("Digite uma expressão (ou pressione Enter para sair): ");
        io::stdout().flush().unwrap(); 

        let mut expression = String::new();
        io::stdin().read_line(&mut expression).unwrap();
        let expression = expression.trim(); 

        if expression.is_empty() {
            println!("Encerrando.");
            break;
        }

        let mut index = 0;
        let mut text = expression;

        loop {
            match next(text, index) {
                Ok((pos, token, resto, novo_index)) => {
                    println!("Token: {}, posição: {}", token, pos);
                    text = resto;
                    index = novo_index;
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
