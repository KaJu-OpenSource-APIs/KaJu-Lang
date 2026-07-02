//! Interpretador tree-walking: percorre a AST e executa o programa.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::ambiente::{Ambiente, ResultadoAtrib};
use crate::ast::{Cmd, Expr, OpBinaria, OpLogica, OpUnaria};
use crate::embutidos;
use crate::erros::{sugerir_nome, Diagnostico};
use crate::metodos;
use crate::token::Span;
use crate::valor::{FuncaoKaju, Valor};

/// Sinal de controle de fluxo propagado ao executar comandos.
enum Fluxo {
    Segue,
    Retorna(Valor),
    Pare,
    Continue,
}

pub struct Interpretador {
    global: Rc<RefCell<Ambiente>>,
}

impl Interpretador {
    pub fn novo() -> Self {
        let global = Ambiente::global();
        embutidos::registrar(&global);
        Interpretador { global }
    }

    /// Executa um programa inteiro.
    pub fn executar_programa(&mut self, programa: &[Cmd]) -> Result<(), Diagnostico> {
        let amb = self.global.clone();
        for cmd in programa {
            self.executar(cmd, &amb)?;
        }
        Ok(())
    }

    fn executar_bloco(
        &mut self,
        cmds: &[Cmd],
        amb: &Rc<RefCell<Ambiente>>,
    ) -> Result<Fluxo, Diagnostico> {
        for cmd in cmds {
            match self.executar(cmd, amb)? {
                Fluxo::Segue => {}
                outro => return Ok(outro),
            }
        }
        Ok(Fluxo::Segue)
    }

    fn executar(&mut self, cmd: &Cmd, amb: &Rc<RefCell<Ambiente>>) -> Result<Fluxo, Diagnostico> {
        match cmd {
            Cmd::DeclVar {
                nome,
                valor,
                constante,
                ..
            } => {
                let v = self.avaliar(valor, amb)?;
                amb.borrow_mut().definir(nome.clone(), v, *constante);
                Ok(Fluxo::Segue)
            }
            Cmd::DeclFuncao {
                nome,
                params,
                corpo,
                ..
            } => {
                let f = Valor::Funcao(Rc::new(FuncaoKaju {
                    nome: Some(nome.clone()),
                    params: params.clone(),
                    corpo: corpo.clone(),
                    closure: amb.clone(),
                }));
                amb.borrow_mut().definir(nome.clone(), f, false);
                Ok(Fluxo::Segue)
            }
            Cmd::Expressao(e) => {
                self.avaliar(e, amb)?;
                Ok(Fluxo::Segue)
            }
            Cmd::Se {
                condicao,
                entao,
                senao,
            } => {
                let cond = self.avaliar(condicao, amb)?;
                if cond.eh_verdadeiro() {
                    let filho = Ambiente::com_pai(amb.clone());
                    self.executar_bloco(entao, &filho)
                } else if let Some(bloco_senao) = senao {
                    let filho = Ambiente::com_pai(amb.clone());
                    self.executar_bloco(bloco_senao, &filho)
                } else {
                    Ok(Fluxo::Segue)
                }
            }
            Cmd::Enquanto { condicao, corpo } => {
                while self.avaliar(condicao, amb)?.eh_verdadeiro() {
                    let filho = Ambiente::com_pai(amb.clone());
                    match self.executar_bloco(corpo, &filho)? {
                        Fluxo::Segue | Fluxo::Continue => {}
                        Fluxo::Pare => break,
                        Fluxo::Retorna(v) => return Ok(Fluxo::Retorna(v)),
                    }
                }
                Ok(Fluxo::Segue)
            }
            Cmd::ParaNumerico {
                variavel,
                de,
                ate,
                corpo,
            } => {
                let de_v = self.numero(de, amb, "o início do laço 'para'")?;
                let ate_v = self.numero(ate, amb, "o fim do laço 'para'")?;
                let mut i = de_v;
                while i <= ate_v {
                    let filho = Ambiente::com_pai(amb.clone());
                    filho.borrow_mut().definir(variavel.clone(), Valor::Numero(i), false);
                    match self.executar_bloco(corpo, &filho)? {
                        Fluxo::Segue | Fluxo::Continue => {}
                        Fluxo::Pare => break,
                        Fluxo::Retorna(v) => return Ok(Fluxo::Retorna(v)),
                    }
                    i += 1.0;
                }
                Ok(Fluxo::Segue)
            }
            Cmd::ParaCada {
                variavel,
                iteravel,
                corpo,
            } => {
                let v = self.avaliar(iteravel, amb)?;
                // Instantâneo dos itens a percorrer. Em dicionários, itera pelas
                // chaves (ordenadas, para ser previsível).
                let copia: Vec<Valor> = match v {
                    Valor::Lista(l) => l.borrow().clone(),
                    Valor::Dicionario(d) => {
                        let mut chaves: Vec<String> = d.borrow().keys().cloned().collect();
                        chaves.sort();
                        chaves.into_iter().map(Valor::Texto).collect()
                    }
                    outro => {
                        return Err(Diagnostico::novo(
                            "K202",
                            format!(
                                "só é possível usar 'para cada' com listas ou dicionários, mas recebi um '{}'",
                                outro.tipo_nome()
                            ),
                            iteravel.span(),
                        )
                        .com_rotulo("isto não é iterável"))
                    }
                };
                for item in copia {
                    let filho = Ambiente::com_pai(amb.clone());
                    filho.borrow_mut().definir(variavel.clone(), item, false);
                    match self.executar_bloco(corpo, &filho)? {
                        Fluxo::Segue | Fluxo::Continue => {}
                        Fluxo::Pare => break,
                        Fluxo::Retorna(v) => return Ok(Fluxo::Retorna(v)),
                    }
                }
                Ok(Fluxo::Segue)
            }
            Cmd::Retorne(expr, _) => {
                let v = match expr {
                    Some(e) => self.avaliar(e, amb)?,
                    None => Valor::Nulo,
                };
                Ok(Fluxo::Retorna(v))
            }
            Cmd::Pare(_) => Ok(Fluxo::Pare),
            Cmd::Continue(_) => Ok(Fluxo::Continue),
        }
    }

