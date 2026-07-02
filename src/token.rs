//! Tokens da linguagem kaju e sua posição no código-fonte.

/// Posição de um trecho no código-fonte, usada para diagnósticos ricos (§11).
/// `linha` e `coluna` são 1-based; `comprimento` é o número de caracteres do trecho.
#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    pub linha: usize,
    pub coluna: usize,
    pub comprimento: usize,
}

impl Span {
    pub fn novo(linha: usize, coluna: usize, comprimento: usize) -> Self {
        Span { linha, coluna, comprimento }
    }
}

/// Tipo (categoria) de um token.
#[derive(Clone, Debug, PartialEq)]
pub enum TipoToken {
    // Literais
    Numero(f64),
    Texto(String),
    Identificador(String),

    // Palavras-chave da Fase 1
    Var,
    Constante,
    Funcao,
    Retorne,
    Se,
    Senao,
    Enquanto,
    Para,
    Cada,
    Em,
    De,
    Ate,
    Pare,
    Continue,
    E,
    Ou,
    Nao,
    Verdadeiro,
    Falso,
    Nulo,

    // Palavras-chave de orientação a objetos (Fase 2)
    Classe,
    Herda,
    Metodo,
    Construtor,
    Novo,
    Isto,
    Base,
    Tente,
    Capture,
    Finalmente,
    Lance,
    Importe,
    Como,

    // Símbolos
    Mais,
    Menos,
    Estrela,
    Barra,
    Porcento,
    IgualIgual,
    Diferente,
    Menor,
    Maior,
    MenorIgual,
    MaiorIgual,
    Igual,
    ParenEsq,
    ParenDir,
    ChaveEsq,
    ChaveDir,
    ColcheteEsq,
    ColcheteDir,
    Virgula,
    DoisPontos,
    Ponto,

    FimDeArquivo,
}

/// Um token: seu tipo, o texto original (lexema) e onde ele está.
#[derive(Clone, Debug)]
pub struct Token {
    pub tipo: TipoToken,
    pub lexema: String,
    pub span: Span,
}

impl Token {
    pub fn novo(tipo: TipoToken, lexema: String, span: Span) -> Self {
        Token { tipo, lexema, span }
    }
}

/// Converte um identificador em palavra-chave, se for uma.
pub fn palavra_chave(texto: &str) -> Option<TipoToken> {
    let tipo = match texto {
        "var" => TipoToken::Var,
        "constante" => TipoToken::Constante,
        "funcao" => TipoToken::Funcao,
        "retorne" => TipoToken::Retorne,
        "se" => TipoToken::Se,
        "senao" => TipoToken::Senao,
        "enquanto" => TipoToken::Enquanto,
        "para" => TipoToken::Para,
        "cada" => TipoToken::Cada,
        "em" => TipoToken::Em,
        "de" => TipoToken::De,
        "ate" => TipoToken::Ate,
        "pare" => TipoToken::Pare,
        "continue" => TipoToken::Continue,
        "e" => TipoToken::E,
        "ou" => TipoToken::Ou,
        "nao" => TipoToken::Nao,
        "verdadeiro" => TipoToken::Verdadeiro,
        "falso" => TipoToken::Falso,
        "nulo" => TipoToken::Nulo,
        "classe" => TipoToken::Classe,
        "herda" => TipoToken::Herda,
        "metodo" => TipoToken::Metodo,
        "construtor" => TipoToken::Construtor,
        "novo" => TipoToken::Novo,
        "isto" => TipoToken::Isto,
        "base" => TipoToken::Base,
        "tente" => TipoToken::Tente,
        "capture" => TipoToken::Capture,
        "finalmente" => TipoToken::Finalmente,
        "lance" => TipoToken::Lance,
        "importe" => TipoToken::Importe,
        "como" => TipoToken::Como,
        _ => return None,
    };
    Some(tipo)
}
