//! kaju — interpretador da linguagem kaju.
//!
//! Uso:
//!   kaju arquivo.kaju     executa um arquivo
//!   kaju                  abre o REPL interativo

mod ambiente;
mod ast;
mod embutidos;
mod erros;
mod explicacoes;
mod interpreter;
mod lexer;
mod metodos;
mod parser;
mod token;
mod valor;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use interpreter::Interpretador;
use lexer::Lexer;
use parser::Parser;
use token::TipoToken;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        Some("explique") => explicar_codigo(args.get(2)),
        Some("--ajuda") | Some("-a") | Some("ajuda") => {
            mostrar_ajuda();
            ExitCode::SUCCESS
        }
        Some("--versao") | Some("-v") | Some("versao") => {
            println!("kaju {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Some(caminho) => executar_arquivo(caminho),
        None => {
            repl();
            ExitCode::SUCCESS
        }
    }
}

fn mostrar_ajuda() {
    println!(
        "kaju {} — linguagem de programação interpretada, em português.

USO:
    kaju                     abre o REPL interativo
    kaju <arquivo>           executa um arquivo .kaju (ou .kj)
    kaju explique <codigo>   explica um código de erro (ex.: kaju explique K016)
    kaju --versao            mostra a versão instalada
    kaju --ajuda             mostra esta ajuda

EXEMPLOS:
    kaju programa.kaju
    kaju explique K001

No REPL: digite código e Enter; as setas ↑/↓ navegam no histórico;
Ctrl+D sai.

Documentação: o livro em docs/livro e a especificação em ESPECIFICACAO.md.",
        env!("CARGO_PKG_VERSION")
    );
}

/// `kaju explique K016` — mostra a explicação detalhada de um código de erro.
fn explicar_codigo(codigo: Option<&String>) -> ExitCode {
    let codigo = match codigo {
        Some(c) => c,
        None => {
            eprintln!("uso: kaju explique <codigo>   (ex.: kaju explique K016)");
            eprintln!(
                "códigos com explicação: {}",
                explicacoes::codigos_conhecidos().join(", ")
            );
            return ExitCode::FAILURE;
        }
    };
    // aceita "k016", "016" ou "K016"
    let mut norm = codigo.trim().to_uppercase();
    if !norm.starts_with('K') {
        norm = format!("K{}", norm);
    }
    match explicacoes::explicar(&norm) {
        Some(texto) => {
            println!("{}", texto);
            ExitCode::SUCCESS
        }
        None => {
            eprintln!("Ainda não há explicação detalhada para {}.", norm);
            eprintln!(
                "códigos com explicação: {}",
                explicacoes::codigos_conhecidos().join(", ")
            );
            ExitCode::FAILURE
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

    let base = Path::new(caminho)
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."));

    match rodar(&fonte, base) {
        Ok(()) => ExitCode::SUCCESS,
        Err(diags) => {
            for diag in &diags {
                eprint!("{}", diag.render(caminho, &fonte));
                eprintln!();
            }
            if diags.len() > 1 {
                eprintln!("({} erros encontrados)", diags.len());
            }
            if let Some(primeiro) = diags.first() {
                eprintln!(
                    "dica: rode 'kaju explique {}' para entender este erro.",
                    primeiro.codigo
                );
            }
            ExitCode::FAILURE
        }
    }
}

/// Pipeline completo: fonte -> tokens -> AST -> execução.
/// Devolve uma lista de erros (o parser pode relatar vários de uma vez).
fn rodar(fonte: &str, base: PathBuf) -> Result<(), Vec<erros::Diagnostico>> {
    let tokens = Lexer::novo(fonte).tokenizar().map_err(|d| vec![d])?;
    let programa = Parser::novo(tokens).analisar()?;
    let mut interp = Interpretador::com_base(base);
    interp.executar_programa(&programa).map_err(|d| vec![d])
}

/// Uma entrada do REPL está completa quando os delimitadores {} () [] estão
/// balanceados. Enquanto houver algo aberto, o REPL pede mais linhas.
fn entrada_completa(fonte: &str) -> bool {
    let tokens = match Lexer::novo(fonte).tokenizar() {
        Ok(t) => t,
        // erro léxico (ex.: aspas sem fechar): deixa o executor reportar.
        Err(_) => return true,
    };
    let mut saldo: i32 = 0;
    for t in &tokens {
        match t.tipo {
            TipoToken::ChaveEsq | TipoToken::ParenEsq | TipoToken::ColcheteEsq => saldo += 1,
            TipoToken::ChaveDir | TipoToken::ParenDir | TipoToken::ColcheteDir => saldo -= 1,
            _ => {}
        }
    }
    saldo <= 0
}

fn caminho_historico() -> Option<PathBuf> {
    std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".kaju_historico"))
}

fn repl() {
    use rustyline::error::ReadlineError;
    use rustyline::DefaultEditor;

    println!("kaju — REPL interativo.");
    println!("Digite código e Enter. Blocos com {{ }} pedem mais linhas (prompt ....>).");
    println!("Seta ↑/↓ navega no histórico; Ctrl+C cancela a linha; Ctrl+D sai.");

    let mut editor = match DefaultEditor::new() {
        Ok(e) => e,
        Err(err) => {
            eprintln!("kaju: não consegui iniciar o REPL: {}", err);
            return;
        }
    };
    let historico = caminho_historico();
    if let Some(h) = &historico {
        let _ = editor.load_history(h);
    }

    let mut interp = Interpretador::novo();

    'sessao: loop {
        let mut buffer = String::new();
        let mut prompt = "kaju> ";
        // Lê linhas até a entrada ficar completa (blocos multilinha).
        loop {
            match editor.readline(prompt) {
                Ok(linha) => {
                    buffer.push_str(&linha);
                    buffer.push('\n');
                    if entrada_completa(&buffer) {
                        break;
                    }
                    prompt = "....> ";
                }
                // Ctrl+C: abandona o que estava sendo digitado e recomeça.
                Err(ReadlineError::Interrupted) => continue 'sessao,
                // Ctrl+D: sai do REPL.
                Err(ReadlineError::Eof) => break 'sessao,
                Err(err) => {
                    eprintln!("kaju: erro de leitura: {}", err);
                    break 'sessao;
                }
            }
        }

        if buffer.trim().is_empty() {
            continue;
        }
        let _ = editor.add_history_entry(buffer.trim_end());

        let tokens = match Lexer::novo(&buffer).tokenizar() {
            Ok(t) => t,
            Err(diag) => {
                eprint!("{}", diag.render("<repl>", &buffer));
                continue;
            }
        };
        match Parser::novo(tokens).analisar() {
            Ok(programa) => match interp.executar_repl(&programa) {
                Ok(Some(resultado)) => println!("{}", resultado),
                Ok(None) => {}
                Err(diag) => eprint!("{}", diag.render("<repl>", &buffer)),
            },
            Err(diags) => {
                for diag in diags {
                    eprint!("{}", diag.render("<repl>", &buffer));
                }
            }
        }
    }

    if let Some(h) = &historico {
        let _ = editor.save_history(h);
    }
}