    /// Avalia uma expressão exigindo que o resultado seja um `numero`.
    fn numero(
        &mut self,
        expr: &Expr,
        amb: &Rc<RefCell<Ambiente>>,
        contexto: &str,
    ) -> Result<f64, Diagnostico> {
        match self.avaliar(expr, amb)? {
            Valor::Numero(n) => Ok(n),
            outro => Err(Diagnostico::novo(
                "K205",
                format!("{} precisa ser um 'numero', mas é um '{}'", contexto, outro.tipo_nome()),
                expr.span(),
            )
            .com_rotulo("esperava um 'numero' aqui")),
        }
    }

    // ---- Avaliação de expressões ----

    fn avaliar(&mut self, expr: &Expr, amb: &Rc<RefCell<Ambiente>>) -> Result<Valor, Diagnostico> {
        match expr {
            Expr::Numero(n, _) => Ok(Valor::Numero(*n)),
            Expr::Texto(t, _) => Ok(Valor::Texto(t.clone())),
            Expr::Booleano(b, _) => Ok(Valor::Logico(*b)),
            Expr::Nulo(_) => Ok(Valor::Nulo),
            Expr::Lista(itens, _) => {
                let mut vs = Vec::with_capacity(itens.len());
                for it in itens {
                    vs.push(self.avaliar(it, amb)?);
                }
                Ok(Valor::Lista(Rc::new(RefCell::new(vs))))
            }
            Expr::Dicionario(pares, _) => {
                let mut mapa = HashMap::with_capacity(pares.len());
                for (chave, vexpr) in pares {
                    let v = self.avaliar(vexpr, amb)?;
                    mapa.insert(chave.clone(), v);
                }
                Ok(Valor::Dicionario(Rc::new(RefCell::new(mapa))))
            }
            Expr::Indice { alvo, indice, span } => {
                let base = self.avaliar(alvo, amb)?;
                let idx = self.avaliar(indice, amb)?;
                self.indexar(base, idx, span)
            }
            Expr::AtribIndice {
                alvo,
                indice,
                valor,
                span,
            } => {
                let base = self.avaliar(alvo, amb)?;
                let idx = self.avaliar(indice, amb)?;
                let v = self.avaliar(valor, amb)?;
                self.atribuir_indice(base, idx, v, span)
            }
            Expr::Variavel(nome, span) => match amb.borrow().obter(nome) {
                Some(v) => Ok(v),
                None => Err(self.erro_var_indefinida(nome, span, amb)),
            },
            Expr::Unaria { op, expr, span } => {
                let v = self.avaliar(expr, amb)?;
                self.aplicar_unaria(op, v, span)
            }
            Expr::Binaria { op, esq, dir, span } => {
                let a = self.avaliar(esq, amb)?;
                let b = self.avaliar(dir, amb)?;
                self.aplicar_binaria(op, a, b, span)
            }
            Expr::Logica { op, esq, dir, .. } => {
                let a = self.avaliar(esq, amb)?;
                match op {
                    OpLogica::E => {
                        if !a.eh_verdadeiro() {
                            Ok(Valor::Logico(false))
                        } else {
                            let b = self.avaliar(dir, amb)?;
                            Ok(Valor::Logico(b.eh_verdadeiro()))
                        }
                    }
                    OpLogica::Ou => {
                        if a.eh_verdadeiro() {
                            Ok(Valor::Logico(true))
                        } else {
                            let b = self.avaliar(dir, amb)?;
                            Ok(Valor::Logico(b.eh_verdadeiro()))
                        }
                    }
                }
            }
            Expr::Atribuicao { nome, valor, span } => {
                let v = self.avaliar(valor, amb)?;
                match amb.borrow_mut().atribuir(nome, v.clone()) {
                    ResultadoAtrib::Ok => Ok(v),
                    ResultadoAtrib::Constante => Err(Diagnostico::novo(
                        "K009",
                        format!("não é possível reatribuir a constante '{}'", nome),
                        span.clone(),
                    )
                    .com_rotulo("esta é uma constante")
                    .com_ajuda("declare com 'var' em vez de 'constante' se precisar alterá-la")),
                    ResultadoAtrib::NaoExiste => Err(self.erro_var_indefinida(nome, span, amb)
                        .com_ajuda(format!(
                            "para criar uma variável nova, use 'var {} = ...'",
                            nome
                        ))),
                }
            }
            Expr::Chamada { alvo, args, span } => {
                // Chamada de método: `receptor.membro(args)`
                if let Expr::Acesso {
                    alvo: receptor,
                    membro,
                    ..
                } = alvo.as_ref()
                {
                    let recv = self.avaliar(receptor, amb)?;
                    let mut vals = Vec::with_capacity(args.len());
                    for a in args {
                        vals.push(self.avaliar(a, amb)?);
                    }
                    return metodos::chamar_metodo(recv, membro, vals).map_err(|msg| {
                        Diagnostico::novo("K212", msg, span.clone())
                            .com_rotulo("nesta chamada de método")
                    });
                }
                // Chamada normal de função
                let f = self.avaliar(alvo, amb)?;
                let mut vals = Vec::with_capacity(args.len());
                for a in args {
                    vals.push(self.avaliar(a, amb)?);
                }
                self.chamar(f, vals, span)
            }
            Expr::Acesso { membro, span, .. } => Err(Diagnostico::novo(
                "K211",
                format!("'{}' só pode ser usado como método, chamando-o com ()", membro),
                span.clone(),
            )
            .com_rotulo("falta chamar o método")
            .com_ajuda(format!("use '.{}(...)' para chamar o método", membro))),
            Expr::FuncaoAnon { params, corpo, .. } => Ok(Valor::Funcao(Rc::new(FuncaoKaju {
                nome: None,
                params: params.clone(),
                corpo: corpo.clone(),
                closure: amb.clone(),
            }))),
        }
    }

