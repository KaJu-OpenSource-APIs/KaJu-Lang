//! Modelo de valores em tempo de execução da kaju.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ambiente::Ambiente;
use crate::ast::Cmd;

pub type ListaRef = Rc<RefCell<Vec<Valor>>>;
pub type DicRef = Rc<RefCell<HashMap<String, Valor>>>;

/// Uma função definida em kaju, com o ambiente capturado (closure).
pub struct FuncaoKaju {
    pub nome: Option<String>,
    pub params: Vec<String>,
    pub corpo: Vec<Cmd>,
    pub closure: Rc<RefCell<Ambiente>>,
}

/// Uma função embutida, implementada em Rust.
pub struct Nativa {
    pub nome: String,
    pub func: fn(Vec<Valor>) -> Result<Valor, String>,
}

/// Um valor kaju.
#[derive(Clone)]
pub enum Valor {
    Numero(f64),
    Texto(String),
    Logico(bool),
    Lista(ListaRef),
    Dicionario(DicRef),
    Funcao(Rc<FuncaoKaju>),
    Nativa(Rc<Nativa>),
    Nulo,
}

/// Formata um número: inteiros sem casas decimais, o resto normalmente.
pub fn formatar_numero(n: f64) -> String {
    if n.is_finite() && n.fract() == 0.0 && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        format!("{}", n)
    }
}

impl Valor {
    /// Nome do tipo, como devolvido por `tipo(x)`.
    pub fn tipo_nome(&self) -> &'static str {
        match self {
            Valor::Numero(_) => "numero",
            Valor::Texto(_) => "texto",
            Valor::Logico(_) => "logico",
            Valor::Lista(_) => "lista",
            Valor::Dicionario(_) => "dicionario",
            Valor::Funcao(_) | Valor::Nativa(_) => "funcao",
            Valor::Nulo => "nulo",
        }
    }

    /// Veracidade: apenas `falso` e `nulo` são falsos (§3).
    pub fn eh_verdadeiro(&self) -> bool {
        !matches!(self, Valor::Logico(false) | Valor::Nulo)
    }

    /// Representação textual do valor (usada por `escreva` e `paraTexto`).
    pub fn para_texto(&self) -> String {
        match self {
            Valor::Numero(n) => formatar_numero(*n),
            Valor::Texto(t) => t.clone(),
            Valor::Logico(b) => if *b { "verdadeiro" } else { "falso" }.to_string(),
            Valor::Nulo => "nulo".to_string(),
            Valor::Lista(itens) => {
                let partes: Vec<String> =
                    itens.borrow().iter().map(|v| v.para_texto()).collect();
                format!("[{}]", partes.join(", "))
            }
            Valor::Dicionario(mapa) => {
                let mapa = mapa.borrow();
                // ordena as chaves para uma saída estável e previsível
                let mut chaves: Vec<&String> = mapa.keys().collect();
                chaves.sort();
                let partes: Vec<String> = chaves
                    .iter()
                    .map(|c| format!("\"{}\": {}", c, mapa[*c].para_texto()))
                    .collect();
                format!("{{{}}}", partes.join(", "))
            }
            Valor::Funcao(f) => match &f.nome {
                Some(n) => format!("<funcao {}>", n),
                None => "<funcao anônima>".to_string(),
            },
            Valor::Nativa(n) => format!("<funcao embutida {}>", n.nome),
        }
    }

    /// Igualdade estrutural (para `==` e `!=`).
    pub fn igual(&self, outro: &Valor) -> bool {
        match (self, outro) {
            (Valor::Numero(a), Valor::Numero(b)) => a == b,
            (Valor::Texto(a), Valor::Texto(b)) => a == b,
            (Valor::Logico(a), Valor::Logico(b)) => a == b,
            (Valor::Nulo, Valor::Nulo) => true,
            (Valor::Lista(a), Valor::Lista(b)) => Rc::ptr_eq(a, b),
            (Valor::Dicionario(a), Valor::Dicionario(b)) => Rc::ptr_eq(a, b),
            (Valor::Funcao(a), Valor::Funcao(b)) => Rc::ptr_eq(a, b),
            (Valor::Nativa(a), Valor::Nativa(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}
