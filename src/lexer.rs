//! Analisador léxico: transforma o código-fonte em uma sequência de tokens.

use crate::erros::Diagnostico;
use crate::token::{palavra_chave, Span, TipoToken, Token};

pub struct Lexer {
    fonte: Vec<char>,
    pos: usize,
    linha: usize,
    coluna: usize,
}

impl Lexer {
    pub fn novo(fonte: &str) -> Self {
        Lexer {
            fonte: fonte.chars().collect(),
            pos: 0,
            linha: 1,
            coluna: 1,
        }
    }

    fn fim(&self) -> bool {
        self.pos >= self.fonte.len()
    }

    fn atual(&self) -> char {
        self.fonte.get(self.pos).copied().unwrap_or('\0')
    }

    fn proximo(&self) -> char {
        self.fonte.get(self.pos + 1).copied().unwrap_or('\0')
    }

    /// Avança um caractere, mantendo linha/coluna corretos.
    fn avancar(&mut self) -> char {
        let c = self.atual();
        self.pos += 1;
        if c == '\n' {
            self.linha += 1;
            self.coluna = 1;
        } else {
            self.coluna += 1;
        }
        c
    }

    /// Produz todos os tokens até o fim do arquivo.
    pub fn tokenizar(&mut self) -> Result<Vec<Token>, Diagnostico> {
        let mut tokens = Vec::new();
        loop {
            self.pular_espacos_e_comentarios()?;
            if self.fim() {
                let span = Span::novo(self.linha, self.coluna, 1);
                tokens.push(Token::novo(TipoToken::FimDeArquivo, String::new(), span));
                break;
            }
            tokens.push(self.proximo_token()?);
        }
        Ok(tokens)
    }

    fn pular_espacos_e_comentarios(&mut self) -> Result<(), Diagnostico> {
        loop {
            let c = self.atual();
            if c == ' ' || c == '\t' || c == '\r' || c == '\n' {
                self.avancar();
            } else if c == '/' && self.proximo() == '/' {
                // comentário de linha
                while !self.fim() && self.atual() != '\n' {
                    self.avancar();
                }
            } else if c == '/' && self.proximo() == '*' {
                // comentário de bloco
                let ini_linha = self.linha;
                let ini_coluna = self.coluna;
                self.avancar();
                self.avancar();
                loop {
                    if self.fim() {
                        return Err(Diagnostico::novo(
                            "K102",
                            "comentário de bloco não foi fechado",
                            Span::novo(ini_linha, ini_coluna, 2),
                        )
                        .com_rotulo("este /* nunca é fechado")
                        .com_ajuda("feche o comentário com */"));
                    }
                    if self.atual() == '*' && self.proximo() == '/' {
                        self.avancar();
                        self.avancar();
                        break;
                    }
                    self.avancar();
                }
            } else {
                break;
            }
        }
        Ok(())
    }

