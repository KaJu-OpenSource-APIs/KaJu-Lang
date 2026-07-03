//! Árvore sintática abstrata (AST) do kaju.

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
    // operadores de bits (só entre inteiros)
    EBit,
    OuBit,
    XorBit,
    DeslocaEsq,
    DeslocaDir,
}

/// Operadores lógicos com curto-circuito.
#[derive(Clone, Debug, PartialEq)]
pub enum OpLogica {
    E,
    Ou,
    /// `??` — coalescência de nulo: devolve o operando da esquerda se ele não
    /// for nulo; caso contrário, avalia e devolve o da direita.
    CoalesceNulo,
}

/// Operadores unários.
#[derive(Clone, Debug, PartialEq)]
pub enum OpUnaria {
    Negacao,  // nao
    Negativo, // -
    NaoBit,   // ~
}

/// Uma entrada de um literal de dicionário: um par `"chave": valor` ou um
/// espalhamento `...outroDicionario`.
#[derive(Clone, Debug)]
pub enum EntradaDic {
    Par(String, Expr),
    Espalhar(Expr),
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
    Dicionario(Vec<EntradaDic>, Span),
    /// Espalhamento `...expr` — só válido dentro de literais de lista/dicionário
    /// e de listas de argumentos. Expande a coleção da direita no lugar.
    Espalhar(Box<Expr>, Span),
    Variavel(String, Span),
    Indice {
        alvo: Box<Expr>,
        indice: Box<Expr>,
        span: Span,
    },
    Fatia {
        alvo: Box<Expr>,
        /// Limites opcionais do fatiamento `alvo[inicio:fim]`. `None` significa
        /// "do começo" (início) ou "até o fim" (fim). Índices negativos contam
        /// a partir do fim.
        inicio: Option<Box<Expr>>,
        fim: Option<Box<Expr>>,
        span: Span,
    },
    Acesso {
        alvo: Box<Expr>,
        membro: String,
        /// Acesso opcional (`?.`): se `alvo` for nulo, o resultado é nulo em vez
        /// de erro (encadeamento seguro de nulos).
        opcional: bool,
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
        /// Referência à classe: um nome (`Ponto`) ou um acesso qualificado
        /// vindo de um módulo importado com `como` (`geo.Ponto`).
        classe: Box<Expr>,
        args: Vec<Expr>,
        /// Argumentos nomeados `Classe(nome: valor)`, sempre depois dos posicionais.
        nomeados: Vec<(String, Expr)>,
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
    /// Encadeamento `esq |> dir`: injeta `esq` como primeiro argumento da chamada
    /// `dir`. Se o alvo de `dir` for um nome que não é uma função em escopo, a
    /// chamada é interpretada como método (`esq.dir(...)`).
    Pipe {
        esq: Box<Expr>,
        dir: Box<Expr>,
        span: Span,
    },
    Ternario {
        condicao: Box<Expr>,
        entao: Box<Expr>,
        senao: Box<Expr>,
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
        /// Argumentos nomeados `f(nome: valor)`, sempre depois dos posicionais.
        nomeados: Vec<(String, Expr)>,
        span: Span,
    },
    FuncaoAnon {
        params: Vec<Parametro>,
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
            | Expr::Espalhar(_, s)
            | Expr::Variavel(_, s)
            | Expr::Indice { span: s, .. }
            | Expr::Fatia { span: s, .. }
            | Expr::Acesso { span: s, .. }
            | Expr::AtribIndice { span: s, .. }
            | Expr::AtribCampo { span: s, .. }
            | Expr::Novo { span: s, .. }
            | Expr::Unaria { span: s, .. }
            | Expr::Binaria { span: s, .. }
            | Expr::Logica { span: s, .. }
            | Expr::Pipe { span: s, .. }
            | Expr::Ternario { span: s, .. }
            | Expr::Atribuicao { span: s, .. }
            | Expr::Chamada { span: s, .. }
            | Expr::FuncaoAnon { span: s, .. } => s.clone(),
        }
    }
}

/// Um parâmetro de função: nome, valor padrão opcional e se é variádico (...).
#[derive(Clone, Debug)]
pub struct Parametro {
    pub nome: String,
    pub padrao: Option<Expr>,
    pub variadico: bool,
}

/// Definição de um método ou construtor dentro de uma classe.
#[derive(Clone, Debug)]
pub struct MetodoDef {
    pub nome: String,
    pub params: Vec<Parametro>,
    pub corpo: Vec<Cmd>,
}

/// Um padrão de `caso` no `escolha`.
#[derive(Clone, Debug)]
pub enum Padrao {
    /// `_` — casa com qualquer valor, sem vincular.
    Curinga,
    /// Um identificador — casa com qualquer valor e o vincula a esse nome.
    Ligar(String),
    /// Um literal (número, texto, lógico, nulo…) — casa por igualdade.
    Literal(Expr),
    /// `[p1, p2, ...resto]` — casa uma lista, vinculando sub-padrões.
    /// `resto` guarda o nome que recebe o restante (ou "_" para ignorar); `None`
    /// exige que o tamanho seja exato.
    Lista {
        elementos: Vec<Padrao>,
        resto: Option<String>,
    },
    /// `{"chave": padrao, ...}` — casa um dicionário que contenha as chaves.
    Dicionario(Vec<(String, Padrao)>),
}

/// Um ramo `caso` do `escolha`: um ou mais padrões, uma guarda opcional (`se`)
/// e o corpo a executar quando algum padrão casa e a guarda é verdadeira.
#[derive(Clone, Debug)]
pub struct CasoEscolha {
    pub padroes: Vec<Padrao>,
    pub guarda: Option<Expr>,
    pub corpo: Vec<Cmd>,
}

/// Comandos e declarações: executam ações.
// Alguns `Span` ainda não são lidos; ficam reservados para diagnósticos futuros
// de fluxo (ex.: 'pare' fora de laço, 'retorne' fora de função).
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Cmd {
    DeclClasse {
        nome: String,
        superclasse: Option<String>,
        construtor: Option<MetodoDef>,
        metodos: Vec<MetodoDef>,
        metodos_estaticos: Vec<MetodoDef>,
        campos_estaticos: Vec<(String, Expr)>,
        span: Span,
    },
    DeclVar {
        nome: String,
        valor: Expr,
        constante: bool,
        span: Span,
    },
    // desempacotamento: var a, b = 1, 2  |  var a, b = umaLista
    DeclVarMulti {
        nomes: Vec<String>,
        valores: Vec<Expr>,
        constante: bool,
        span: Span,
    },
    // reatribuição múltipla: a, b = b, a
    AtribMulti {
        nomes: Vec<String>,
        valores: Vec<Expr>,
        span: Span,
    },
    DeclFuncao {
        nome: String,
        params: Vec<Parametro>,
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
        /// Incremento por iteração; ausente equivale a passo 1. Pode ser
        /// negativo para contar de forma regressiva.
        passo: Option<Expr>,
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
    Escolha {
        valor: Expr,
        casos: Vec<CasoEscolha>,
        padrao: Option<Vec<Cmd>>,
    },
    Importe {
        caminho: String,
        alias: Option<String>,
        span: Span,
    },
    Expressao(Expr),
}
