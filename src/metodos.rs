//! Métodos embutidos dos tipos de coleção (chamados com `.`), ex.: `lista.adicione(x)`.
//!
//! Esta é a mesma máquina de despacho por `.` que a orientação a objetos (Fase 2)
//! vai reaproveitar para métodos de classes definidas pelo usuário.

use std::cell::RefCell;
use std::rc::Rc;

use crate::valor::{DicRef, ListaRef, Valor};

/// Despacha uma chamada de método `receiver.nome(args)`.
pub fn chamar_metodo(receiver: Valor, nome: &str, args: Vec<Valor>) -> Result<Valor, String> {
    match &receiver {
        Valor::Lista(l) => metodo_lista(l, nome, args),
        Valor::Texto(t) => metodo_texto(t, nome, args),
        Valor::Dicionario(d) => metodo_dic(d, nome, args),
        outro => Err(format!(
            "o tipo '{}' não tem métodos",
            outro.tipo_nome()
        )),
    }
}

// ---- Helpers de aridade e tipos de argumento ----

fn checar_aridade(nome: &str, args: &[Valor], esperado: usize) -> Result<(), String> {
    if args.len() != esperado {
        Err(format!(
            "o método '{}' espera {} argumento(s), mas recebeu {}",
            nome,
            esperado,
            args.len()
        ))
    } else {
        Ok(())
    }
}

fn arg_texto(nome: &str, args: &[Valor], i: usize) -> Result<String, String> {
    match &args[i] {
        Valor::Texto(t) => Ok(t.clone()),
        outro => Err(format!(
            "o método '{}' espera um 'texto', mas recebeu um '{}'",
            nome,
            outro.tipo_nome()
        )),
    }
}

fn arg_indice(nome: &str, args: &[Valor], i: usize) -> Result<usize, String> {
    match &args[i] {
        Valor::Numero(n) if n.fract() == 0.0 && *n >= 0.0 => Ok(*n as usize),
        outro => Err(format!(
            "o método '{}' espera um índice inteiro não negativo, mas recebeu um '{}'",
            nome,
            outro.tipo_nome()
        )),
    }
}

// ---- Métodos de lista ----

fn metodo_lista(l: &ListaRef, nome: &str, args: Vec<Valor>) -> Result<Valor, String> {
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
                return Err(format!(
                    "índice {} fora da lista (tamanho {})",
                    i,
                    lista.len()
                ));
            }
            Ok(lista.remove(i))
        }
        "tamanho" => {
            checar_aridade(nome, &args, 0)?;
            Ok(Valor::Numero(l.borrow().len() as f64))
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
        outro => Err(format!("o tipo 'lista' não tem o método '{}'", outro)),
    }
}

// ---- Métodos de texto ----

fn metodo_texto(t: &str, nome: &str, args: Vec<Valor>) -> Result<Valor, String> {
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
            Ok(Valor::Numero(t.chars().count() as f64))
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
        outro => Err(format!("o tipo 'texto' não tem o método '{}'", outro)),
    }
}

// ---- Métodos de dicionário ----

fn metodo_dic(d: &DicRef, nome: &str, args: Vec<Valor>) -> Result<Valor, String> {
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
        "remova" => {
            checar_aridade(nome, &args, 1)?;
            let chave = arg_texto(nome, &args, 0)?;
            Ok(d.borrow_mut().remove(&chave).unwrap_or(Valor::Nulo))
        }
        "tamanho" => {
            checar_aridade(nome, &args, 0)?;
            Ok(Valor::Numero(d.borrow().len() as f64))
        }
        outro => Err(format!("o tipo 'dicionario' não tem o método '{}'", outro)),
    }
}
