//! Analisador sintático: transforma tokens em uma AST por descida recursiva.

use crate::ast::{Cmd, Expr, MetodoDef, OpBinaria, OpLogica, OpUnaria, Parametro};
use crate::erros::Diagnostico;
use crate::lexer::Lexer;
use crate::token::{Pedaco, Span, TipoToken, Token};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    /// Profundidade de laços aninhados (para validar 'pare'/'continue').
    prof_laco: usize,
    /// Profundidade de funções aninhadas (para validar 'retorne').
    prof_funcao: usize,
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
        Parser {
            tokens,
            pos: 0,
            prof_laco: 0,
            prof_funcao: 0,
        }
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

    pub fn analisar(&mut self) -> Result<Vec<Cmd>, Vec<Diagnostico>> {
        let mut cmds = Vec::new();
        let mut erros = Vec::new();
        while !self.fim() {
            match self.declaracao() {
                Ok(cmd) => cmds.push(cmd),
                Err(diag) => {
                    erros.push(diag);
                    // limita para não inundar a saída em arquivos muito quebrados
                    if erros.len() >= 20 {
                        break;
                    }
                    self.sincronizar();
                }
            }
        }
        if erros.is_empty() {
            Ok(cmds)
        } else {
            Err(erros)
        }
    }

    /// Após um erro, descarta tokens até o começo provável da próxima
    /// declaração, para conseguir relatar mais de um erro por vez.
    fn sincronizar(&mut self) {
        // garante progresso: consome ao menos o token problemático
        if !self.fim() {
            self.avancar();
        }
        while !self.fim() {
            if matches!(
                self.atual().tipo,
                TipoToken::Var
                    | TipoToken::Constante
                    | TipoToken::Funcao
                    | TipoToken::Classe
                    | TipoToken::Importe
                    | TipoToken::Se
                    | TipoToken::Enquanto
                    | TipoToken::Para
                    | TipoToken::Retorne
                    | TipoToken::Tente
                    | TipoToken::Lance
            ) {
                return;
            }
            self.avancar();
        }
    }

    /// Analisa uma única expressão completa (usado ao interpolar textos).
    pub fn analisar_expressao(&mut self) -> Result<Expr, Diagnostico> {
        let e = self.expressao()?;
        if !self.fim() {
            return Err(Diagnostico::novo(
                "K018",
                "expressão extra na interpolação",
                self.atual().span.clone(),
            )
            .com_rotulo("sobrou isto"));
        }
        Ok(e)
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

    /// Lista de expressões separadas por vírgula (lado direito de um desempacotamento).
    fn lista_valores(&mut self) -> Result<Vec<Expr>, Diagnostico> {
        let mut valores = vec![self.expressao()?];
        while self.casar(&TipoToken::Virgula) {
            valores.push(self.expressao()?);
        }
        Ok(valores)
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

        // desempacotamento: var a, b = ...
        if self.verificar(&TipoToken::Virgula) {
            let mut nomes = vec![nome];
            while self.casar(&TipoToken::Virgula) {
                let n = self.consumir(
                    &TipoToken::Identificador(String::new()),
                    "K002",
                    "esperava o nome de uma variável".into(),
                    "nome da variável aqui".into(),
                )?;
                nomes.push(n.lexema.clone());
            }
            self.consumir(
                &TipoToken::Igual,
                "K003",
                "esperava '=' após os nomes".into(),
                "atribua os valores com '='".into(),
            )?;
            let valores = self.lista_valores()?;
            return Ok(Cmd::DeclVarMulti {
                nomes,
                valores,
                constante,
                span: inicio.span,
            });
        }

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
        let corpo = self.corpo_funcao()?;
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
        let mut metodos_estaticos = Vec::new();
        let mut campos_estaticos = Vec::new();
        while !self.verificar(&TipoToken::ChaveDir) && !self.fim() {
            match &self.atual().tipo {
                TipoToken::Construtor => {
                    self.avancar();
                    let params = self.lista_parametros()?;
                    let corpo = self.corpo_funcao()?;
                    construtor = Some(MetodoDef {
                        nome: "construtor".into(),
                        params,
                        corpo,
                    });
                }
                TipoToken::Metodo => {
                    metodos.push(self.membro_metodo()?);
                }
                // membro estático: 'estatico metodo ...' ou 'estatico nome = valor'
                TipoToken::Estatico => {
                    self.avancar();
                    if self.verificar(&TipoToken::Metodo) {
                        metodos_estaticos.push(self.membro_metodo()?);
                    } else {
                        let nome_c = self.consumir(
                            &TipoToken::Identificador(String::new()),
                            "K013",
                            "esperava o nome de um campo estático ou 'metodo'".into(),
                            "ex.: 'estatico contador = 0' ou 'estatico metodo criar() {}'".into(),
                        )?;
                        self.consumir(
                            &TipoToken::Igual,
                            "K013",
                            "esperava '=' após o nome do campo estático".into(),
                            "dê um valor inicial ao campo estático".into(),
                        )?;
                        let valor = self.expressao()?;
                        campos_estaticos.push((nome_c.lexema.clone(), valor));
                    }
                }
                _ => {
                    return Err(Diagnostico::novo(
                        "K013",
                        "dentro de uma classe só cabem 'construtor', 'metodo' e 'estatico'",
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
            metodos_estaticos,
            campos_estaticos,
            span: inicio.span,
        })
    }

    /// Analisa `metodo nome(params) { corpo }` (o token 'metodo' está no atual).
    fn membro_metodo(&mut self) -> Result<MetodoDef, Diagnostico> {
        self.avancar(); // 'metodo'
        let nome_m = self.consumir(
            &TipoToken::Identificador(String::new()),
            "K013",
            "esperava o nome do método".into(),
            "nome do método aqui".into(),
        )?;
        let params = self.lista_parametros()?;
        let corpo = self.corpo_funcao()?;
        Ok(MetodoDef {
            nome: nome_m.lexema.clone(),
            params,
            corpo,
        })
    }

    fn lista_parametros(&mut self) -> Result<Vec<Parametro>, Diagnostico> {
        self.consumir(
            &TipoToken::ParenEsq,
            "K004",
            "esperava '(' para a lista de parâmetros".into(),
            "abra os parênteses aqui".into(),
        )?;
        let mut params: Vec<Parametro> = Vec::new();
        let mut viu_padrao = false;
        if !self.verificar(&TipoToken::ParenDir) {
            loop {
                // '...' antes do nome marca parâmetro variádico (coleta o resto)
                let variadico = self.casar(&TipoToken::Reticencias);
                let p = self.consumir(
                    &TipoToken::Identificador(String::new()),
                    "K004",
                    "esperava o nome de um parâmetro".into(),
                    "nome do parâmetro aqui".into(),
                )?;
                // valor padrão opcional: nome = expr
                let padrao = if !variadico && self.casar(&TipoToken::Igual) {
                    Some(self.expressao()?)
                } else {
                    None
                };
                if padrao.is_some() {
                    viu_padrao = true;
                } else if !variadico && viu_padrao {
                    return Err(Diagnostico::novo(
                        "K004",
                        format!(
                            "o parâmetro '{}' (sem padrão) não pode vir depois de um com valor padrão",
                            p.lexema
                        ),
                        p.span.clone(),
                    )
                    .com_rotulo("dê um valor padrão a ele ou mova-o para antes"));
                }
                if variadico && !self.verificar(&TipoToken::ParenDir) {
                    return Err(Diagnostico::novo(
                        "K004",
                        "o parâmetro variádico (...) precisa ser o último",
                        self.atual().span.clone(),
                    )
                    .com_rotulo("nada pode vir depois dele"));
                }
                params.push(Parametro {
                    nome: p.lexema.clone(),
                    padrao,
                    variadico,
                });
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

    /// Analisa o corpo de um laço, marcando o contexto para 'pare'/'continue'.
    fn corpo_laco(&mut self) -> Result<Vec<Cmd>, Diagnostico> {
        self.prof_laco += 1;
        let corpo = self.bloco();
        self.prof_laco -= 1;
        corpo
    }

    /// Analisa o corpo de uma função. Marca o contexto para 'retorne' e zera o
    /// contexto de laço (um 'pare' dentro de função não afeta laços externos).
    fn corpo_funcao(&mut self) -> Result<Vec<Cmd>, Diagnostico> {
        let laco_salvo = self.prof_laco;
        self.prof_laco = 0;
        self.prof_funcao += 1;
        let corpo = self.bloco();
        self.prof_funcao -= 1;
        self.prof_laco = laco_salvo;
        corpo
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
            TipoToken::Escolha => self.cmd_escolha(),
            TipoToken::Enquanto => self.cmd_enquanto(),
            TipoToken::Para => self.cmd_para(),
            TipoToken::Tente => self.cmd_tente(),
            TipoToken::Lance => self.cmd_lance(),
            TipoToken::Retorne => self.cmd_retorne(),
            TipoToken::Pare => {
                let t = self.avancar();
                if self.prof_laco == 0 {
                    return Err(Diagnostico::novo(
                        "K016",
                        "'pare' só pode ser usado dentro de um laço",
                        t.span,
                    )
                    .com_rotulo("fora de um laço aqui")
                    .com_ajuda("'pare' interrompe um 'enquanto' ou 'para'"));
                }
                Ok(Cmd::Pare(t.span))
            }
            TipoToken::Continue => {
                let t = self.avancar();
                if self.prof_laco == 0 {
                    return Err(Diagnostico::novo(
                        "K016",
                        "'continue' só pode ser usado dentro de um laço",
                        t.span,
                    )
                    .com_rotulo("fora de um laço aqui")
                    .com_ajuda("'continue' pula para a próxima volta de um 'enquanto' ou 'para'"));
                }
                Ok(Cmd::Continue(t.span))
            }
            _ => {
                let e = self.expressao()?;
                // reatribuição múltipla: a, b = b, a
                if let Expr::Variavel(primeiro, span) = &e {
                    if self.verificar(&TipoToken::Virgula) {
                        let primeiro = primeiro.clone();
                        let span = span.clone();
                        let mut nomes = vec![primeiro];
                        while self.casar(&TipoToken::Virgula) {
                            let n = self.consumir(
                                &TipoToken::Identificador(String::new()),
                                "K022",
                                "na atribuição múltipla, os alvos devem ser nomes de variáveis".into(),
                                "esperava um nome aqui".into(),
                            )?;
                            nomes.push(n.lexema.clone());
                        }
                        self.consumir(
                            &TipoToken::Igual,
                            "K022",
                            "esperava '=' na atribuição múltipla".into(),
                            "ex.: a, b = b, a".into(),
                        )?;
                        let valores = self.lista_valores()?;
                        return Ok(Cmd::AtribMulti {
                            nomes,
                            valores,
                            span,
                        });
                    }
                }
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

    fn cmd_escolha(&mut self) -> Result<Cmd, Diagnostico> {
        self.avancar(); // 'escolha'
        let valor = self.expressao()?;
        self.consumir(
            &TipoToken::ChaveEsq,
            "K021",
            "esperava '{' para abrir o corpo do 'escolha'".into(),
            "abra o corpo com '{'".into(),
        )?;
        let mut casos: Vec<(Vec<Expr>, Vec<Cmd>)> = Vec::new();
        let mut padrao: Option<Vec<Cmd>> = None;
        while !self.verificar(&TipoToken::ChaveDir) && !self.fim() {
            if self.casar(&TipoToken::Caso) {
                let mut valores = vec![self.expressao()?];
                while self.casar(&TipoToken::Virgula) {
                    valores.push(self.expressao()?);
                }
                let corpo = self.bloco()?;
                casos.push((valores, corpo));
            } else if self.casar(&TipoToken::Padrao) {
                if padrao.is_some() {
                    return Err(Diagnostico::novo(
                        "K021",
                        "só pode haver um 'padrao' no 'escolha'",
                        self.atual().span.clone(),
                    )
                    .com_rotulo("segundo 'padrao' aqui"));
                }
                padrao = Some(self.bloco()?);
            } else {
                return Err(Diagnostico::novo(
                    "K021",
                    "dentro de 'escolha' só cabem 'caso' e 'padrao'",
                    self.atual().span.clone(),
                )
                .com_rotulo("não esperava isto aqui")
                .com_ajuda("use 'caso valor { ... }' ou 'padrao { ... }'"));
            }
        }
        self.consumir(
            &TipoToken::ChaveDir,
            "K021",
            "esperava '}' para fechar o 'escolha'".into(),
            "feche com '}'".into(),
        )?;
        Ok(Cmd::Escolha {
            valor,
            casos,
            padrao,
        })
    }

    fn cmd_enquanto(&mut self) -> Result<Cmd, Diagnostico> {
        self.avancar(); // 'enquanto'
        let condicao = self.expressao()?;
        let corpo = self.corpo_laco()?;
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
            let corpo = self.corpo_laco()?;
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
            let corpo = self.corpo_laco()?;
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
        if self.prof_funcao == 0 {
            return Err(Diagnostico::novo(
                "K016",
                "'retorne' só pode ser usado dentro de uma função",
                t.span,
            )
            .com_rotulo("fora de uma função aqui"));
        }
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
        let esq = self.ternario()?;

        // atribuição simples: alvo = valor
        if self.verificar(&TipoToken::Igual) {
            let igual = self.avancar();
            let valor = self.atribuicao()?;
            return self.montar_atribuicao(esq, valor, igual.span);
        }

        // atribuição composta: alvo OP= valor  ->  alvo = alvo OP valor
        if let Some(op) = self.op_composto() {
            let tok = self.avancar();
            let direita = self.atribuicao()?;
            let span = unir_span(&esq.span(), &direita.span());
            let combinado = Expr::Binaria {
                op,
                esq: Box::new(esq.clone()),
                dir: Box::new(direita),
                span,
            };
            return self.montar_atribuicao(esq, combinado, tok.span);
        }

        Ok(esq)
    }

    /// Operador de uma atribuição composta (`+=`, `-=`, ...), se houver.
    fn op_composto(&self) -> Option<OpBinaria> {
        match self.atual().tipo {
            TipoToken::MaisIgual => Some(OpBinaria::Soma),
            TipoToken::MenosIgual => Some(OpBinaria::Subtracao),
            TipoToken::EstrelaIgual => Some(OpBinaria::Multiplicacao),
            TipoToken::BarraIgual => Some(OpBinaria::Divisao),
            TipoToken::PorcentoIgual => Some(OpBinaria::Resto),
            _ => None,
        }
    }

    /// Monta o nó de atribuição conforme o alvo (variável, índice ou campo).
    fn montar_atribuicao(
        &self,
        alvo: Expr,
        valor: Expr,
        span_op: Span,
    ) -> Result<Expr, Diagnostico> {
        match alvo {
            Expr::Variavel(nome, span) => Ok(Expr::Atribuicao {
                nome,
                span: unir_span(&span, &valor.span()),
                valor: Box::new(valor),
            }),
            Expr::Indice { alvo, indice, span } => Ok(Expr::AtribIndice {
                span: unir_span(&span, &valor.span()),
                alvo,
                indice,
                valor: Box::new(valor),
            }),
            Expr::Acesso { alvo, membro, span } => Ok(Expr::AtribCampo {
                span: unir_span(&span, &valor.span()),
                alvo,
                membro,
                valor: Box::new(valor),
            }),
            _ => Err(Diagnostico::novo(
                "K007",
                "só é possível atribuir a uma variável, índice ou campo",
                span_op,
            )
            .com_rotulo("o lado esquerdo precisa ser um nome, um acesso com [] ou um campo")),
        }
    }

    /// Expressão condicional: `condicao ? entao : senao`.
    fn ternario(&mut self) -> Result<Expr, Diagnostico> {
        let condicao = self.ou_logico()?;
        if self.casar(&TipoToken::Interrogacao) {
            let entao = self.ternario()?;
            self.consumir(
                &TipoToken::DoisPontos,
                "K019",
                "esperava ':' no operador condicional".into(),
                "use 'condicao ? valorSeVerdadeiro : valorSeFalso'".into(),
            )?;
            let senao = self.ternario()?;
            let span = unir_span(&condicao.span(), &senao.span());
            return Ok(Expr::Ternario {
                condicao: Box::new(condicao),
                entao: Box::new(entao),
                senao: Box::new(senao),
                span,
            });
        }
        Ok(condicao)
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
        let mut esq = self.ou_bit()?;
        while self.verificar(&TipoToken::E) {
            self.avancar();
            let dir = self.ou_bit()?;
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

    // Operadores de bits: | (menor precedência) -> ^ -> & (maior)
    fn ou_bit(&mut self) -> Result<Expr, Diagnostico> {
        let mut esq = self.xor_bit()?;
        while self.verificar(&TipoToken::OuBit) {
            self.avancar();
            let dir = self.xor_bit()?;
            let span = unir_span(&esq.span(), &dir.span());
            esq = Expr::Binaria { op: OpBinaria::OuBit, esq: Box::new(esq), dir: Box::new(dir), span };
        }
        Ok(esq)
    }

    fn xor_bit(&mut self) -> Result<Expr, Diagnostico> {
        let mut esq = self.e_bit()?;
        while self.verificar(&TipoToken::OuExclusivo) {
            self.avancar();
            let dir = self.e_bit()?;
            let span = unir_span(&esq.span(), &dir.span());
            esq = Expr::Binaria { op: OpBinaria::XorBit, esq: Box::new(esq), dir: Box::new(dir), span };
        }
        Ok(esq)
    }

    fn e_bit(&mut self) -> Result<Expr, Diagnostico> {
        let mut esq = self.igualdade()?;
        while self.verificar(&TipoToken::EBit) {
            self.avancar();
            let dir = self.igualdade()?;
            let span = unir_span(&esq.span(), &dir.span());
            esq = Expr::Binaria { op: OpBinaria::EBit, esq: Box::new(esq), dir: Box::new(dir), span };
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
        let mut esq = self.deslocamento()?;
        loop {
            let op = match self.atual().tipo {
                TipoToken::Menor => OpBinaria::Menor,
                TipoToken::Maior => OpBinaria::Maior,
                TipoToken::MenorIgual => OpBinaria::MenorIgual,
                TipoToken::MaiorIgual => OpBinaria::MaiorIgual,
                _ => break,
            };
            self.avancar();
            let dir = self.deslocamento()?;
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

    fn deslocamento(&mut self) -> Result<Expr, Diagnostico> {
        let mut esq = self.soma()?;
        loop {
            let op = match self.atual().tipo {
                TipoToken::DeslocaEsq => OpBinaria::DeslocaEsq,
                TipoToken::DeslocaDir => OpBinaria::DeslocaDir,
                _ => break,
            };
            self.avancar();
            let dir = self.soma()?;
            let span = unir_span(&esq.span(), &dir.span());
            esq = Expr::Binaria { op, esq: Box::new(esq), dir: Box::new(dir), span };
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
            TipoToken::Til => Some(OpUnaria::NaoBit),
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

    /// Erro de uma expressão inválida dentro de uma interpolação de texto.
    fn erro_interp(&self, interno: &Diagnostico, span: &Span) -> Diagnostico {
        Diagnostico::novo(
            "K018",
            format!("erro na interpolação de texto: {}", interno.mensagem),
            span.clone(),
        )
        .com_rotulo("dentro deste texto interpolado")
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
            TipoToken::TextoInterp(pedacos) => {
                let pedacos = pedacos.clone();
                self.avancar();
                // constrói: "" + parte0 + parte1 + ... (garante semântica de texto)
                let mut acc = Expr::Texto(String::new(), tok.span.clone());
                for p in pedacos {
                    let parte = match p {
                        Pedaco::Lit(s) => Expr::Texto(s, tok.span.clone()),
                        Pedaco::Cod(src) => {
                            let toks = Lexer::novo(&src)
                                .tokenizar()
                                .map_err(|d| self.erro_interp(&d, &tok.span))?;
                            Parser::novo(toks)
                                .analisar_expressao()
                                .map_err(|d| self.erro_interp(&d, &tok.span))?
                        }
                    };
                    acc = Expr::Binaria {
                        op: OpBinaria::Soma,
                        esq: Box::new(acc),
                        dir: Box::new(parte),
                        span: tok.span.clone(),
                    };
                }
                Ok(acc)
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
                let ini = self.consumir(
                    &TipoToken::Identificador(String::new()),
                    "K014",
                    "esperava o nome da classe após 'novo'".into(),
                    "escreva o nome da classe aqui".into(),
                )?;
                // Referência à classe: nome simples ou qualificado (`geo.Ponto`).
                let mut classe = Expr::Variavel(ini.lexema.clone(), ini.span.clone());
                while self.casar(&TipoToken::Ponto) {
                    let membro = self.consumir(
                        &TipoToken::Identificador(String::new()),
                        "K011",
                        "esperava o nome da classe após '.'".into(),
                        "escreva o nome do membro aqui".into(),
                    )?;
                    let span = unir_span(&classe.span(), &membro.span);
                    classe = Expr::Acesso {
                        alvo: Box::new(classe),
                        membro: membro.lexema.clone(),
                        span,
                    };
                }
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
                    classe: Box::new(classe),
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
                let corpo = self.corpo_funcao()?;
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
