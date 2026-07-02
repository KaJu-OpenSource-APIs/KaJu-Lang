//! Árvore sintática abstrata (AST) da kaju — Fase 1.

use crate::token::Span;

/// Operadores binários aritméticos e de comparação.
#[derive(Clone, Debug, PartialEq)]
pub enum OpBinaria {
    Soma,
    Subtracao,
    Multiplicacao,
    Divisao,
    Resto,
    Igual,
    Diferente,
    Menor,
    Maior,
    MenorIgual,
    MaiorIgual,
}

/// Operadores lógicos com curto-circuito.
#[derive(Clone, Debug, PartialEq)]
pub enum OpLogica {
    E,
    Ou,
}

/// Operadores unários.
#[derive(Clone, Debug, PartialEq)]
pub enum OpUnaria {
    Negacao,   // nao
    Negativo,  // -
}

/// Expressões: produzem um valor.
#[derive(Clone, Debug)]
pub enum Expr {
    Inteiro(i64, Span),
    Decimal(f64, Span),
    Texto(String, Span),
    Booleano(bool, Span),
    Nulo(Span),
    Isto(Span),
    Base(Span),
    Lista(Vec<Expr>, Span),
    Dicionario(Vec<(String, Expr)>, Span),
    Variavel(String, Span),
    Indice {
        alvo: Box<Expr>,
        indice: Box<Expr>,
        span: Span,
    },
    Acesso {
        alvo: Box<Expr>,
        membro: String,
        span: Span,
    },
    AtribIndice {
        alvo: Box<Expr>,
        indice: Box<Expr>,
        valor: Box<Expr>,
        span: Span,
    },
    AtribCampo {
        alvo: Box<Expr>,
        membro: String,
        valor: Box<Expr>,
        span: Span,
    },
    Novo {
        classe: String,
        args: Vec<Expr>,
        span: Span,
    },
    Unaria {
        op: OpUnaria,
        expr: Box<Expr>,
        span: Span,
    },
    Binaria {
        op: OpBinaria,
        esq: Box<Expr>,
        dir: Box<Expr>,
        span: Span,
    },
    Logica {
        op: OpLogica,
        esq: Box<Expr>,
        dir: Box<Expr>,
        span: Span,
    },
    Atribuicao {
        nome: String,
        valor: Box<Expr>,
        span: Span,
    },
    Chamada {
        alvo: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
    },
    FuncaoAnon {
        params: Vec<String>,
        corpo: Vec<Cmd>,
        span: Span,
    },
}

impl Expr {
    /// O span (posição) desta expressão, para diagnósticos.
    pub fn span(&self) -> Span {
        match self {
            Expr::Inteiro(_, s)
            | Expr::Decimal(_, s)
            | Expr::Texto(_, s)
            | Expr::Booleano(_, s)
            | Expr::Nulo(s)
            | Expr::Isto(s)
            | Expr::Base(s)
            | Expr::Lista(_, s)
            | Expr::Dicionario(_, s)
            | Expr::Variavel(_, s)
            | Expr::Indice { span: s, .. }
            | Expr::Acesso { span: s, .. }
            | Expr::AtribIndice { span: s, .. }
            | Expr::AtribCampo { span: s, .. }
            | Expr::Novo { span: s, .. }
            | Expr::Unaria { span: s, .. }
            | Expr::Binaria { span: s, .. }
            | Expr::Logica { span: s, .. }
            | Expr::Atribuicao { span: s, .. }
            | Expr::Chamada { span: s, .. }
            | Expr::FuncaoAnon { span: s, .. } => s.clone(),
        }
    }
}

/// Definição de um método ou construtor dentro de uma classe.
#[derive(Clone, Debug)]
pub struct MetodoDef {
    pub nome: String,
    pub params: Vec<String>,
    pub corpo: Vec<Cmd>,
}

/// Comandos e declarações: executam ações.
// Alguns `Span` ainda não são lidos; serão usados na Fase 2 para diagnósticos
// de fluxo (ex.: 'pare' fora de laço, 'retorne' fora de função).
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Cmd {
    DeclClasse {
        nome: String,
        superclasse: Option<String>,
        construtor: Option<MetodoDef>,
        metodos: Vec<MetodoDef>,
        span: Span,
    },
    DeclVar {
        nome: String,
        valor: Expr,
        constante: bool,
        span: Span,
    },
    DeclFuncao {
        nome: String,
        params: Vec<String>,
        corpo: Vec<Cmd>,
        span: Span,
    },
    Se {
        condicao: Expr,
        entao: Vec<Cmd>,
        // `senao` pode ser outro `se` (via "senao se") ou um bloco final.
        senao: Option<Vec<Cmd>>,
    },
    Enquanto {
        condicao: Expr,
        corpo: Vec<Cmd>,
    },
    ParaNumerico {
        variavel: String,
        de: Expr,
        ate: Expr,
        corpo: Vec<Cmd>,
    },
    ParaCada {
        variavel: String,
        iteravel: Expr,
        corpo: Vec<Cmd>,
    },
    Retorne(Option<Expr>, Span),
    Pare(Span),
    Continue(Span),
    Tente {
        corpo: Vec<Cmd>,
        erro_nome: String,
        captura: Vec<Cmd>,
        finalmente: Option<Vec<Cmd>>,
    },
    Lance(Expr, Span),
    Importe {
        caminho: String,
        alias: Option<String>,
        span: Span,
    },
    Expressao(Expr),
}