    fn proximo_token(&mut self) -> Result<Token, Diagnostico> {
        let ini_linha = self.linha;
        let ini_coluna = self.coluna;
        let c = self.atual();

        // Texto interpolado: $"... {expr} ..."
        if c == '$' && self.proximo() == '"' {
            self.avancar(); // consome '$'
            return self.ler_texto_interp(ini_linha, ini_coluna);
        }
        // Número
        if c.is_ascii_digit() {
            return Ok(self.ler_numero(ini_linha, ini_coluna));
        }
        // Texto
        if c == '"' {
            return self.ler_texto(ini_linha, ini_coluna);
        }
        // Identificador ou palavra-chave
        if c.is_alphabetic() || c == '_' {
            return Ok(self.ler_identificador(ini_linha, ini_coluna));
        }

        // Símbolos
        self.avancar();
        let (tipo, comprimento) = match c {
            '+' => self.talvez_igual('+', TipoToken::MaisIgual, TipoToken::Mais),
            '-' => self.talvez_igual('-', TipoToken::MenosIgual, TipoToken::Menos),
            '*' => self.talvez_igual('*', TipoToken::EstrelaIgual, TipoToken::Estrela),
            '/' => self.talvez_igual('/', TipoToken::BarraIgual, TipoToken::Barra),
            '%' => self.talvez_igual('%', TipoToken::PorcentoIgual, TipoToken::Porcento),
            '&' => (TipoToken::EBit, 1),
            '|' => (TipoToken::OuBit, 1),
            '^' => (TipoToken::OuExclusivo, 1),
            '~' => (TipoToken::Til, 1),
            '(' => (TipoToken::ParenEsq, 1),
            ')' => (TipoToken::ParenDir, 1),
            '{' => (TipoToken::ChaveEsq, 1),
            '}' => (TipoToken::ChaveDir, 1),
            '[' => (TipoToken::ColcheteEsq, 1),
            ']' => (TipoToken::ColcheteDir, 1),
            ',' => (TipoToken::Virgula, 1),
            ':' => (TipoToken::DoisPontos, 1),
            '?' => (TipoToken::Interrogacao, 1),
            '.' => {
                if self.atual() == '.' && self.proximo() == '.' {
                    self.avancar();
                    self.avancar();
                    (TipoToken::Reticencias, 3)
                } else {
                    (TipoToken::Ponto, 1)
                }
            }
            '=' => {
                if self.atual() == '=' {
                    self.avancar();
                    (TipoToken::IgualIgual, 2)
                } else {
                    (TipoToken::Igual, 1)
                }
            }
            '!' => {
                if self.atual() == '=' {
                    self.avancar();
                    (TipoToken::Diferente, 2)
                } else {
                    return Err(Diagnostico::novo(
                        "K101",
                        "caractere '!' inesperado",
                        Span::novo(ini_linha, ini_coluna, 1),
                    )
                    .com_rotulo("aqui")
                    .com_ajuda("para 'diferente de', use '!='; para negação lógica, use 'nao'"));
                }
            }
            '<' => {
                if self.atual() == '=' {
                    self.avancar();
                    (TipoToken::MenorIgual, 2)
                } else if self.atual() == '<' {
                    self.avancar();
                    (TipoToken::DeslocaEsq, 2)
                } else {
                    (TipoToken::Menor, 1)
                }
            }
            '>' => {
                if self.atual() == '=' {
                    self.avancar();
                    (TipoToken::MaiorIgual, 2)
                } else if self.atual() == '>' {
                    self.avancar();
                    (TipoToken::DeslocaDir, 2)
                } else {
                    (TipoToken::Maior, 1)
                }
            }
            outro => {
                return Err(Diagnostico::novo(
                    "K101",
                    format!("caractere '{}' inesperado", outro),
                    Span::novo(ini_linha, ini_coluna, 1),
                )
                .com_rotulo("não faz parte da linguagem kaju aqui"));
            }
        };

        let span = Span::novo(ini_linha, ini_coluna, comprimento);
        Ok(Token::novo(tipo, c.to_string(), span))
    }

    /// Se o próximo caractere for '=', devolve o token composto; senão o simples.
    fn talvez_igual(
        &mut self,
        _c: char,
        composto: TipoToken,
        simples: TipoToken,
    ) -> (TipoToken, usize) {
        if self.atual() == '=' {
            self.avancar();
            (composto, 2)
        } else {
            (simples, 1)
        }
    }

    fn ler_numero(&mut self, linha: usize, coluna: usize) -> Token {
        let inicio = self.pos;
        while self.atual().is_ascii_digit() {
            self.avancar();
        }
        // parte decimal (só é decimal se houver ponto seguido de dígito)
        let mut eh_decimal = false;
        if self.atual() == '.' && self.proximo().is_ascii_digit() {
            eh_decimal = true;
            self.avancar(); // consome '.'
            while self.atual().is_ascii_digit() {
                self.avancar();
            }
        }
        let lexema: String = self.fonte[inicio..self.pos].iter().collect();
        let span = Span::novo(linha, coluna, lexema.chars().count());
        // Inteiro se não tem ponto E cabe em i64; senão vira decimal.
        let tipo = if !eh_decimal {
            match lexema.parse::<i64>() {
                Ok(i) => TipoToken::Inteiro(i),
                Err(_) => TipoToken::Decimal(lexema.parse().unwrap_or(0.0)),
            }
        } else {
            TipoToken::Decimal(lexema.parse().unwrap_or(0.0))
        };
        Token::novo(tipo, lexema, span)
    }

    fn ler_texto(&mut self, linha: usize, coluna: usize) -> Result<Token, Diagnostico> {
        self.avancar(); // consome a aspa inicial
        let mut conteudo = String::new();
        loop {
            if self.fim() || self.atual() == '\n' {
                return Err(Diagnostico::novo(
                    "K103",
                    "texto não foi fechado com aspas",
                    Span::novo(linha, coluna, 1),
                )
                .com_rotulo("o texto começa aqui e nunca fecha")
                .com_ajuda("feche o texto com \" no fim da linha"));
            }
            let c = self.avancar();
            if c == '"' {
                break;
            }
            if c == '\\' {
                // sequências de escape
                let e = self.avancar();
                match e {
                    'n' => conteudo.push('\n'),
                    't' => conteudo.push('\t'),
                    '\\' => conteudo.push('\\'),
                    '"' => conteudo.push('"'),
                    outro => {
                        conteudo.push('\\');
                        conteudo.push(outro);
                    }
                }
            } else {
                conteudo.push(c);
            }
        }
        let comprimento = self.coluna.saturating_sub(coluna);
        let span = Span::novo(linha, coluna, comprimento.max(1));
        Ok(Token::novo(
            TipoToken::Texto(conteudo.clone()),
            conteudo,
            span,
        ))
    }

