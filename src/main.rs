//! kaju — interpretador da linguagem kaju (Fase 1).
//!
//! Uso:
//!   kaju arquivo.kaju     executa um arquivo
//!   kaju                  abre o REPL interativo

mod ambiente;
mod ast;
mod embutidos;
mod erros;
mod interpreter;
mod lexer;
mod metodos;
mod parser;
mod token;
mod valor;

use std::io::{self, BufRead, Write};
use std::process::ExitCode;

use interpreter::Interpretador;
use lexer::Lexer;
use parser::Parser;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(caminho) => executar_arquivo(caminho),
        None => {
            repl();
            ExitCode::SUCCESS
        }
    }
}

fn executar_arquivo(caminho: &str) -> ExitCode {
    let fonte = match std::fs::read_to_string(caminho) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("kaju: não consegui ler '{}': {}", caminho, e);
            return ExitCode::FAILURE;
        }
    };

    match rodar(&fonte) {
        Ok(()) => ExitCode::SUCCESS,
        Err(diag) => {
            eprint!("{}", diag.render(caminho, &fonte));
            ExitCode::FAILURE
        }
    }
}

/// Pipeline completo: fonte -> tokens -> AST -> execução.
fn rodar(fonte: &str) -> Result<(), erros::Diagnostico> {
    let tokens = Lexer::novo(fonte).tokenizar()?;
    let programa = Parser::novo(tokens).analisar()?;
    let mut interp = Interpretador::novo();
    interp.executar_programa(&programa)
}

fn repl() {
    println!("kaju (Fase 1) — digite código e pressione Enter. Ctrl+D para sair.");
    let entrada = io::stdin();
    let mut interp = Interpretador::novo();

    loop {
        print!("kaju> ");
        let _ = io::stdout().flush();

        let mut linha = String::new();
        match entrada.lock().read_line(&mut linha) {
            Ok(0) => {
                println!();
                break;
            }
            Ok(_) => {}
            Err(_) => break,
        }

        if linha.trim().is_empty() {
            continue;
        }

        match Lexer::novo(&linha)
            .tokenizar()
            .and_then(|t| Parser::novo(t).analisar())
        {
            Ok(programa) => {
                if let Err(diag) = interp.executar_programa(&programa) {
                    eprint!("{}", diag.render("<repl>", &linha));
                }
            }
            Err(diag) => eprint!("{}", diag.render("<repl>", &linha)),
        }
    }
}
