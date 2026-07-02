//! Interpretador tree-walking: percorre a AST e executa o programa.

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

use crate::ambiente::{Ambiente, ResultadoAtrib};
use crate::ast::{Cmd, Expr, OpBinaria, OpLogica, OpUnaria};
use crate::embutidos;
use crate::erros::{sugerir_nome, Diagnostico};
use crate::lexer::Lexer;
use crate::metodos;
use crate::parser::Parser;
use crate::token::Span;
use crate::valor::{ClasseKaju, FuncaoKaju, Objeto, Valor};

/// Sinal de controle de fluxo propagado ao executar comandos.
enum Fluxo {
    Segue,
    Retorna(Valor),
    Pare,
    Continue,
}

pub struct Interpretador {
    global: Rc<RefCell<Ambiente>>,
    /// Classe embutida usada para embrulhar erros capturados por `tente`.
    classe_erro: Rc<ClasseKaju>,
    /// Classe embutida usada para o objeto-namespace de `importe ... como`.
    classe_modulo: Rc<ClasseKaju>,
    /// Diretório-base para resolver caminhos de `importe`.
    base_dir: PathBuf,
    /// Cache de módulos já carregados (por caminho canônico).
    modulos: HashMap<PathBuf, Rc<RefCell<Ambiente>>>,
}

impl Interpretador {
    pub fn novo() -> Self {
        Self::com_base(PathBuf::from("."))
    }

    /// Cria o interpretador resolvendo `importe` relativo a `base_dir`.
    pub fn com_base(base_dir: PathBuf) -> Self {
        let global = Ambiente::global();
        embutidos::registrar(&global);
        let classe_erro = Rc::new(ClasseKaju {
            nome: "Erro".to_string(),
            construtor: None,
            metodos: HashMap::new(),
            superclasse: None,
        });
        let classe_modulo = Rc::new(ClasseKaju {
            nome: "Modulo".to_string(),
            construtor: None,
            metodos: HashMap::new(),
            superclasse: None,
        });
        // Torna a classe Erro visível para o usuário também.
        global
            .borrow_mut()
            .definir("Erro", Valor::Classe(classe_erro.clone()), false);
        Interpretador {
            global,
            classe_erro,
            classe_modulo,
            base_dir,
            modulos: HashMap::new(),
        }
    }

    /// Carrega (ou reaproveita do cache) o ambiente de um módulo.
    fn carregar_modulo(
        &mut self,
        caminho: &str,
        span: &Span,
    ) -> Result<Rc<RefCell<Ambiente>>, Diagnostico> {
        let completo = self.base_dir.join(caminho);
        let canonico = std::fs::canonicalize(&completo).unwrap_or_else(|_| completo.clone());

        if let Some(env) = self.modulos.get(&canonico) {
            return Ok(env.clone());
        }

        let fonte = std::fs::read_to_string(&completo).map_err(|e| {
            Diagnostico::novo(
                "K220",
                format!("não consegui importar \"{}\": {}", caminho, e),
                span.clone(),
            )
            .com_rotulo("não foi possível abrir o arquivo")
        })?;

        let tokens = Lexer::novo(&fonte)
            .tokenizar()
            .map_err(|d| self.envolver_erro_modulo(caminho, &d, span))?;
        let programa = Parser::novo(tokens)
            .analisar()
            .map_err(|ds| self.envolver_erro_modulo(caminho, &ds[0], span))?;

        // O módulo roda num escopo filho do global; guardamos no cache ANTES de
        // executar, para importações circulares não entrarem em loop.
        let modulo_env = Ambiente::com_pai(self.global.clone());
        self.modulos.insert(canonico, modulo_env.clone());

        // Enquanto executa o módulo, resolvemos importes dele relativos à pasta dele.
        let nova_base = completo
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| PathBuf::from("."));
        let base_antiga = std::mem::replace(&mut self.base_dir, nova_base);

        let mut resultado = Ok(());
        for cmd in &programa {
            if let Err(d) = self.executar(cmd, &modulo_env) {
                resultado = Err(self.envolver_erro_modulo(caminho, &d, span));
                break;
            }
        }