    fn chamar(&mut self, alvo: Valor, args: Vec<Valor>, span: &Span) -> Result<Valor, Diagnostico> {
        match alvo {
            Valor::Funcao(f) => {
                if args.len() != f.params.len() {
                    let nome = f.nome.clone().unwrap_or_else(|| "a função".to_string());
                    return Err(Diagnostico::novo(
                        "K201",
                        format!(
                            "'{}' espera {} argumento(s), mas recebeu {}",
                            nome,
                            f.params.len(),
                            args.len()
                        ),
                        span.clone(),
                    )
                    .com_rotulo("número de argumentos incorreto"));
                }
                let escopo = Ambiente::com_pai(f.closure.clone());
                for (nome, valor) in f.params.iter().zip(args.into_iter()) {
                    escopo.borrow_mut().definir(nome.clone(), valor, false);
                }
                match self.executar_bloco(&f.corpo, &escopo)? {
                    Fluxo::Retorna(v) => Ok(v),
                    _ => Ok(Valor::Nulo),
                }
            }
            Valor::Nativa(n) => (n.func)(args).map_err(|msg| {
                Diagnostico::novo("K203", msg, span.clone()).com_rotulo("nesta chamada")
            }),
            outro => Err(Diagnostico::novo(
                "K204",
                format!("não é possível chamar um '{}' como função", outro.tipo_nome()),
                span.clone(),
            )
            .com_rotulo("isto não é uma função")),
        }
    }

