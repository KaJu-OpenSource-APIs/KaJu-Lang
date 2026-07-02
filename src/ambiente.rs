//! Ambiente de execução: escopos aninhados com suporte a closures.

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::valor::Valor;

/// Resultado de uma tentativa de atribuição.
pub enum ResultadoAtrib {
    Ok,
    NaoExiste,
    Constante,
}

/// Um escopo: mapeia nomes a valores (com marca de constante) e aponta para o pai.
pub struct Ambiente {
    valores: HashMap<String, (Valor, bool)>,
    pai: Option<Rc<RefCell<Ambiente>>>,
}

impl Ambiente {
    /// Cria o ambiente global (sem pai).
    pub fn global() -> Rc<RefCell<Ambiente>> {
        Rc::new(RefCell::new(Ambiente {
            valores: HashMap::new(),
            pai: None,
        }))
    }

    /// Cria um escopo filho de `pai`.
    pub fn com_pai(pai: Rc<RefCell<Ambiente>>) -> Rc<RefCell<Ambiente>> {
        Rc::new(RefCell::new(Ambiente {
            valores: HashMap::new(),
            pai: Some(pai),
        }))
    }

    /// Define (ou redefine) uma variável no escopo atual.
    pub fn definir(&mut self, nome: impl Into<String>, valor: Valor, constante: bool) {
        self.valores.insert(nome.into(), (valor, constante));
    }

    /// Busca um valor, subindo pela cadeia de escopos.
    pub fn obter(&self, nome: &str) -> Option<Valor> {
        if let Some((v, _)) = self.valores.get(nome) {
            Some(v.clone())
        } else if let Some(pai) = &self.pai {
            pai.borrow().obter(nome)
        } else {
            None
        }
    }

    /// Atribui a uma variável existente, subindo pela cadeia. Respeita constantes.
    pub fn atribuir(&mut self, nome: &str, valor: Valor) -> ResultadoAtrib {
        if let Some(slot) = self.valores.get_mut(nome) {
            if slot.1 {
                return ResultadoAtrib::Constante;
            }
            slot.0 = valor;
            return ResultadoAtrib::Ok;
        }
        if let Some(pai) = &self.pai {
            return pai.borrow_mut().atribuir(nome, valor);
        }
        ResultadoAtrib::NaoExiste
    }

    /// Todos os nomes visíveis a partir deste escopo (para sugestões de erro).
    pub fn nomes_disponiveis(&self) -> Vec<String> {
        let mut nomes: Vec<String> = self.valores.keys().cloned().collect();
        if let Some(pai) = &self.pai {
            nomes.extend(pai.borrow().nomes_disponiveis());
        }
        nomes
    }
}