    /// Lê um texto interpolado `$"...{expr}..."`, produzindo pedaços literais e
    /// de código. `{{` e `}}` viram chaves literais.
    fn ler_texto_interp(&mut self, linha: usize, coluna: usize) -> Result<Token, Diagnostico> {
        use crate::token::Pedaco;
        self.avancar(); // consome a aspa inicial
        let mut pedacos: Vec<Pedaco> = Vec::new();
        let mut lit = String::new();

        let nao_fechado = |l: usize, c: usize| {
            Diagnostico::novo("K104", "texto interpolado não foi fechado", Span::novo(l, c, 2))
                .com_rotulo("começa aqui e nunca fecha")
                .com_ajuda("feche com \" na mesma linha")
        };

        loop {
            if self.fim() || self.atual() == '\n' {
                return Err(nao_fechado(linha, coluna));
            }
            let c = self.atual();
            if c == '"' {
                self.avancar();
                break;
            }
            if c == '\\' {
                self.avancar();
                let e = self.avancar();
                match e {
                    'n' => lit.push('\n'),
                    't' => lit.push('\t'),
                    '\\' => lit.push('\\'),
                    '"' => lit.push('"'),
                    outro => {
                        lit.push('\\');
                        lit.push(outro);
                    }
                }
                continue;
            }
            if c == '{' {
                if self.proximo() == '{' {
                    self.avancar();
                    self.avancar();
                    lit.push('{');
                    continue;
                }
                self.avancar(); // consome '{'
                if !lit.is_empty() {
                    pedacos.push(Pedaco::Lit(std::mem::take(&mut lit)));
                }
                // coleta o código até o '}' correspondente (ciente de strings e aninhamento)
                let mut cod = String::new();
                let mut prof = 1;
                let mut em_texto = false;
                loop {
                    if self.fim() || self.atual() == '\n' {
                        return Err(Diagnostico::novo(
                            "K104",
                            "'{' não foi fechado na interpolação",
                            Span::novo(self.linha, self.coluna, 1),
                        )
                        .com_rotulo("faltou '}'"));
                    }
                    let d = self.atual();
                    if em_texto {
                        if d == '\\' {
                            cod.push(d);
                            self.avancar();
                            cod.push(self.atual());
                            self.avancar();
                            continue;
                        }
                        if d == '"' {
                            em_texto = false;
                        }
                    } else {
                        match d {
                            '"' => em_texto = true,
                            '{' => prof += 1,
                            '}' => {
                                prof -= 1;
                                if prof == 0 {
                                    self.avancar();
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    cod.push(d);
                    self.avancar();
                }
                pedacos.push(Pedaco::Cod(cod));
                continue;
            }
            if c == '}' {
                if self.proximo() == '}' {
                    self.avancar();
                    self.avancar();
                    lit.push('}');
                    continue;
                }
                return Err(Diagnostico::novo(
                    "K104",
                    "'}' inesperado na interpolação",
                    Span::novo(self.linha, self.coluna, 1),
                )
                .com_ajuda("para uma chave literal, use '}}'"));
            }
            lit.push(c);
            self.avancar();
        }

        if !lit.is_empty() {
            pedacos.push(Pedaco::Lit(lit));
        }
        let comprimento = self.coluna.saturating_sub(coluna).max(2);
        let span = Span::novo(linha, coluna, comprimento);
        Ok(Token::novo(TipoToken::TextoInterp(pedacos), String::new(), span))
    }

    fn ler_identificador(&mut self, linha: usize, coluna: usize) -> Token {
        let inicio = self.pos;
        while self.atual().is_alphanumeric() || self.atual() == '_' {
            self.avancar();
        }
        let lexema: String = self.fonte[inicio..self.pos].iter().collect();
        let span = Span::novo(linha, coluna, lexema.chars().count());
        let tipo = palavra_chave(&lexema)
            .unwrap_or_else(|| TipoToken::Identificador(lexema.clone()));
        Token::novo(tipo, lexema, span)
    }
}
