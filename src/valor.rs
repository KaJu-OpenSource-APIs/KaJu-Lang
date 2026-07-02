//! Modelo de valores em tempo de execução da kaju.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ambiente::Ambiente;
use crate::ast::{Cmd, Parametro};

pub type ListaRef = Rc<RefCell<Vec<Valor>>>;
pub type DicRef = Rc<RefCell<HashMap<String, Valor>>>;

/// Uma função definida em kaju, com o ambiente capturado (closure).
pub struct FuncaoKaju {
    pub nome: Option<String>,
    pub params: Vec<Parametro>,
    pub corpo: Vec<Cmd>,
    pub closure: Rc<RefCell<Ambiente>>,
}

/// Uma função embutida, implementada em Rust.
pub struct Nativa {
    pub nome: String,
    pub func: fn(Vec<Valor>) -> Result<Valor, String>,
}

/// Uma classe: nome, construtor, métodos, membros estáticos e superclasse.
pub struct ClasseKaju {
    pub nome: String,
    pub construtor: Option<Rc<FuncaoKaju>>,
    pub metodos: HashMap<String, Rc<FuncaoKaju>>,
    pub metodos_estaticos: HashMap<String, Rc<FuncaoKaju>>,
    pub campos_estaticos: RefCell<HashMap<String, Valor>>,
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

    /// Procura um método estático subindo pela cadeia de herança.
    pub fn buscar_metodo_estatico(self: &Rc<Self>, nome: &str) -> Option<Rc<FuncaoKaju>> {
        if let Some(m) = self.metodos_estaticos.get(nome) {
            Some(m.clone())
        } else if let Some(sup) = &self.superclasse {
            sup.buscar_metodo_estatico(nome)
        } else {
            None
        }
    }

    /// Lê um campo estático, subindo pela cadeia de herança.
    pub fn campo_estatico(self: &Rc<Self>, nome: &str) -> Option<Valor> {
        if let Some(v) = self.campos_estaticos.borrow().get(nome) {
            Some(v.clone())
        } else if let Some(sup) = &self.superclasse {
            sup.campo_estatico(nome)
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
    // Número é um único tipo para o usuário ('numero'), mas internamente
    // distingue inteiro (i64) de decimal (f64), à la Lua 5.3.
    Inteiro(i64),
    Decimal(f64),
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

/// Formata um decimal. Valores inteiros mostram ".0" para deixar claro que
/// são decimais (ex.: 10/2 -> "5.0"), distinguindo-os de inteiros de verdade.
pub fn formatar_decimal(n: f64) -> String {
    if n.is_finite() && n == n.trunc() && n.abs() < 1e15 {
        format!("{}.0", n as i64)
    } else {
        format!("{}", n)
    }
}

impl Valor {
    /// Nome do tipo, como devolvido por `tipo(x)`.
    pub fn tipo_nome(&self) -> &'static str {
        match self {
            Valor::Inteiro(_) | Valor::Decimal(_) => "numero",
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

    /// Valor numérico como f64 (para operações/comparações mistas). None se não for número.
    pub fn como_f64(&self) -> Option<f64> {
        match self {
            Valor::Inteiro(i) => Some(*i as f64),
            Valor::Decimal(f) => Some(*f),
            _ => None,
        }
    }

    /// Representação textual do valor (usada por `escreva` e `paraTexto`).
    pub fn para_texto(&self) -> String {
        match self {
            Valor::Inteiro(i) => i.to_string(),
            Valor::Decimal(f) => formatar_decimal(*f),
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
            (Valor::Inteiro(a), Valor::Inteiro(b)) => a == b,
            // Comparação mista/decimal é matemática: 5 == 5.0 é verdadeiro.
            (Valor::Inteiro(_) | Valor::Decimal(_), Valor::Inteiro(_) | Valor::Decimal(_)) => {
                self.como_f64() == outro.como_f64()
            }
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
