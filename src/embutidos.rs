//! Funções embutidas (biblioteca padrão mínima da Fase 1) — §10.

use std::cell::RefCell;
use std::rc::Rc;

use crate::ambiente::Ambiente;
use crate::valor::{Nativa, Valor};

/// Registra todas as funções embutidas no ambiente global.
pub fn registrar(amb: &Rc<RefCell<Ambiente>>) {
    let mut a = amb.borrow_mut();
    // Núcleo
    registrar_uma(&mut a, "escreva", escreva);
    registrar_uma(&mut a, "leia", leia);
    registrar_uma(&mut a, "tamanho", tamanho);
    registrar_uma(&mut a, "tipo", tipo);
    registrar_uma(&mut a, "paraTexto", para_texto);
    registrar_uma(&mut a, "paraNumero", para_numero);
    // Matemática (na Fase 2 vira o módulo 'matematica' via importe)
    registrar_uma(&mut a, "raiz", raiz);
    registrar_uma(&mut a, "absoluto", absoluto);
    registrar_uma(&mut a, "potencia", potencia);
    registrar_uma(&mut a, "piso", piso);
    registrar_uma(&mut a, "teto", teto);
    registrar_uma(&mut a, "arredonde", arredonde);
    registrar_uma(&mut a, "aleatorio", aleatorio);
    a.definir("PI", Valor::Numero(std::f64::consts::PI), true);
}

fn registrar_uma(amb: &mut Ambiente, nome: &str, func: fn(Vec<Valor>) -> Result<Valor, String>) {
    amb.definir(
        nome,
        Valor::Nativa(Rc::new(Nativa {
            nome: nome.to_string(),
            func,
        })),
        true,
    );
}

fn escreva(args: Vec<Valor>) -> Result<Valor, String> {
    let partes: Vec<String> = args.iter().map(|v| v.para_texto()).collect();
    println!("{}", partes.join(" "));
    Ok(Valor::Nulo)
}

fn tamanho(args: Vec<Valor>) -> Result<Valor, String> {
    let arg = um_argumento("tamanho", &args)?;
    match arg {
        Valor::Texto(t) => Ok(Valor::Numero(t.chars().count() as f64)),
        Valor::Lista(l) => Ok(Valor::Numero(l.borrow().len() as f64)),
        Valor::Dicionario(d) => Ok(Valor::Numero(d.borrow().len() as f64)),
        outro => Err(format!(
            "'tamanho' espera um 'texto', 'lista' ou 'dicionario', mas recebeu um '{}'",
            outro.tipo_nome()
        )),
    }
}

fn tipo(args: Vec<Valor>) -> Result<Valor, String> {
    let arg = um_argumento("tipo", &args)?;
    Ok(Valor::Texto(arg.tipo_nome().to_string()))
}

fn para_texto(args: Vec<Valor>) -> Result<Valor, String> {
    let arg = um_argumento("paraTexto", &args)?;
    Ok(Valor::Texto(arg.para_texto()))
}

fn para_numero(args: Vec<Valor>) -> Result<Valor, String> {
    let arg = um_argumento("paraNumero", &args)?;
    match arg {
        Valor::Numero(n) => Ok(Valor::Numero(*n)),
        Valor::Logico(b) => Ok(Valor::Numero(if *b { 1.0 } else { 0.0 })),
        Valor::Texto(t) => t
            .trim()
            .parse::<f64>()
            .map(Valor::Numero)
            .map_err(|_| format!("não consegui converter o texto \"{}\" em número", t)),
        outro => Err(format!(
            "não é possível converter um '{}' em número",
            outro.tipo_nome()
        )),
    }
}

fn leia(args: Vec<Valor>) -> Result<Valor, String> {
    if !args.is_empty() {
        return Err(format!("'leia' não espera argumentos, mas recebeu {}", args.len()));
    }
    use std::io::BufRead;
    let mut linha = String::new();
    std::io::stdin()
        .lock()
        .read_line(&mut linha)
        .map_err(|e| format!("erro ao ler a entrada: {}", e))?;
    let limpa = linha.trim_end_matches(['\n', '\r']).to_string();
    Ok(Valor::Texto(limpa))
}

// ---- Matemática ----

fn raiz(args: Vec<Valor>) -> Result<Valor, String> {
    let x = um_numero("raiz", &args)?;
    if x < 0.0 {
        return Err("'raiz' não aceita números negativos".into());
    }
    Ok(Valor::Numero(x.sqrt()))
}

fn absoluto(args: Vec<Valor>) -> Result<Valor, String> {
    Ok(Valor::Numero(um_numero("absoluto", &args)?.abs()))
}

fn piso(args: Vec<Valor>) -> Result<Valor, String> {
    Ok(Valor::Numero(um_numero("piso", &args)?.floor()))
}

fn teto(args: Vec<Valor>) -> Result<Valor, String> {
    Ok(Valor::Numero(um_numero("teto", &args)?.ceil()))
}

fn arredonde(args: Vec<Valor>) -> Result<Valor, String> {
    Ok(Valor::Numero(um_numero("arredonde", &args)?.round()))
}

fn potencia(args: Vec<Valor>) -> Result<Valor, String> {
    if args.len() != 2 {
        return Err(format!("'potencia' espera 2 argumentos, mas recebeu {}", args.len()));
    }
    let base = como_numero("potencia", &args[0])?;
    let exp = como_numero("potencia", &args[1])?;
    Ok(Valor::Numero(base.powf(exp)))
}

fn aleatorio(args: Vec<Valor>) -> Result<Valor, String> {
    if !args.is_empty() {
        return Err(format!("'aleatorio' não espera argumentos, mas recebeu {}", args.len()));
    }
    Ok(Valor::Numero(proximo_aleatorio()))
}

/// Gerador pseudoaleatório simples (xorshift64) com semente do relógio.
/// Suficiente para uso geral; não é criptograficamente seguro.
fn proximo_aleatorio() -> f64 {
    thread_local! {
        static ESTADO: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    }
    ESTADO.with(|estado| {
        let mut x = estado.get();
        if x == 0 {
            x = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos() as u64)
                .unwrap_or(0x9e3779b97f4a7c15)
                | 1;
        }
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        estado.set(x);
        // mapeia para [0, 1) usando 53 bits de mantissa
        (x >> 11) as f64 / ((1u64 << 53) as f64)
    })
}

/// Garante que a função recebeu exatamente um argumento.
fn um_argumento<'a>(nome: &str, args: &'a [Valor]) -> Result<&'a Valor, String> {
    if args.len() != 1 {
        Err(format!(
            "'{}' espera exatamente 1 argumento, mas recebeu {}",
            nome,
            args.len()
        ))
    } else {
        Ok(&args[0])
    }
}

/// Garante um único argumento numérico.
fn um_numero(nome: &str, args: &[Valor]) -> Result<f64, String> {
    como_numero(nome, um_argumento(nome, args)?)
}

fn como_numero(nome: &str, v: &Valor) -> Result<f64, String> {
    match v {
        Valor::Numero(n) => Ok(*n),
        outro => Err(format!(
            "'{}' espera um 'numero', mas recebeu um '{}'",
            nome,
            outro.tipo_nome()
        )),
    }
}
