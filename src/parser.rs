//! Analisador sintático: transforma tokens em uma AST por descida recursiva.

use crate::ast::{Cmd, Expr, MetodoDef, OpBinaria, OpLogica, OpUnaria};
use crate::erros::Diagnostico;
use crate::token::{Span, TipoToken, Token};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

/// Une dois spans na mesma linha em um só (para apontar a expressão inteira).
fn unir_span(a: &Span, b: &Span) -> Span {
    if a.linha == b.linha {
        let ini = a.coluna.min(b.coluna);
        let fim = (a.coluna + a.comprimento).max(b.coluna + b.comprimento);
        Span::novo(a.linha, ini, fim - ini)
    } else {
        a.clone()
    }
}

/// Nome amigável de um tipo de token, para mensagens de erro.
fn descricao(tipo: &TipoToken) -> String {
    match tipo {
        TipoToken::ChaveEsq => "'{'".into(),
        TipoToken::ChaveDir => "'}'".into(),
        TipoToken::ParenEsq => "'('".into(),
        TipoToken::ParenDir => "')'".into(),
        TipoToken::ColcheteDir => "']'".into(),
        TipoToken::Igual => "'='".into(),
        TipoToken::Ate => "'ate'".into(),
        TipoToken::Em => "'em'".into(),
        TipoToken::De => "'de'".into(),
        TipoToken::Identificador(_) => "um nome".into(),
        TipoToken::FimDeArquivo => "o fim do arquivo".into(),
        outro => format!("{:?}", outro),
    }
}