        self.base_dir = base_antiga;
        resultado?;
        Ok(modulo_env)
    }

    /// Embrulha um erro ocorrido dentro de um módulo, apontando para o `importe`.
    fn envolver_erro_modulo(&self, caminho: &str, interno: &Diagnostico, span: &Span) -> Diagnostico {
        Diagnostico::novo(
            "K221",
            format!("erro ao importar \"{}\"", caminho),
            span.clone(),
        )
        .com_rotulo("falha neste importe")
        .com_nota(format!(
            "dentro do módulo, linha {}: [{}] {}",
            interno.span.linha, interno.codigo, interno.mensagem
        ))
    }

    /// Constrói o objeto de erro que será passado ao `capture (erro)`.
    fn valor_de_erro(&self, diag: Diagnostico) -> Valor {
        // Se o usuário lançou um objeto, entrega o próprio objeto.
        if let Some(Valor::Objeto(o)) = &diag.valor_lancado {
            return Valor::Objeto(o.clone());
        }
        // Caso contrário, embrulha num objeto da classe Erro com mensagem/codigo.
        let mut campos = HashMap::new();
        campos.insert("mensagem".to_string(), Valor::Texto(diag.mensagem.clone()));
        campos.insert("codigo".to_string(), Valor::Texto(diag.codigo.clone()));
        Valor::Objeto(Rc::new(RefCell::new(Objeto {
            classe: self.classe_erro.clone(),
            campos,
        })))
    }

    /// Executa um programa inteiro.
    pub fn executar_programa(&mut self, programa: &[Cmd]) -> Result<(), Diagnostico> {
        let amb = self.global.clone();
        for cmd in programa {
            self.executar(cmd, &amb)?;
        }
        Ok(())
    }

    /// Executa uma entrada do REPL. Se o último comando for uma expressão,
    /// devolve seu valor formatado para ser ecoado (None se for `nulo`, para
    /// não poluir a saída após chamadas como `escreva(...)`).
    pub fn executar_repl(&mut self, programa: &[Cmd]) -> Result<Option<String>, Diagnostico> {
        let amb = self.global.clone();
        let ultimo = programa.len().saturating_sub(1);
        for (i, cmd) in programa.iter().enumerate() {
            if i == ultimo {
                if let Cmd::Expressao(e) = cmd {
                    let v = self.avaliar(e, &amb)?;
                    return Ok(match v {
                        Valor::Nulo => None,
                        outro => Some(outro.para_texto()),
                    });
                }
            }
            self.executar(cmd, &amb)?;
        }
        Ok(None)
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
            Cmd::DeclClasse {
                nome,
                superclasse,
                construtor,
                metodos,
                span,
            } => {
                // Resolve a superclasse, se houver.
                let super_rc = match superclasse {
                    Some(nome_sup) => match amb.borrow().obter(nome_sup) {
                        Some(Valor::Classe(c)) => Some(c),
                        Some(_) => {
                            return Err(Diagnostico::novo(
                                "K216",
                                format!("'{}' não é uma classe", nome_sup),
                                span.clone(),
                            )
                            .com_rotulo("só é possível herdar de uma classe"))
                        }
                        None => {
                            return Err(Diagnostico::novo(
                                "K216",
                                format!("a superclasse '{}' não foi definida", nome_sup),
                                span.clone(),
                            )
                            .com_rotulo("esta classe não existe"))
                        }
                    },
                    None => None,
                };

                let criar_funcao = |def: &crate::ast::MetodoDef| {
                    Rc::new(FuncaoKaju {
                        nome: Some(def.nome.clone()),
                        params: def.params.clone(),
                        corpo: def.corpo.clone(),
                        closure: amb.clone(),
                    })
                };

                let construtor_rc = construtor.as_ref().map(&criar_funcao);
                let mut mapa_metodos = HashMap::new();
                for def in metodos {
                    mapa_metodos.insert(def.nome.clone(), criar_funcao(def));
                }

                let classe = Rc::new(ClasseKaju {
                    nome: nome.clone(),
                    construtor: construtor_rc,
                    metodos: mapa_metodos,
                    superclasse: super_rc,
                });
                amb.borrow_mut().definir(nome.clone(), Valor::Classe(classe), false);
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
                // Se ambos os limites são inteiros, a variável do laço é inteira.
                let inteiros = matches!(de_v, Valor::Inteiro(_)) && matches!(ate_v, Valor::Inteiro(_));
                let inicio = de_v.como_f64().unwrap();
                let fim = ate_v.como_f64().unwrap();
                let mut passo = inicio;
                while passo <= fim {
                    let valor = if inteiros {
                        Valor::Inteiro(passo as i64)
                    } else {
                        Valor::Decimal(passo)
                    };
                    let filho = Ambiente::com_pai(amb.clone());
                    filho.borrow_mut().definir(variavel.clone(), valor, false);
                    match self.executar_bloco(corpo, &filho)? {
                        Fluxo::Segue | Fluxo::Continue => {}
                        Fluxo::Pare => break,
                        Fluxo::Retorna(v) => return Ok(Fluxo::Retorna(v)),
                    }
                    passo += 1.0;
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
            Cmd::Importe {
                caminho,
                alias,
                span,
            } => {
                let modulo_env = self.carregar_modulo(caminho, span)?;
                let exports = modulo_env.borrow().exportar();
                match alias {
                    Some(nome) => {
                        // cria um objeto-namespace: u.membro acessa os exports
                        let mut campos = HashMap::new();
                        for (n, v) in exports {
                            campos.insert(n, v);
                        }
                        let obj = Valor::Objeto(Rc::new(RefCell::new(Objeto {
                            classe: self.classe_modulo.clone(),
                            campos,
                        })));
                        amb.borrow_mut().definir(nome.clone(), obj, false);
                    }
                    None => {
                        // traz todos os nomes públicos para o escopo atual
                        for (n, v) in exports {
                            amb.borrow_mut().definir(n, v, false);
                        }
                    }
                }
                Ok(Fluxo::Segue)
            }
            Cmd::Escolha {
                valor,
                casos,
                padrao,
            } => {
                let v = self.avaliar(valor, amb)?;
                for (valores, corpo) in casos {
                    let mut casou = false;
                    for ve in valores {
                        if self.avaliar(ve, amb)?.igual(&v) {
                            casou = true;
                            break;
                        }
                    }
                    if casou {
                        let filho = Ambiente::com_pai(amb.clone());
                        return self.executar_bloco(corpo, &filho);
                    }
                }
                match padrao {
                    Some(corpo) => {
                        let filho = Ambiente::com_pai(amb.clone());
                        self.executar_bloco(corpo, &filho)
                    }
                    None => Ok(Fluxo::Segue),
                }
            }
            Cmd::Lance(expr, span) => {
                let v = self.avaliar(expr, amb)?;
                let mensagem = match &v {
                    Valor::Texto(t) => t.clone(),
                    Valor::Objeto(o) => match o.borrow().campos.get("mensagem") {
                        Some(Valor::Texto(m)) => m.clone(),
                        _ => v.para_texto(),
                    },
                    outro => outro.para_texto(),
                };
                Err(Diagnostico::lancado(mensagem, span.clone(), v))
            }
            Cmd::Tente {
                corpo,
                erro_nome,
                captura,
                finalmente,
            } => {
                // Executa o corpo num escopo próprio.
                let escopo_corpo = Ambiente::com_pai(amb.clone());
                let resultado = self.executar_bloco(corpo, &escopo_corpo);

                // Se deu erro, executa o bloco 'capture' com o erro ligado.
                let resultado = match resultado {
                    Ok(fluxo) => Ok(fluxo),
                    Err(diag) => {
                        let valor_erro = self.valor_de_erro(diag);
                        let escopo_cap = Ambiente::com_pai(amb.clone());
                        escopo_cap
                            .borrow_mut()
                            .definir(erro_nome.clone(), valor_erro, false);
                        self.executar_bloco(captura, &escopo_cap)
                    }
                };

                // O bloco 'finalmente' sempre roda.
                if let Some(bloco_final) = finalmente {
                    let escopo_final = Ambiente::com_pai(amb.clone());
                    match self.executar_bloco(bloco_final, &escopo_final)? {
                        Fluxo::Segue => {}
                        // um retorno/pare/continue no 'finalmente' tem prioridade
                        outro => return Ok(outro),
                    }
                }

                resultado
            }
        }
    }

    /// Avalia uma expressão exigindo que o resultado seja um `numero`.
    fn numero(
        &mut self,
        expr: &Expr,
        amb: &Rc<RefCell<Ambiente>>,
        contexto: &str,
    ) -> Result<Valor, Diagnostico> {
        let v = self.avaliar(expr, amb)?;
        if v.como_f64().is_some() {
            Ok(v)
        } else {
            Err(Diagnostico::novo(
                "K205",
                format!("{} precisa ser um 'numero', mas é um '{}'", contexto, v.tipo_nome()),
                expr.span(),
            )
            .com_rotulo("esperava um 'numero' aqui"))
        }
    }

    // ---- Avaliação de expressões ----

    fn avaliar(&mut self, expr: &Expr, amb: &Rc<RefCell<Ambiente>>) -> Result<Valor, Diagnostico> {
        match expr {
            Expr::Inteiro(n, _) => Ok(Valor::Inteiro(*n)),
            Expr::Decimal(n, _) => Ok(Valor::Decimal(*n)),
            Expr::Texto(t, _) => Ok(Valor::Texto(t.clone())),
            Expr::Booleano(b, _) => Ok(Valor::Logico(*b)),
            Expr::Nulo(_) => Ok(Valor::Nulo),
            Expr::Isto(span) => amb.borrow().obter("isto").ok_or_else(|| {
                Diagnostico::novo(
                    "K214",
                    "'isto' só pode ser usado dentro de um método",
                    span.clone(),
                )
                .com_rotulo("fora de um método aqui")
            }),
            Expr::Base(span) => Err(Diagnostico::novo(
                "K215",
                "'base' só pode ser usado para chamar um método da superclasse",
                span.clone(),
            )
            .com_rotulo("use 'base.metodo(...)'")),
            Expr::Novo { classe, args, span } => self.instanciar(classe, args, amb, span),
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
            Expr::Ternario {
                condicao,
                entao,
                senao,
                ..
            } => {
                if self.avaliar(condicao, amb)?.eh_verdadeiro() {
                    self.avaliar(entao, amb)
                } else {
                    self.avaliar(senao, amb)
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
                    let mut vals = Vec::with_capacity(args.len());
                    for a in args {
                        vals.push(self.avaliar(a, amb)?);
                    }
                    // base.metodo(...) — chamada à superclasse
                    if let Expr::Base(_) = receptor.as_ref() {
                        return self.chamar_base(membro, vals, amb, span);
                    }
                    let recv = self.avaliar(receptor, amb)?;
                    // Métodos de ordem superior de lista precisam chamar funções kaju,
                    // então são tratados aqui (onde há acesso ao interpretador).
                    if let Valor::Lista(l) = &recv {
                        if matches!(membro.as_str(), "mapeie" | "filtre" | "reduza" | "ordenePor") {
                            return self.metodo_lista_superior(l.clone(), membro, vals, span);
                        }
                    }
                    return match recv {
                        Valor::Objeto(obj) => self.chamar_metodo_objeto(obj, membro, vals, span),
                        outro => metodos::chamar_metodo(outro, membro, vals).map_err(|msg| {
                            Diagnostico::novo("K212", msg, span.clone())
                                .com_rotulo("nesta chamada de método")
                        }),
                    };
                }
                // Chamada normal de função
                let f = self.avaliar(alvo, amb)?;
                let mut vals = Vec::with_capacity(args.len());
                for a in args {
                    vals.push(self.avaliar(a, amb)?);
                }
                self.chamar(f, vals, span)
            }
            Expr::Acesso { alvo, membro, span } => {
                let base = self.avaliar(alvo, amb)?;
                match base {
                    Valor::Objeto(obj) => {
                        if let Some(v) = obj.borrow().campos.get(membro) {
                            return Ok(v.clone());
                        }
                        // não é campo: talvez seja um método usado sem ()
                        if obj.borrow().classe.buscar_metodo(membro).is_some() {
                            Err(Diagnostico::novo(
                                "K211",
                                format!("'{}' é um método, chame-o com ()", membro),
                                span.clone(),
                            )
                            .com_ajuda(format!("use '.{}(...)'", membro)))
                        } else {
                            Err(Diagnostico::novo(
                                "K213",
                                format!(
                                    "o objeto da classe '{}' não tem o campo ou método '{}'",
                                    obj.borrow().classe.nome,
                                    membro
                                ),
                                span.clone(),
                            )
                            .com_rotulo("membro inexistente"))
                        }
                    }
                    _ => Err(Diagnostico::novo(
                        "K211",
                        format!("'{}' só pode ser usado como método, chamando-o com ()", membro),
                        span.clone(),
                    )
                    .com_rotulo("falta chamar o método")
                    .com_ajuda(format!("use '.{}(...)' para chamar o método", membro))),
                }
            }
            Expr::AtribCampo {
                alvo,
                membro,
                valor,
                span,
            } => {
                let base = self.avaliar(alvo, amb)?;
                let v = self.avaliar(valor, amb)?;
                match base {
                    Valor::Objeto(obj) => {
                        obj.borrow_mut().campos.insert(membro.clone(), v.clone());
                        Ok(v)
                    }
                    outro => Err(Diagnostico::novo(
                        "K217",
                        format!("não é possível atribuir um campo em '{}'", outro.tipo_nome()),
                        span.clone(),
                    )
                    .com_rotulo("só objetos têm campos")),
                }
            }
            Expr::FuncaoAnon { params, corpo, .. } => Ok(Valor::Funcao(Rc::new(FuncaoKaju {
                nome: None,
                params: params.clone(),
                corpo: corpo.clone(),
                closure: amb.clone(),
            }))),
        }
    }

    /// Liga os argumentos aos parâmetros no `escopo`, tratando valores padrão
    /// (avaliados em `env_padrao`) e o parâmetro variádico (coleta o resto).
    fn vincular_args(
        &mut self,
        nome_fn: &str,
        params: &[crate::ast::Parametro],
        args: Vec<Valor>,
        escopo: &Rc<RefCell<Ambiente>>,
        env_padrao: &Rc<RefCell<Ambiente>>,
        span: &Span,
    ) -> Result<(), Diagnostico> {
        let tem_var = params.last().map(|p| p.variadico).unwrap_or(false);
        let fixos = params.len() - if tem_var { 1 } else { 0 };
        let obrig = params.iter().take(fixos).filter(|p| p.padrao.is_none()).count();

        if args.len() < obrig || (!tem_var && args.len() > fixos) {
            let msg = if tem_var {
                format!("'{}' espera pelo menos {} argumento(s), mas recebeu {}", nome_fn, obrig, args.len())
            } else if obrig == fixos {
                format!("'{}' espera {} argumento(s), mas recebeu {}", nome_fn, fixos, args.len())
            } else {
                format!("'{}' espera de {} a {} argumento(s), mas recebeu {}", nome_fn, obrig, fixos, args.len())
            };
            return Err(Diagnostico::novo("K201", msg, span.clone())
                .com_rotulo("número de argumentos incorreto"));
        }

        let mut it = args.into_iter();
        for p in params.iter().take(fixos) {
            let valor = match it.next() {
                Some(v) => v,
                None => match &p.padrao {
                    Some(expr) => self.avaliar(expr, env_padrao)?,
                    None => Valor::Nulo,
                },
            };
            escopo.borrow_mut().definir(p.nome.clone(), valor, false);
        }
        if tem_var {
            let resto: Vec<Valor> = it.collect();
            escopo
                .borrow_mut()
                .definir(params[fixos].nome.clone(), Valor::Lista(Rc::new(RefCell::new(resto))), false);
        }
        Ok(())
    }

    fn chamar(&mut self, alvo: Valor, args: Vec<Valor>, span: &Span) -> Result<Valor, Diagnostico> {
        match alvo {
            Valor::Funcao(f) => {
                let nome = f.nome.clone().unwrap_or_else(|| "a função".to_string());
                let escopo = Ambiente::com_pai(f.closure.clone());
                self.vincular_args(&nome, &f.params, args, &escopo, &f.closure, span)?;
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

    /// Métodos de lista que recebem uma função: mapeie, filtre, reduza.
    fn metodo_lista_superior(
        &mut self,
        lista: crate::valor::ListaRef,
        nome: &str,
        args: Vec<Valor>,
        span: &Span,
    ) -> Result<Valor, Diagnostico> {
        let itens: Vec<Valor> = lista.borrow().clone();
        match nome {
            "mapeie" => {
                let f = self.arg_funcao(nome, &args, 0, 1, span)?;
                let mut saida = Vec::with_capacity(itens.len());
                for item in itens {
                    saida.push(self.chamar(f.clone(), vec![item], span)?);
                }
                Ok(Valor::Lista(Rc::new(RefCell::new(saida))))
            }
            "filtre" => {
                let f = self.arg_funcao(nome, &args, 0, 1, span)?;
                let mut saida = Vec::new();
                for item in itens {
                    if self.chamar(f.clone(), vec![item.clone()], span)?.eh_verdadeiro() {
                        saida.push(item);
                    }
                }
                Ok(Valor::Lista(Rc::new(RefCell::new(saida))))
            }
            "reduza" => {
                // reduza(inicial, funcao) -> acc = funcao(acc, item)
                self.checar_aridade_metodo(nome, &args, 2, span)?;
                let mut acc = args[0].clone();
                let f = self.como_funcao(nome, &args[1], span)?;
                for item in itens {
                    acc = self.chamar(f.clone(), vec![acc, item], span)?;
                }
                Ok(acc)
            }
            "ordenePor" => {
                // ordena in-place usando f(item) como chave de ordenação
                let f = self.arg_funcao(nome, &args, 0, 1, span)?;
                let mut pares: Vec<(Valor, Valor)> = Vec::with_capacity(itens.len());
                for item in itens {
                    let chave = self.chamar(f.clone(), vec![item.clone()], span)?;
                    pares.push((chave, item));
                }
                pares.sort_by(|a, b| Self::ordem_chaves(&a.0, &b.0));
                let ordenada: Vec<Valor> = pares.into_iter().map(|(_, it)| it).collect();
                *lista.borrow_mut() = ordenada;
                Ok(Valor::Nulo)
            }
            _ => unreachable!(),
        }
    }

    /// Ordem entre chaves de ordenação (números por valor, textos alfabético).
    fn ordem_chaves(a: &Valor, b: &Valor) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match (a, b) {
            (Valor::Texto(x), Valor::Texto(y)) => x.cmp(y),
            _ => match (a.como_f64(), b.como_f64()) {
                (Some(x), Some(y)) => x.partial_cmp(&y).unwrap_or(Ordering::Equal),
                _ => Ordering::Equal,
            },
        }
    }

    fn checar_aridade_metodo(
        &self,
        nome: &str,
        args: &[Valor],
        esperado: usize,
        span: &Span,
    ) -> Result<(), Diagnostico> {
        if args.len() != esperado {
            Err(Diagnostico::novo(
                "K212",
                format!(
                    "o método '{}' espera {} argumento(s), mas recebeu {}",
                    nome,
                    esperado,
                    args.len()
                ),
                span.clone(),
            )
            .com_rotulo("número de argumentos incorreto"))
        } else {
            Ok(())
        }
    }

    /// Verifica aridade e extrai o argumento `i` como função.
    fn arg_funcao(
        &self,
        nome: &str,
        args: &[Valor],
        i: usize,
        aridade: usize,
        span: &Span,
    ) -> Result<Valor, Diagnostico> {
        self.checar_aridade_metodo(nome, args, aridade, span)?;
        self.como_funcao(nome, &args[i], span)
    }

    fn como_funcao(&self, nome: &str, v: &Valor, span: &Span) -> Result<Valor, Diagnostico> {
        match v {
            Valor::Funcao(_) | Valor::Nativa(_) => Ok(v.clone()),
            outro => Err(Diagnostico::novo(
                "K212",
                format!(
                    "'{}' espera uma função, mas recebeu um '{}'",
                    nome,
                    outro.tipo_nome()
                ),
                span.clone(),
            )
            .com_rotulo("esperava uma função aqui")),
        }
    }

    // ---- Orientação a objetos ----

    /// Cria uma instância de uma classe, rodando o construtor se houver.
    fn instanciar(
        &mut self,
        nome: &str,
        args_expr: &[Expr],
        amb: &Rc<RefCell<Ambiente>>,
        span: &Span,
    ) -> Result<Valor, Diagnostico> {
        let classe = match amb.borrow().obter(nome) {
            Some(Valor::Classe(c)) => c,
            Some(_) => {
                return Err(Diagnostico::novo(
                    "K218",
                    format!("'{}' não é uma classe", nome),
                    span.clone(),
                )
                .com_rotulo("só é possível usar 'novo' com uma classe"))
            }
            None => {
                return Err(Diagnostico::novo(
                    "K218",
                    format!("a classe '{}' não foi definida", nome),
                    span.clone(),
                )
                .com_rotulo("esta classe não existe"))
            }
        };

        let obj = Rc::new(RefCell::new(Objeto {
            classe: classe.clone(),
            campos: HashMap::new(),
        }));
        let valor_obj = Valor::Objeto(obj);

        let mut vals = Vec::with_capacity(args_expr.len());
        for a in args_expr {
            vals.push(self.avaliar(a, amb)?);
        }

        match classe.buscar_construtor() {
            Some((ctor, classe_ctor)) => {
                self.invocar_metodo(ctor, valor_obj.clone(), classe_ctor, vals, span)?;
            }
            None if !vals.is_empty() => {
                return Err(Diagnostico::novo(
                    "K201",
                    format!(
                        "a classe '{}' não tem construtor, mas recebeu {} argumento(s)",
                        nome,
                        vals.len()
                    ),
                    span.clone(),
                )
                .com_rotulo("remova os argumentos ou defina um 'construtor'"));
            }
            None => {}
        }

        Ok(valor_obj)
    }

    /// Chama um método de um objeto, subindo pela cadeia de herança.
    fn chamar_metodo_objeto(
        &mut self,
        obj: Rc<RefCell<Objeto>>,
        nome: &str,
        args: Vec<Valor>,
        span: &Span,
    ) -> Result<Valor, Diagnostico> {
        let classe = obj.borrow().classe.clone();
        if let Some((metodo, classe_do_metodo)) = classe.buscar_metodo(nome) {
            return self.invocar_metodo(metodo, Valor::Objeto(obj), classe_do_metodo, args, span);
        }
        // Fallback: campo que guarda uma função (usado por 'importe ... como').
        let campo = obj.borrow().campos.get(nome).cloned();
        if let Some(f @ (Valor::Funcao(_) | Valor::Nativa(_))) = campo {
            return self.chamar(f, args, span);
        }
        Err(Diagnostico::novo(
            "K212",
            format!("o objeto da classe '{}' não tem o método '{}'", classe.nome, nome),
            span.clone(),
        )
        .com_rotulo("método inexistente"))
    }

    /// Chama um método (ou construtor) da superclasse via `base`.
    fn chamar_base(
        &mut self,
        membro: &str,
        args: Vec<Valor>,
        amb: &Rc<RefCell<Ambiente>>,
        span: &Span,
    ) -> Result<Valor, Diagnostico> {
        let fora_de_metodo = || {
            Diagnostico::novo(
                "K215",
                "'base' só pode ser usado dentro de um método",
                span.clone(),
            )
            .com_rotulo("fora de um método aqui")
        };
        let isto = amb.borrow().obter("isto").ok_or_else(fora_de_metodo)?;
        let classe_atual = match amb.borrow().obter("@classe") {
            Some(Valor::Classe(c)) => c,
            _ => return Err(fora_de_metodo()),
        };
        let sup = classe_atual.superclasse.clone().ok_or_else(|| {
            Diagnostico::novo(
                "K215",
                format!("a classe '{}' não tem superclasse para usar 'base'", classe_atual.nome),
                span.clone(),
            )
            .com_rotulo("esta classe não herda de ninguém")
        })?;

        let achado = if membro == "construtor" {
            sup.buscar_construtor()
        } else {
            sup.buscar_metodo(membro)
        };

        match achado {
            Some((metodo, classe_do_metodo)) => {
                self.invocar_metodo(metodo, isto, classe_do_metodo, args, span)
            }
            None => Err(Diagnostico::novo(
                "K212",
                format!("a superclasse '{}' não tem '{}'", sup.nome, membro),
                span.clone(),
            )
            .com_rotulo("membro inexistente na superclasse")),
        }
    }

    /// Executa um método com `isto` e a classe atual ligados no escopo.
    fn invocar_metodo(
        &mut self,
        metodo: Rc<FuncaoKaju>,
        isto: Valor,
        classe: Rc<ClasseKaju>,
        args: Vec<Valor>,
        span: &Span,
    ) -> Result<Valor, Diagnostico> {
        let nome = metodo.nome.clone().unwrap_or_else(|| "o método".to_string());
        let escopo = Ambiente::com_pai(metodo.closure.clone());
        {
            let mut e = escopo.borrow_mut();
            e.definir("isto", isto, true);
            e.definir("@classe", Valor::Classe(classe), true);
        }
        self.vincular_args(&nome, &metodo.params, args, &escopo, &metodo.closure, span)?;
        match self.executar_bloco(&metodo.corpo, &escopo)? {
            Fluxo::Retorna(v) => Ok(v),
            _ => Ok(Valor::Nulo),
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
                Valor::Inteiro(i) => Ok(Valor::Inteiro(i.wrapping_neg())),
                Valor::Decimal(f) => Ok(Valor::Decimal(-f)),
                outro => Err(Diagnostico::novo(
                    "K012",
                    format!("não é possível aplicar '-' a um '{}'", outro.tipo_nome()),
                    span.clone(),
                )
                .com_rotulo("esperava um 'numero' aqui")),
            },
            OpUnaria::NaoBit => match v {
                Valor::Inteiro(i) => Ok(Valor::Inteiro(!i)),
                outro => Err(Diagnostico::novo(
                    "K012",
                    format!("'~' (bits) só funciona em inteiros, mas recebeu '{}'", outro.tipo_nome()),
                    span.clone(),
                )
                .com_rotulo("esperava um inteiro aqui")),
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
            // '+' concatena quando qualquer lado é texto (§4.1); senão soma numérica.
            Soma if matches!(a, Valor::Texto(_)) || matches!(b, Valor::Texto(_)) => {
                Ok(Valor::Texto(format!("{}{}", a.para_texto(), b.para_texto())))
            }
            Soma => self.num_op(&a, &b, span, "+", |x, y| x.checked_add(y), |x, y| x + y),
            Subtracao => self.num_op(&a, &b, span, "-", |x, y| x.checked_sub(y), |x, y| x - y),
            Multiplicacao => self.num_op(&a, &b, span, "*", |x, y| x.checked_mul(y), |x, y| x * y),
            Divisao => self.divisao_real(&a, &b, span),
            Resto => self.resto(&a, &b, span),
            Menor => self.comparar(&a, &b, span, "<", |o| o.is_lt()),
            Maior => self.comparar(&a, &b, span, ">", |o| o.is_gt()),
            MenorIgual => self.comparar(&a, &b, span, "<=", |o| o.is_le()),
            MaiorIgual => self.comparar(&a, &b, span, ">=", |o| o.is_ge()),
            Igual => Ok(Valor::Logico(a.igual(&b))),
            Diferente => Ok(Valor::Logico(!a.igual(&b))),
            EBit => self.bit_op(&a, &b, span, "&", |x, y| x & y),
            OuBit => self.bit_op(&a, &b, span, "|", |x, y| x | y),
            XorBit => self.bit_op(&a, &b, span, "^", |x, y| x ^ y),
            DeslocaEsq => self.deslocar(&a, &b, span, true),
            DeslocaDir => self.deslocar(&a, &b, span, false),
        }
    }

    /// Operação de bits: exige dois inteiros.
    fn bit_op(
        &self,
        a: &Valor,
        b: &Valor,
        span: &Span,
        simbolo: &str,
        f: impl Fn(i64, i64) -> i64,
    ) -> Result<Valor, Diagnostico> {
        match (a, b) {
            (Valor::Inteiro(x), Valor::Inteiro(y)) => Ok(Valor::Inteiro(f(*x, *y))),
            _ => Err(Diagnostico::novo(
                "K012",
                format!(
                    "operação de bits '{}' só funciona entre inteiros, mas recebeu '{}' e '{}'",
                    simbolo,
                    a.tipo_nome(),
                    b.tipo_nome()
                ),
                span.clone(),
            )
            .com_rotulo("esperava inteiros aqui")),
        }
    }

    fn deslocar(&self, a: &Valor, b: &Valor, span: &Span, esquerda: bool) -> Result<Valor, Diagnostico> {
        match (a, b) {
            (Valor::Inteiro(x), Valor::Inteiro(y)) => {
                if *y < 0 {
                    return Err(Diagnostico::novo(
                        "K012",
                        "o deslocamento de bits não pode ser negativo",
                        span.clone(),
                    )
                    .com_rotulo("quantidade negativa"));
                }
                let n = (*y as u32) & 63;
                let r = if esquerda { x.wrapping_shl(n) } else { x.wrapping_shr(n) };
                Ok(Valor::Inteiro(r))
            }
            _ => Err(Diagnostico::novo(
                "K012",
                format!(
                    "deslocamento de bits só funciona entre inteiros, mas recebeu '{}' e '{}'",
                    a.tipo_nome(),
                    b.tipo_nome()
                ),
                span.clone(),
            )
            .com_rotulo("esperava inteiros aqui")),
        }
    }

    /// Operação aritmética com promoção: inteiro∘inteiro = inteiro (decimal em
    /// caso de estouro); qualquer decimal envolvido resulta em decimal.
    fn num_op(
        &self,
        a: &Valor,
        b: &Valor,
        span: &Span,
        simbolo: &str,
        fi: impl Fn(i64, i64) -> Option<i64>,
        ff: impl Fn(f64, f64) -> f64,
    ) -> Result<Valor, Diagnostico> {
        match (a, b) {
            (Valor::Inteiro(x), Valor::Inteiro(y)) => Ok(match fi(*x, *y) {
                Some(r) => Valor::Inteiro(r),
                None => Valor::Decimal(ff(*x as f64, *y as f64)), // estouro -> decimal
            }),
            _ => match (a.como_f64(), b.como_f64()) {
                (Some(x), Some(y)) => Ok(Valor::Decimal(ff(x, y))),
                _ => Err(self.erro_tipos(simbolo, a, b, span)),
            },
        }
    }

    /// Divisão real '/': sempre produz decimal (mesmo entre inteiros).
    fn divisao_real(&self, a: &Valor, b: &Valor, span: &Span) -> Result<Valor, Diagnostico> {
        match (a.como_f64(), b.como_f64()) {
            (Some(x), Some(y)) => {
                if y == 0.0 {
                    return Err(Diagnostico::novo("K020", "divisão por zero", span.clone())
                        .com_rotulo("o divisor vale 0 neste ponto")
                        .com_nota("a divisão por zero não é definida em kaju."));
                }
                Ok(Valor::Decimal(x / y))
            }
            _ => Err(self.erro_tipos("/", a, b, span)),
        }
    }

    /// Resto '%': inteiro se ambos inteiros, decimal caso contrário.
    fn resto(&self, a: &Valor, b: &Valor, span: &Span) -> Result<Valor, Diagnostico> {
        let zero = |span: &Span| {
            Diagnostico::novo("K020", "divisão por zero", span.clone())
                .com_rotulo("o divisor vale 0 neste ponto")
                .com_nota("o resto por zero não é definido em kaju.")
        };
        match (a, b) {
            (Valor::Inteiro(x), Valor::Inteiro(y)) => {
                if *y == 0 {
                    return Err(zero(span));
                }
                Ok(Valor::Inteiro(x % y))
            }
            _ => match (a.como_f64(), b.como_f64()) {
                (Some(x), Some(y)) => {
                    if y == 0.0 {
                        return Err(zero(span));
                    }
                    Ok(Valor::Decimal(x % y))
                }
                _ => Err(self.erro_tipos("%", a, b, span)),
            },
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
        let ordem = match (a, b) {
            // texto compara em ordem alfabética (lexicográfica)
            (Valor::Texto(x), Valor::Texto(y)) => x.cmp(y),
            _ => match (a.como_f64(), b.como_f64()) {
                (Some(x), Some(y)) => match x.partial_cmp(&y) {
                    Some(o) => o,
                    None => return Ok(Valor::Logico(false)),
                },
                _ => return Err(self.erro_tipos(simbolo, a, b, span)),
            },
        };
        Ok(Valor::Logico(f(ordem)))
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
        let invalido = || {
            Diagnostico::novo(
                "K207",
                "o índice deve ser um número inteiro não negativo",
                span.clone(),
            )
            .com_rotulo("índice inválido")
        };
        match idx {
            Valor::Inteiro(i) if *i >= 0 => Ok(*i as usize),
            // aceita também um decimal com valor inteiro (ex.: piso(x))
            Valor::Decimal(f) if f.fract() == 0.0 && *f >= 0.0 => Ok(*f as usize),
            Valor::Inteiro(_) | Valor::Decimal(_) => Err(invalido()),
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
