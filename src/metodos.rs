//! Métodos embutidos dos tipos de coleção (chamados com `.`), ex.: `lista.adicione(x)`.
//!
//! É a mesma máquina de despacho por `.` que a orientação a objetos reaproveita
//! para os métodos de classes definidas pelo usuário.

use std::cell::RefCell;
use std::rc::Rc;

use crate::valor::{DicRef, ListaRef, Valor};

/// Erro de um método: código de diagnóstico (`Kxxx`) + mensagem. O interpretador
/// converte isso num `Diagnostico` completo, preservando o código correto.
pub type ErroMetodo = (&'static str, String);

/// Despacha uma chamada de método `receiver.nome(args)`.
pub fn chamar_metodo(receiver: Valor, nome: &str, args: Vec<Valor>) -> Result<Valor, ErroMetodo> {
    match &receiver {
        Valor::Lista(l) => metodo_lista(l, nome, args),
        Valor::Texto(t) => metodo_texto(t, nome, args),
        Valor::Dicionario(d) => metodo_dic(d, nome, args),
        outro => Err((
            "K212",
            format!("o tipo '{}' não tem métodos", outro.tipo_nome()),
        )),
    }
}

// ---- Helpers de aridade e tipos de argumento ----

fn checar_aridade(nome: &str, args: &[Valor], esperado: usize) -> Result<(), ErroMetodo> {
    if args.len() != esperado {
        Err((
            "K201",
            format!(
                "o método '{}' espera {} argumento(s), mas recebeu {}",
                nome,
                esperado,
                args.len()
            ),
        ))
    } else {
        Ok(())
    }
}

fn arg_texto(nome: &str, args: &[Valor], i: usize) -> Result<String, ErroMetodo> {
    match &args[i] {
        Valor::Texto(t) => Ok(t.clone()),
        outro => Err((
            "K203",
            format!(
                "o método '{}' espera um 'texto', mas recebeu um '{}'",
                nome,
                outro.tipo_nome()
            ),
        )),
    }
}

fn arg_indice(nome: &str, args: &[Valor], i: usize) -> Result<usize, ErroMetodo> {
    match &args[i] {
        Valor::Inteiro(n) if *n >= 0 => Ok(*n as usize),
        Valor::Decimal(f) if f.fract() == 0.0 && *f >= 0.0 => Ok(*f as usize),
        outro => Err((
            "K203",
            format!(
                "o método '{}' espera um índice inteiro não negativo, mas recebeu um '{}'",
                nome,
                outro.tipo_nome()
            ),
        )),
    }
}

// ---- Métodos de lista ----

fn metodo_lista(l: &ListaRef, nome: &str, args: Vec<Valor>) -> Result<Valor, ErroMetodo> {
    match nome {
        "adicione" => {
            checar_aridade(nome, &args, 1)?;
            l.borrow_mut().push(args.into_iter().next().unwrap());
            Ok(Valor::Nulo)
        }
        "remova" => {
            checar_aridade(nome, &args, 1)?;
            let i = arg_indice(nome, &args, 0)?;
            let mut lista = l.borrow_mut();
            if i >= lista.len() {
                return Err((
                    "K206",
                    format!("índice {} fora da lista (tamanho {})", i, lista.len()),
                ));
            }
            Ok(lista.remove(i))
        }
        "tamanho" => {
            checar_aridade(nome, &args, 0)?;
            Ok(Valor::Inteiro(l.borrow().len() as i64))
        }
        "contem" => {
            checar_aridade(nome, &args, 1)?;
            let alvo = &args[0];
            let achou = l.borrow().iter().any(|v| v.igual(alvo));
            Ok(Valor::Logico(achou))
        }
        "inverta" => {
            checar_aridade(nome, &args, 0)?;
            l.borrow_mut().reverse();
            Ok(Valor::Nulo)
        }
        "junte" => {
            checar_aridade(nome, &args, 1)?;
            let sep = arg_texto(nome, &args, 0)?;
            let partes: Vec<String> = l.borrow().iter().map(|v| v.para_texto()).collect();
            Ok(Valor::Texto(partes.join(&sep)))
        }
        "indiceDe" => {
            checar_aridade(nome, &args, 1)?;
            let alvo = &args[0];
            let pos = l.borrow().iter().position(|v| v.igual(alvo));
            Ok(Valor::Inteiro(pos.map(|p| p as i64).unwrap_or(-1)))
        }
        "fatie" => {
            checar_aridade(nome, &args, 2)?;
            let inicio = arg_indice(nome, &args, 0)?;
            let fim = arg_indice(nome, &args, 1)?;
            let lista = l.borrow();
            let fim = fim.min(lista.len());
            let fatia: Vec<Valor> = if inicio >= fim {
                Vec::new()
            } else {
                lista[inicio..fim].to_vec()
            };
            Ok(Valor::Lista(Rc::new(RefCell::new(fatia))))
        }
        "ordene" => {
            checar_aridade(nome, &args, 0)?;
            ordenar_lista(l)?;
            Ok(Valor::Nulo)
        }
        "soma" => {
            checar_aridade(nome, &args, 0)?;
            somar_lista(l)
        }
        outro => Err((
            "K212",
            format!("o tipo 'lista' não tem o método '{}'", outro),
        )),
    }
}

/// Soma os itens de uma lista de números. Inteiro se todos forem inteiros.
fn somar_lista(l: &ListaRef) -> Result<Valor, ErroMetodo> {
    let lista = l.borrow();
    let todos_inteiros = lista.iter().all(|v| matches!(v, Valor::Inteiro(_)));
    if todos_inteiros {
        let mut total: i64 = 0;
        for v in lista.iter() {
            if let Valor::Inteiro(i) = v {
                total = total.wrapping_add(*i);
            }
        }
        Ok(Valor::Inteiro(total))
    } else {
        let mut total = 0.0;
        for v in lista.iter() {
            match v.como_f64() {
                Some(n) => total += n,
                None => {
                    return Err(("K203", "'soma' só funciona com listas de números".to_string()))
                }
            }
        }
        Ok(Valor::Decimal(total))
    }
}

/// Ordena uma lista in-place: números por valor, textos em ordem alfabética.
fn ordenar_lista(l: &ListaRef) -> Result<(), ErroMetodo> {
    let mut lista = l.borrow_mut();
    let todos_numeros = lista.iter().all(|v| v.como_f64().is_some());
    let todos_textos = lista.iter().all(|v| matches!(v, Valor::Texto(_)));

    if todos_numeros {
        lista.sort_by(|a, b| {
            a.como_f64()
                .unwrap()
                .partial_cmp(&b.como_f64().unwrap())
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Ok(())
    } else if todos_textos {
        lista.sort_by(|a, b| match (a, b) {
            (Valor::Texto(x), Valor::Texto(y)) => x.cmp(y),
            _ => std::cmp::Ordering::Equal,
        });
        Ok(())
    } else {
        Err((
            "K203",
            "'ordene' só funciona com listas de números ou de textos".to_string(),
        ))
    }
}

// ---- Métodos de texto ----

fn metodo_texto(t: &str, nome: &str, args: Vec<Valor>) -> Result<Valor, ErroMetodo> {
    match nome {
        "maiusculas" => {
            checar_aridade(nome, &args, 0)?;
            Ok(Valor::Texto(t.to_uppercase()))
        }
        "minusculas" => {
            checar_aridade(nome, &args, 0)?;
            Ok(Valor::Texto(t.to_lowercase()))
        }
        "tamanho" => {
            checar_aridade(nome, &args, 0)?;
            Ok(Valor::Inteiro(t.chars().count() as i64))
        }
        "contem" => {
            checar_aridade(nome, &args, 1)?;
            let sub = arg_texto(nome, &args, 0)?;
            Ok(Valor::Logico(t.contains(&sub)))
        }
        "apara" => {
            checar_aridade(nome, &args, 0)?;
            Ok(Valor::Texto(t.trim().to_string()))
        }
        "substitua" => {
            checar_aridade(nome, &args, 2)?;
            let de = arg_texto(nome, &args, 0)?;
            let para = arg_texto(nome, &args, 1)?;
            Ok(Valor::Texto(t.replace(&de, &para)))
        }
        "divida" => {
            checar_aridade(nome, &args, 1)?;
            let sep = arg_texto(nome, &args, 0)?;
            let partes: Vec<Valor> = if sep.is_empty() {
                t.chars().map(|c| Valor::Texto(c.to_string())).collect()
            } else {
                t.split(&sep).map(|s| Valor::Texto(s.to_string())).collect()
            };
            Ok(Valor::Lista(Rc::new(RefCell::new(partes))))
        }
        "comecaCom" => {
            checar_aridade(nome, &args, 1)?;
            let pre = arg_texto(nome, &args, 0)?;
            Ok(Valor::Logico(t.starts_with(&pre)))
        }
        "terminaCom" => {
            checar_aridade(nome, &args, 1)?;
            let suf = arg_texto(nome, &args, 0)?;
            Ok(Valor::Logico(t.ends_with(&suf)))
        }
        "repita" => {
            checar_aridade(nome, &args, 1)?;
            let n = arg_indice(nome, &args, 0)?;
            Ok(Valor::Texto(t.repeat(n)))
        }
        "indiceDe" => {
            checar_aridade(nome, &args, 1)?;
            let sub = arg_texto(nome, &args, 0)?;
            // índice em caracteres (não em bytes), ou -1 se não achar
            let pos = t.find(&sub).map(|byte| t[..byte].chars().count() as i64);
            Ok(Valor::Inteiro(pos.unwrap_or(-1)))
        }
        "fatie" => {
            checar_aridade(nome, &args, 2)?;
            let inicio = arg_indice(nome, &args, 0)?;
            let fim = arg_indice(nome, &args, 1)?;
            let chars: Vec<char> = t.chars().collect();
            let fim = fim.min(chars.len());
            let fatia: String = if inicio >= fim {
                String::new()
            } else {
                chars[inicio..fim].iter().collect()
            };
            Ok(Valor::Texto(fatia))
        }
        outro => Err((
            "K212",
            format!("o tipo 'texto' não tem o método '{}'", outro),
        )),
    }
}

// ---- Métodos de dicionário ----

fn metodo_dic(d: &DicRef, nome: &str, args: Vec<Valor>) -> Result<Valor, ErroMetodo> {
    match nome {
        "chaves" => {
            checar_aridade(nome, &args, 0)?;
            let mut chaves: Vec<String> = d.borrow().keys().cloned().collect();
            chaves.sort();
            let itens: Vec<Valor> = chaves.into_iter().map(Valor::Texto).collect();
            Ok(Valor::Lista(Rc::new(RefCell::new(itens))))
        }
        "valores" => {
            checar_aridade(nome, &args, 0)?;
            let mapa = d.borrow();
            let mut chaves: Vec<&String> = mapa.keys().collect();
            chaves.sort();
            let itens: Vec<Valor> = chaves.iter().map(|c| mapa[*c].clone()).collect();
            Ok(Valor::Lista(Rc::new(RefCell::new(itens))))
        }
        "tem" => {
            checar_aridade(nome, &args, 1)?;
            let chave = arg_texto(nome, &args, 0)?;
            Ok(Valor::Logico(d.borrow().contains_key(&chave)))
        }
        "obtem" => {
            // obtem(chave, padrao) -> valor da chave, ou padrao se ausente
            checar_aridade(nome, &args, 2)?;
            let chave = arg_texto(nome, &args, 0)?;
            Ok(d.borrow().get(&chave).cloned().unwrap_or_else(|| args[1].clone()))
        }
        "remova" => {
            checar_aridade(nome, &args, 1)?;
            let chave = arg_texto(nome, &args, 0)?;
            Ok(d.borrow_mut().remove(&chave).unwrap_or(Valor::Nulo))
        }
        "tamanho" => {
            checar_aridade(nome, &args, 0)?;
            Ok(Valor::Inteiro(d.borrow().len() as i64))
        }
        outro => Err((
            "K212",
            format!("o tipo 'dicionario' não tem o método '{}'", outro),
        )),
    }
}