impl Parser {
    pub fn novo(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn atual(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn fim(&self) -> bool {
        matches!(self.atual().tipo, TipoToken::FimDeArquivo)
    }

    fn avancar(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        if !self.fim() {
            self.pos += 1;
        }
        t
    }

    fn verificar(&self, tipo: &TipoToken) -> bool {
        std::mem::discriminant(&self.atual().tipo) == std::mem::discriminant(tipo)
    }

    /// Se o token atual for de `tipo`, consome e devolve `true`.
    fn casar(&mut self, tipo: &TipoToken) -> bool {
        if self.verificar(tipo) {
            self.avancar();
            true
        } else {
            false
        }
    }

    fn consumir(
        &mut self,
        tipo: &TipoToken,
        codigo: &str,
        mensagem: String,
        rotulo: String,
    ) -> Result<Token, Diagnostico> {
        if self.verificar(tipo) {
            Ok(self.avancar())
        } else {
            Err(Diagnostico::novo(codigo, mensagem, self.atual().span.clone())
                .com_rotulo(rotulo))
        }
    }

    // ---- Nível superior ----

    pub fn analisar(&mut self) -> Result<Vec<Cmd>, Diagnostico> {
        let mut cmds = Vec::new();
        while !self.fim() {
            cmds.push(self.declaracao()?);
        }
        Ok(cmds)
    }

    fn declaracao(&mut self) -> Result<Cmd, Diagnostico> {
        match &self.atual().tipo {
            TipoToken::Var => self.decl_var(false),
            TipoToken::Constante => self.decl_var(true),
            TipoToken::Importe => self.decl_importe(),
            TipoToken::Classe => self.decl_classe(),
            TipoToken::Funcao => {
                // pode ser declaração de função nomeada ou função anônima em expressão.
                if let TipoToken::Identificador(_) = self.tokens[self.pos + 1].tipo {
                    self.decl_funcao()
                } else {
                    self.comando()
                }
            }
            _ => self.comando(),
        }
    }

    fn decl_var(&mut self, constante: bool) -> Result<Cmd, Diagnostico> {
        let inicio = self.avancar(); // 'var' ou 'constante'
        let nome_tok = self.consumir(
            &TipoToken::Identificador(String::new()),
            "K002",
            "esperava o nome da variável".into(),
            "declare um nome aqui".into(),
        )?;
        let nome = nome_tok.lexema.clone();
        self.consumir(
            &TipoToken::Igual,
            "K003",
            format!("esperava '=' após o nome '{}'", nome),
            "atribua um valor com '='".into(),
        )?;
        let valor = self.expressao()?;
        let span = unir_span(&inicio.span, &valor.span());
        Ok(Cmd::DeclVar {
            nome,
            valor,
            constante,
            span,
        })
    }

    fn decl_funcao(&mut self) -> Result<Cmd, Diagnostico> {
        let inicio = self.avancar(); // 'funcao'
        let nome_tok = self.avancar(); // já sabemos que é Identificador
        let nome = nome_tok.lexema.clone();
        let params = self.lista_parametros()?;
        let corpo = self.bloco()?;
        Ok(Cmd::DeclFuncao {
            nome,
            params,
            corpo,
            span: inicio.span,
        })
    }

    fn decl_importe(&mut self) -> Result<Cmd, Diagnostico> {
        let inicio = self.avancar(); // 'importe'
        let caminho = self.consumir(
            &TipoToken::Texto(String::new()),
            "K017",
            "esperava o caminho do arquivo entre aspas".into(),
            "ex.: importe \"utilidades.kaju\"".into(),
        )?;
        let alias = if self.casar(&TipoToken::Como) {
            let nome = self.consumir(
                &TipoToken::Identificador(String::new()),
                "K017",
                "esperava um nome após 'como'".into(),
                "ex.: importe \"mat.kaju\" como mat".into(),
            )?;
            Some(nome.lexema.clone())
        } else {
            None
        };
        Ok(Cmd::Importe {
            caminho: caminho.lexema.clone(),
            alias,
            span: inicio.span,
        })
    }

    fn decl_classe(&mut self) -> Result<Cmd, Diagnostico> {
        let inicio = self.avancar(); // 'classe'
        let nome_tok = self.consumir(
            &TipoToken::Identificador(String::new()),
            "K013",
            "esperava o nome da classe".into(),
            "dê um nome à classe aqui".into(),
        )?;
        let nome = nome_tok.lexema.clone();

        let superclasse = if self.casar(&TipoToken::Herda) {
            let sup = self.consumir(
                &TipoToken::Identificador(String::new()),
                "K013",
                "esperava o nome da superclasse após 'herda'".into(),
                "nome da superclasse aqui".into(),
            )?;
            Some(sup.lexema.clone())
        } else {
            None
        };

        self.consumir(
            &TipoToken::ChaveEsq,
            "K013",
            "esperava '{' para abrir o corpo da classe".into(),
            "abra o corpo da classe com '{'".into(),
        )?;

        let mut construtor = None;
        let mut metodos = Vec::new();
        while !self.verificar(&TipoToken::ChaveDir) && !self.fim() {
            match &self.atual().tipo {
                TipoToken::Construtor => {
                    self.avancar();
                    let params = self.lista_parametros()?;
                    let corpo = self.bloco()?;
                    construtor = Some(MetodoDef {
                        nome: "construtor".into(),
                        params,
                        corpo,
                    });
                }
                TipoToken::Metodo => {
                    self.avancar();
                    let nome_m = self.consumir(
                        &TipoToken::Identificador(String::new()),
                        "K013",
                        "esperava o nome do método".into(),
                        "nome do método aqui".into(),
                    )?;
                    let params = self.lista_parametros()?;
                    let corpo = self.bloco()?;
                    metodos.push(MetodoDef {
                        nome: nome_m.lexema.clone(),
                        params,
                        corpo,
                    });
                }
                _ => {
                    return Err(Diagnostico::novo(
                        "K013",
                        "dentro de uma classe só podem existir 'construtor' e 'metodo'",
                        self.atual().span.clone(),
                    )
                    .com_rotulo("não esperava isto aqui")
                    .com_ajuda("declare um método com 'metodo nome(...) { ... }'"))
                }
            }
        }

        self.consumir(
            &TipoToken::ChaveDir,
            "K013",
            "esperava '}' para fechar a classe".into(),
            "feche a classe com '}'".into(),
        )?;

        Ok(Cmd::DeclClasse {
            nome,
            superclasse,
            construtor,
            metodos,
            span: inicio.span,
        })
    }

    fn lista_parametros(&mut self) -> Result<Vec<String>, Diagnostico> {
        self.consumir(
            &TipoToken::ParenEsq,
            "K004",
            "esperava '(' para a lista de parâmetros".into(),
            "abra os parênteses aqui".into(),
        )?;
        let mut params = Vec::new();
        if !self.verificar(&TipoToken::ParenDir) {
            loop {
                let p = self.consumir(
                    &TipoToken::Identificador(String::new()),
                    "K004",
                    "esperava o nome de um parâmetro".into(),
                    "nome do parâmetro aqui".into(),
                )?;
                params.push(p.lexema.clone());
                if !self.casar(&TipoToken::Virgula) {
                    break;
                }
            }
        }
        self.consumir(
            &TipoToken::ParenDir,
            "K004",
            "esperava ')' para fechar os parâmetros".into(),
            "feche os parênteses aqui".into(),
        )?;
        Ok(params)
    }

    fn bloco(&mut self) -> Result<Vec<Cmd>, Diagnostico> {
        self.consumir(
            &TipoToken::ChaveEsq,
            "K005",
            "esperava '{' para abrir o bloco".into(),
            "abra o bloco com '{'".into(),
        )?;
        let mut cmds = Vec::new();
        while !self.verificar(&TipoToken::ChaveDir) && !self.fim() {
            cmds.push(self.declaracao()?);
        }
        self.consumir(
            &TipoToken::ChaveDir,
            "K005",
            "esperava '}' para fechar o bloco".into(),
            "feche o bloco com '}'".into(),
        )?;
        Ok(cmds)
    }

    // ---- Comandos ----

    fn comando(&mut self) -> Result<Cmd, Diagnostico> {
        match &self.atual().tipo {
            TipoToken::Se => self.cmd_se(),
            TipoToken::Enquanto => self.cmd_enquanto(),
            TipoToken::Para => self.cmd_para(),
            TipoToken::Tente => self.cmd_tente(),
            TipoToken::Lance => self.cmd_lance(),
            TipoToken::Retorne => self.cmd_retorne(),
            TipoToken::Pare => {
                let t = self.avancar();
                Ok(Cmd::Pare(t.span))
            }
            TipoToken::Continue => {
                let t = self.avancar();
                Ok(Cmd::Continue(t.span))
            }
            _ => {
                let e = self.expressao()?;
                Ok(Cmd::Expressao(e))
            }
        }
    }

    fn cmd_se(&mut self) -> Result<Cmd, Diagnostico> {
        self.avancar(); // 'se' ou 'senaose'
        let condicao = self.expressao()?;
        let entao = self.bloco()?;
        let senao = if self.verificar(&TipoToken::SenaoSe) {
            // 'senaose' encadeia outro se (palavra única)
            Some(vec![self.cmd_se()?])
        } else if self.casar(&TipoToken::Senao) {
            if self.verificar(&TipoToken::Se) {
                // 'senao se' (duas palavras) também é aceito
                Some(vec![self.cmd_se()?])
            } else {
                Some(self.bloco()?)
            }
        } else {
            None
        };
        Ok(Cmd::Se {
            condicao,
            entao,
            senao,
        })
    }

    fn cmd_enquanto(&mut self) -> Result<Cmd, Diagnostico> {
        self.avancar(); // 'enquanto'
        let condicao = self.expressao()?;
        let corpo = self.bloco()?;
        Ok(Cmd::Enquanto { condicao, corpo })
    }

    fn cmd_para(&mut self) -> Result<Cmd, Diagnostico> {
        self.avancar(); // 'para'
        if self.casar(&TipoToken::Cada) {
            // para cada IDENT em EXPR { }
            let var = self.consumir(
                &TipoToken::Identificador(String::new()),
                "K006",
                "esperava o nome da variável do laço".into(),
                "nome da variável aqui".into(),
            )?;
            self.consumir(
                &TipoToken::Em,
                "K006",
                "esperava 'em' no laço 'para cada'".into(),
                "use 'para cada X em lista'".into(),
            )?;
            let iteravel = self.expressao()?;
            let corpo = self.bloco()?;
            Ok(Cmd::ParaCada {
                variavel: var.lexema,
                iteravel,
                corpo,
            })
        } else {
            // para IDENT de EXPR ate EXPR { }
            let var = self.consumir(
                &TipoToken::Identificador(String::new()),
                "K006",
                "esperava o nome da variável do laço".into(),
                "nome da variável aqui".into(),
            )?;
            self.consumir(
                &TipoToken::De,
                "K006",
                "esperava 'de' no laço 'para'".into(),
                "use 'para X de A ate B'".into(),
            )?;
            let de = self.expressao()?;
            self.consumir(
                &TipoToken::Ate,
                "K006",
                "esperava 'ate' no laço 'para'".into(),
                "use 'para X de A ate B'".into(),
            )?;
            let ate = self.expressao()?;
            let corpo = self.bloco()?;
            Ok(Cmd::ParaNumerico {
                variavel: var.lexema,
                de,
                ate,
                corpo,
            })
        }
    }

    fn cmd_tente(&mut self) -> Result<Cmd, Diagnostico> {
        self.avancar(); // 'tente'
        let corpo = self.bloco()?;
        self.consumir(
            &TipoToken::Capture,
            "K015",
            "esperava 'capture' após o bloco 'tente'".into(),
            "todo 'tente' precisa de um 'capture (erro) { ... }'".into(),
        )?;
        self.consumir(
            &TipoToken::ParenEsq,
            "K015",
            "esperava '(' após 'capture'".into(),
            "use 'capture (erro) { ... }'".into(),
        )?;
        let erro = self.consumir(
            &TipoToken::Identificador(String::new()),
            "K015",
            "esperava o nome da variável de erro".into(),
            "dê um nome ao erro capturado, ex.: 'erro'".into(),
        )?;
        self.consumir(
            &TipoToken::ParenDir,
            "K015",
            "esperava ')' após o nome do erro".into(),
            "feche os parênteses aqui".into(),
        )?;
        let captura = self.bloco()?;
        let finalmente = if self.casar(&TipoToken::Finalmente) {
            Some(self.bloco()?)
        } else {
            None
        };
        Ok(Cmd::Tente {
            corpo,
            erro_nome: erro.lexema.clone(),
            captura,
            finalmente,
        })
    }

    fn cmd_lance(&mut self) -> Result<Cmd, Diagnostico> {
        let t = self.avancar(); // 'lance'
        let expr = self.expressao()?;
        Ok(Cmd::Lance(expr, t.span))
    }

    fn cmd_retorne(&mut self) -> Result<Cmd, Diagnostico> {
        let t = self.avancar(); // 'retorne'
        // Sem valor se o próximo token fecha o bloco ou termina o arquivo.
        let valor = if self.verificar(&TipoToken::ChaveDir) || self.fim() {
            None
        } else {
            Some(self.expressao()?)
        };
        Ok(Cmd::Retorne(valor, t.span))
    }

    // ---- Expressões (precedência crescente) ----

    fn expressao(&mut self) -> Result<Expr, Diagnostico> {
        self.atribuicao()
    }

    fn atribuicao(&mut self) -> Result<Expr, Diagnostico> {
        let esq = self.ou_logico()?;
        if self.verificar(&TipoToken::Igual) {
            let igual = self.avancar();
            let valor = self.atribuicao()?;
            match esq {
                Expr::Variavel(nome, span) => {
                    let span_total = unir_span(&span, &valor.span());
                    Ok(Expr::Atribuicao {
                        nome,
                        valor: Box::new(valor),
                        span: span_total,
                    })
                }
                Expr::Indice {
                    alvo,
                    indice,
                    span,
                } => {
                    let span_total = unir_span(&span, &valor.span());
                    Ok(Expr::AtribIndice {
                        alvo,
                        indice,
                        valor: Box::new(valor),
                        span: span_total,
                    })
                }
                Expr::Acesso { alvo, membro, span } => {
                    let span_total = unir_span(&span, &valor.span());
                    Ok(Expr::AtribCampo {
                        alvo,
                        membro,
                        valor: Box::new(valor),
                        span: span_total,
                    })
                }
                _ => Err(Diagnostico::novo(
                    "K007",
                    "só é possível atribuir a uma variável ou a um índice",
                    igual.span,
                )
                .com_rotulo("o lado esquerdo de '=' precisa ser um nome ou um acesso com []")),
            }
        } else {
            Ok(esq)
        }
    }

    fn ou_logico(&mut self) -> Result<Expr, Diagnostico> {
        let mut esq = self.e_logico()?;
        while self.verificar(&TipoToken::Ou) {
            self.avancar();
            let dir = self.e_logico()?;
            let span = unir_span(&esq.span(), &dir.span());
            esq = Expr::Logica {
                op: OpLogica::Ou,
                esq: Box::new(esq),
                dir: Box::new(dir),
                span,
            };
        }
        Ok(esq)
    }

    fn e_logico(&mut self) -> Result<Expr, Diagnostico> {
        let mut esq = self.igualdade()?;
        while self.verificar(&TipoToken::E) {
            self.avancar();
            let dir = self.igualdade()?;
            let span = unir_span(&esq.span(), &dir.span());
            esq = Expr::Logica {
                op: OpLogica::E,
                esq: Box::new(esq),
                dir: Box::new(dir),
                span,
            };
        }
        Ok(esq)
    }

    fn igualdade(&mut self) -> Result<Expr, Diagnostico> {
        let mut esq = self.comparacao()?;
        loop {
            let op = match self.atual().tipo {
                TipoToken::IgualIgual => OpBinaria::Igual,
                TipoToken::Diferente => OpBinaria::Diferente,
                _ => break,
            };
            self.avancar();
            let dir = self.comparacao()?;
            let span = unir_span(&esq.span(), &dir.span());
            esq = Expr::Binaria {
                op,
                esq: Box::new(esq),
                dir: Box::new(dir),
                span,
            };
        }
        Ok(esq)
    }

    fn comparacao(&mut self) -> Result<Expr, Diagnostico> {
        let mut esq = self.soma()?;
        loop {
            let op = match self.atual().tipo {
                TipoToken::Menor => OpBinaria::Menor,
                TipoToken::Maior => OpBinaria::Maior,
                TipoToken::MenorIgual => OpBinaria::MenorIgual,
                TipoToken::MaiorIgual => OpBinaria::MaiorIgual,
                _ => break,
            };
            self.avancar();
            let dir = self.soma()?;
            let span = unir_span(&esq.span(), &dir.span());
            esq = Expr::Binaria {
                op,
                esq: Box::new(esq),
                dir: Box::new(dir),
                span,
            };
        }
        Ok(esq)
    }

    fn soma(&mut self) -> Result<Expr, Diagnostico> {
        let mut esq = self.produto()?;
        loop {
            let op = match self.atual().tipo {
                TipoToken::Mais => OpBinaria::Soma,
                TipoToken::Menos => OpBinaria::Subtracao,
                _ => break,
            };
            self.avancar();
            let dir = self.produto()?;
            let span = unir_span(&esq.span(), &dir.span());
            esq = Expr::Binaria {
                op,
                esq: Box::new(esq),
                dir: Box::new(dir),
                span,
            };
        }
        Ok(esq)
    }

    fn produto(&mut self) -> Result<Expr, Diagnostico> {
        let mut esq = self.unario()?;
        loop {
            let op = match self.atual().tipo {
                TipoToken::Estrela => OpBinaria::Multiplicacao,
                TipoToken::Barra => OpBinaria::Divisao,
                TipoToken::Porcento => OpBinaria::Resto,
                _ => break,
            };
            self.avancar();
            let dir = self.unario()?;
            let span = unir_span(&esq.span(), &dir.span());
            esq = Expr::Binaria {
                op,
                esq: Box::new(esq),
                dir: Box::new(dir),
                span,
            };
        }
        Ok(esq)
    }

    fn unario(&mut self) -> Result<Expr, Diagnostico> {
        let op = match self.atual().tipo {
            TipoToken::Nao => Some(OpUnaria::Negacao),
            TipoToken::Menos => Some(OpUnaria::Negativo),
            _ => None,
        };
        if let Some(op) = op {
            let tok = self.avancar();
            let expr = self.unario()?;
            let span = unir_span(&tok.span, &expr.span());
            Ok(Expr::Unaria {
                op,
                expr: Box::new(expr),
                span,
            })
        } else {
            self.chamada()
        }
    }

    fn chamada(&mut self) -> Result<Expr, Diagnostico> {
        let mut expr = self.primario()?;
        loop {
            if self.verificar(&TipoToken::ParenEsq) {
                self.avancar();
                let mut args = Vec::new();
                if !self.verificar(&TipoToken::ParenDir) {
                    loop {
                        args.push(self.expressao()?);
                        if !self.casar(&TipoToken::Virgula) {
                            break;
                        }
                    }
                }
                let fim = self.consumir(
                    &TipoToken::ParenDir,
                    "K004",
                    "esperava ')' para fechar a chamada".into(),
                    "feche os argumentos com ')'".into(),
                )?;
                let span = unir_span(&expr.span(), &fim.span);
                expr = Expr::Chamada {
                    alvo: Box::new(expr),
                    args,
                    span,
                };
            } else if self.verificar(&TipoToken::ColcheteEsq) {
                self.avancar();
                let indice = self.expressao()?;
                let fim = self.consumir(
                    &TipoToken::ColcheteDir,
                    "K004",
                    "esperava ']' para fechar a indexação".into(),
                    "feche o índice com ']'".into(),
                )?;
                let span = unir_span(&expr.span(), &fim.span);
                expr = Expr::Indice {
                    alvo: Box::new(expr),
                    indice: Box::new(indice),
                    span,
                };
            } else if self.verificar(&TipoToken::Ponto) {
                self.avancar();
                // O membro é um identificador; 'construtor' é permitido (para base.construtor()).
                let membro = if self.verificar(&TipoToken::Construtor) {
                    self.avancar()
                } else {
                    self.consumir(
                        &TipoToken::Identificador(String::new()),
                        "K011",
                        "esperava o nome de um membro após '.'".into(),
                        "escreva o nome do método ou atributo aqui".into(),
                    )?
                };
                let span = unir_span(&expr.span(), &membro.span);
                expr = Expr::Acesso {
                    alvo: Box::new(expr),
                    membro: membro.lexema.clone(),
                    span,
                };
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn primario(&mut self) -> Result<Expr, Diagnostico> {
        let tok = self.atual().clone();
        match &tok.tipo {
            TipoToken::Inteiro(n) => {
                self.avancar();
                Ok(Expr::Inteiro(*n, tok.span))
            }
            TipoToken::Decimal(n) => {
                self.avancar();
                Ok(Expr::Decimal(*n, tok.span))
            }
            TipoToken::Texto(t) => {
                self.avancar();
                Ok(Expr::Texto(t.clone(), tok.span))
            }
            TipoToken::Verdadeiro => {
                self.avancar();
                Ok(Expr::Booleano(true, tok.span))
            }
            TipoToken::Falso => {
                self.avancar();
                Ok(Expr::Booleano(false, tok.span))
            }
            TipoToken::Nulo => {
                self.avancar();
                Ok(Expr::Nulo(tok.span))
            }
            TipoToken::Isto => {
                self.avancar();
                Ok(Expr::Isto(tok.span))
            }
            TipoToken::Base => {
                self.avancar();
                Ok(Expr::Base(tok.span))
            }
            TipoToken::Novo => {
                self.avancar();
                let classe = self.consumir(
                    &TipoToken::Identificador(String::new()),
                    "K014",
                    "esperava o nome da classe após 'novo'".into(),
                    "escreva o nome da classe aqui".into(),
                )?;
                self.consumir(
                    &TipoToken::ParenEsq,
                    "K014",
                    "esperava '(' para os argumentos do construtor".into(),
                    "abra os parênteses aqui".into(),
                )?;
                let mut args = Vec::new();
                if !self.verificar(&TipoToken::ParenDir) {
                    loop {
                        args.push(self.expressao()?);
                        if !self.casar(&TipoToken::Virgula) {
                            break;
                        }
                    }
                }
                let fim = self.consumir(
                    &TipoToken::ParenDir,
                    "K014",
                    "esperava ')' para fechar os argumentos".into(),
                    "feche os parênteses aqui".into(),
                )?;
                let span = unir_span(&tok.span, &fim.span);
                Ok(Expr::Novo {
                    classe: classe.lexema.clone(),
                    args,
                    span,
                })
            }
            TipoToken::Identificador(nome) => {
                self.avancar();
                Ok(Expr::Variavel(nome.clone(), tok.span))
            }
            TipoToken::ParenEsq => {
                self.avancar();
                let e = self.expressao()?;
                self.consumir(
                    &TipoToken::ParenDir,
                    "K004",
                    "esperava ')' para fechar a expressão".into(),
                    "feche os parênteses aqui".into(),
                )?;
                Ok(e)
            }
            TipoToken::ColcheteEsq => {
                self.avancar();
                let mut itens = Vec::new();
                if !self.verificar(&TipoToken::ColcheteDir) {
                    loop {
                        itens.push(self.expressao()?);
                        if !self.casar(&TipoToken::Virgula) {
                            break;
                        }
                    }
                }
                let fim = self.consumir(
                    &TipoToken::ColcheteDir,
                    "K004",
                    "esperava ']' para fechar a lista".into(),
                    "feche a lista com ']'".into(),
                )?;
                let span = unir_span(&tok.span, &fim.span);
                Ok(Expr::Lista(itens, span))
            }
            TipoToken::ChaveEsq => {
                self.avancar();
                let mut pares: Vec<(String, Expr)> = Vec::new();
                if !self.verificar(&TipoToken::ChaveDir) {
                    loop {
                        let chave = self.consumir(
                            &TipoToken::Texto(String::new()),
                            "K010",
                            "esperava uma chave de texto no dicionário".into(),
                            "a chave deve ser um texto entre aspas".into(),
                        )?;
                        self.consumir(
                            &TipoToken::DoisPontos,
                            "K010",
                            "esperava ':' entre a chave e o valor".into(),
                            "separe chave e valor com ':'".into(),
                        )?;
                        let valor = self.expressao()?;
                        pares.push((chave.lexema.clone(), valor));
                        if !self.casar(&TipoToken::Virgula) {
                            break;
                        }
                    }
                }
                let fim = self.consumir(
                    &TipoToken::ChaveDir,
                    "K010",
                    "esperava '}' para fechar o dicionário".into(),
                    "feche o dicionário com '}'".into(),
                )?;
                let span = unir_span(&tok.span, &fim.span);
                Ok(Expr::Dicionario(pares, span))
            }
            TipoToken::Funcao => {
                self.avancar();
                let params = self.lista_parametros()?;
                let corpo = self.bloco()?;
                Ok(Expr::FuncaoAnon {
                    params,
                    corpo,
                    span: tok.span,
                })
            }
            outro => Err(Diagnostico::novo(
                "K008",
                format!("esperava uma expressão, mas encontrei {}", descricao(outro)),
                tok.span,
            )
            .com_rotulo("não esperava isto aqui")),
        }
    }
}
