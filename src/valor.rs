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

/// Uma classe: nome, construtor, métodos e superclasse opcional.
pub struct ClasseKaju {
    pub nome: String,
    pub construtor: Option<Rc<FuncaoKaju>>,
    pub metodos: HashMap<String, Rc<FuncaoKaju>>,
    pub superclasse: Option<Rc<ClasseKaju>>,
}

impl ClasseKaju {
    /// Procura um método na classe e, se não achar, sobe pela superclasse.
    /// Devolve o método e a classe onde ele foi encontrado (para o 'base').
    pub fn buscar_metodo(self: &Rc<Self>, nome: &str) -> Option<(Rc<FuncaoKaju>, Rc<ClasseKaju>)> {
        if let Some(m) = self.metodos.get(nome) {
            Some((m.clone(), self.clone()))
        } else if let Some(sup) = &self.superclasse {
            sup.buscar_metodo(nome)
        } else {
            None
        }
    }

    /// Procura o construtor mais próximo na cadeia de herança.
    pub fn buscar_construtor(self: &Rc<Self>) -> Option<(Rc<FuncaoKaju>, Rc<ClasseKaju>)> {
        if let Some(c) = &self.construtor {
            Some((c.clone(), self.clone()))
        } else if let Some(sup) = &self.superclasse {
            sup.buscar_construtor()
        } else {
            None
        }
    }
}

/// Uma instância de uma classe.
pub struct Objeto {
    pub classe: Rc<ClasseKaju>,
    pub campos: HashMap<String, Valor>,
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
    Classe(Rc<ClasseKaju>),
    Objeto(Rc<RefCell<Objeto>>),
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
            Valor::Classe(_) => "classe",
            Valor::Objeto(_) => "objeto",
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
            Valor::Classe(c) => format!("<classe {}>", c.nome),
            Valor::Objeto(o) => format!("<objeto {}>", o.borrow().classe.nome),
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
            (Valor::Classe(a), Valor::Classe(b)) => Rc::ptr_eq(a, b),
            (Valor::Objeto(a), Valor::Objeto(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}