    fn aplicar_unaria(
        &self,
        op: &OpUnaria,
        v: Valor,
        span: &Span,
    ) -> Result<Valor, Diagnostico> {
        match op {
            OpUnaria::Negacao => Ok(Valor::Logico(!v.eh_verdadeiro())),
            OpUnaria::Negativo => match v {
                Valor::Numero(n) => Ok(Valor::Numero(-n)),
                outro => Err(Diagnostico::novo(
                    "K012",
                    format!("não é possível aplicar '-' a um '{}'", outro.tipo_nome()),
                    span.clone(),
                )
                .com_rotulo("esperava um 'numero' aqui")),
            },
        }
    }

    fn aplicar_binaria(
        &self,
        op: &OpBinaria,
        a: Valor,
        b: Valor,
        span: &Span,
    ) -> Result<Valor, Diagnostico> {
        use OpBinaria::*;
        match op {
            Soma => match (&a, &b) {
                (Valor::Numero(x), Valor::Numero(y)) => Ok(Valor::Numero(x + y)),
                // '+' concatena quando qualquer lado é texto (§4.1)
                (Valor::Texto(_), _) | (_, Valor::Texto(_)) => {
                    Ok(Valor::Texto(format!("{}{}", a.para_texto(), b.para_texto())))
                }
                _ => Err(self.erro_tipos("+", &a, &b, span)),
            },
            Subtracao => self.aritmetica(&a, &b, span, "-", |x, y| x - y),
            Multiplicacao => self.aritmetica(&a, &b, span, "*", |x, y| x * y),
            Divisao => self.divisao(&a, &b, span, false),
            Resto => self.divisao(&a, &b, span, true),
            Menor => self.comparar(&a, &b, span, "<", |o| o.is_lt()),
            Maior => self.comparar(&a, &b, span, ">", |o| o.is_gt()),
            MenorIgual => self.comparar(&a, &b, span, "<=", |o| o.is_le()),
            MaiorIgual => self.comparar(&a, &b, span, ">=", |o| o.is_ge()),
            Igual => Ok(Valor::Logico(a.igual(&b))),
            Diferente => Ok(Valor::Logico(!a.igual(&b))),
        }
    }

    fn aritmetica(
        &self,
        a: &Valor,
        b: &Valor,
        span: &Span,
        simbolo: &str,
        f: impl Fn(f64, f64) -> f64,
    ) -> Result<Valor, Diagnostico> {
        match (a, b) {
            (Valor::Numero(x), Valor::Numero(y)) => Ok(Valor::Numero(f(*x, *y))),
            _ => Err(self.erro_tipos(simbolo, a, b, span)),
        }
    }

    fn divisao(
        &self,
        a: &Valor,
        b: &Valor,
        span: &Span,
        resto: bool,
    ) -> Result<Valor, Diagnostico> {
        match (a, b) {
            (Valor::Numero(x), Valor::Numero(y)) => {
                if *y == 0.0 {
                    return Err(Diagnostico::novo("K020", "divisão por zero", span.clone())
                        .com_rotulo("o divisor vale 0 neste ponto")
                        .com_nota("a divisão por zero não é definida em kaju."));
                }
                Ok(Valor::Numero(if resto { x % y } else { x / y }))
            }
            _ => Err(self.erro_tipos(if resto { "%" } else { "/" }, a, b, span)),
        }
    }

    fn comparar(
        &self,
        a: &Valor,
        b: &Valor,
        span: &Span,
        simbolo: &str,
        f: impl Fn(std::cmp::Ordering) -> bool,
    ) -> Result<Valor, Diagnostico> {
        match (a, b) {
            (Valor::Numero(x), Valor::Numero(y)) => {
                match x.partial_cmp(y) {
                    Some(ord) => Ok(Valor::Logico(f(ord))),
                    None => Ok(Valor::Logico(false)),
                }
            }
            _ => Err(self.erro_tipos(simbolo, a, b, span)),
        }
    }

    // ---- Indexação ----

