mod iteradores;
mod analisador;
use iteradores::*;
use analisador::*;

fn main() {
    // Exemplo de uso:
    let entrada = "450 + 20";
    println!("Entrada: {entrada}");

    let mut resto = entrada;

    loop {
        match proximo(resto) {
            Ok((pos, token, novo_resto)) {
                println!("Token: {token}, posição: {pos}");
                resto = novo_resto;
            }
            Err(Some(erro)) => {
                println!("Erro na posição {erro}");
                break;
            }
            Err(None) => {
                println!("Fim da análise");
                break
            }
        }
    }
}