    fn indexar(&self, base: Valor, idx: Valor, span: &Span) -> Result<Valor, Diagnostico> {
        match base {
            Valor::Lista(l) => {
                let i = self.indice_lista(&idx, span)?;
                let lista = l.borrow();
                lista.get(i).cloned().ok_or_else(|| {
                    Diagnostico::novo(
                        "K206",
                        format!(
                            "índice {} fora da lista (tamanho {})",
                            i,
                            lista.len()
                        ),
                        span.clone(),
                    )
                    .com_rotulo("este índice não existe")
                })
            }
            Valor::Texto(t) => {
                let i = self.indice_lista(&idx, span)?;
                t.chars().nth(i).map(|c| Valor::Texto(c.to_string())).ok_or_else(|| {
                    Diagnostico::novo(
                        "K206",
                        format!("índice {} fora do texto", i),
                        span.clone(),
                    )
                    .com_rotulo("este índice não existe")
                })
            }
            Valor::Dicionario(d) => {
                let chave = self.chave_dic(&idx, span)?;
                d.borrow().get(&chave).cloned().ok_or_else(|| {
                    Diagnostico::novo(
                        "K208",
                        format!("a chave \"{}\" não existe no dicionário", chave),
                        span.clone(),
                    )
                    .com_rotulo("chave inexistente")
                })
            }
            outro => Err(Diagnostico::novo(
                "K209",
                format!("não é possível indexar um '{}' com []", outro.tipo_nome()),
                span.clone(),
            )
            .com_rotulo("só listas, textos e dicionários aceitam []")),
        }
    }

    fn atribuir_indice(
        &self,
        base: Valor,
        idx: Valor,
        valor: Valor,
        span: &Span,
    ) -> Result<Valor, Diagnostico> {
        match base {
            Valor::Lista(l) => {
                let i = self.indice_lista(&idx, span)?;
                let mut lista = l.borrow_mut();
                if i >= lista.len() {
                    return Err(Diagnostico::novo(
                        "K206",
                        format!("índice {} fora da lista (tamanho {})", i, lista.len()),
                        span.clone(),
                    )
                    .com_rotulo("não é possível atribuir fora dos limites"));
                }
                lista[i] = valor.clone();
                Ok(valor)
            }
            Valor::Dicionario(d) => {
                let chave = self.chave_dic(&idx, span)?;
                d.borrow_mut().insert(chave, valor.clone());
                Ok(valor)
            }
            outro => Err(Diagnostico::novo(
                "K209",
                format!("não é possível atribuir a um índice de '{}'", outro.tipo_nome()),
                span.clone(),
            )
            .com_rotulo("apenas listas e dicionários aceitam atribuição por []")),
        }
    }

    /// Converte um valor em índice de lista/texto (inteiro não negativo).
    fn indice_lista(&self, idx: &Valor, span: &Span) -> Result<usize, Diagnostico> {
        match idx {
            Valor::Numero(n) if n.fract() == 0.0 && *n >= 0.0 => Ok(*n as usize),
            Valor::Numero(_) => Err(Diagnostico::novo(
                "K207",
                "o índice de uma lista deve ser um número inteiro não negativo",
                span.clone(),
            )
            .com_rotulo("índice inválido")),
            outro => Err(Diagnostico::novo(
                "K207",
                format!("o índice deve ser um 'numero', mas é um '{}'", outro.tipo_nome()),
                span.clone(),
            )
            .com_rotulo("esperava um 'numero' aqui")),
        }
    }

    /// Converte um valor em chave de dicionário (texto).
    fn chave_dic(&self, idx: &Valor, span: &Span) -> Result<String, Diagnostico> {
        match idx {
            Valor::Texto(t) => Ok(t.clone()),
            outro => Err(Diagnostico::novo(
                "K210",
                format!("a chave de um dicionário deve ser um 'texto', mas é um '{}'", outro.tipo_nome()),
                span.clone(),
            )
            .com_rotulo("esperava um 'texto' aqui")),
        }
    }

    // ---- Construtores de erro ----

    fn erro_tipos(&self, simbolo: &str, a: &Valor, b: &Valor, span: &Span) -> Diagnostico {
        Diagnostico::novo(
            "K012",
            format!(
                "operação '{}' não se aplica entre '{}' e '{}'",
                simbolo,
                a.tipo_nome(),
                b.tipo_nome()
            ),
            span.clone(),
        )
        .com_nota(format!(
            "o operador '{}' só funciona entre dois valores do tipo 'numero'.",
            simbolo
        ))
    }

    fn erro_var_indefinida(
        &self,
        nome: &str,
        span: &Span,
        amb: &Rc<RefCell<Ambiente>>,
    ) -> Diagnostico {
        let mut diag = Diagnostico::novo(
            "K001",
            format!("a variável '{}' não foi definida", nome),
            span.clone(),
        )
        .com_rotulo("não existe nenhuma variável com este nome");

        let nomes = amb.borrow().nomes_disponiveis();
        if let Some(sugestao) = sugerir_nome(nome, &nomes) {
            diag = diag.com_ajuda(format!("você quis dizer '{}'?", sugestao));
        }
        diag
    }
}
