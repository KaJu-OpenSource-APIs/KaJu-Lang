//! Diagnósticos ricos em português, no espírito dos erros do compilador Rust (§11).
//!
//! Um `Diagnostico` separa a estrutura do erro da sua renderização. A renderização
//! desenha a moldura com `-->`, `|` e `^^^^` apontando o trecho exato, além de
//! `nota:` e `ajuda:` opcionais.

use crate::token::Span;

/// Um diagnóstico de erro pronto para ser exibido ao usuário.
#[derive(Clone, Debug)]
pub struct Diagnostico {
    /// Código do erro, ex.: "K001".
    pub codigo: String,
    /// Mensagem curta do cabeçalho.
    pub mensagem: String,
    /// Onde o erro acontece.
    pub span: Span,
    /// Texto exibido junto ao `^^^^`.
    pub rotulo: Option<String>,
    /// Explicação do porquê (opcional).
    pub nota: Option<String>,
    /// Sugestão de correção (opcional).
    pub ajuda: Option<String>,
}

impl Diagnostico {
    pub fn novo(codigo: &str, mensagem: impl Into<String>, span: Span) -> Self {
        Diagnostico {
            codigo: codigo.to_string(),
            mensagem: mensagem.into(),
            span,
            rotulo: None,
            nota: None,
            ajuda: None,
        }
    }

    pub fn com_rotulo(mut self, rotulo: impl Into<String>) -> Self {
        self.rotulo = Some(rotulo.into());
        self
    }

    pub fn com_nota(mut self, nota: impl Into<String>) -> Self {
        self.nota = Some(nota.into());
        self
    }

    pub fn com_ajuda(mut self, ajuda: impl Into<String>) -> Self {
        self.ajuda = Some(ajuda.into());
        self
    }

    /// Renderiza o diagnóstico como texto, usando o código-fonte para exibir a linha.
    pub fn render(&self, arquivo: &str, fonte: &str) -> String {
        let linhas: Vec<&str> = fonte.lines().collect();
        let num_linha = self.span.linha;
        let largura_gutter = num_linha.to_string().len().max(1);
        let recuo = " ".repeat(largura_gutter);

        let mut saida = String::new();

        // Cabeçalho: erro[Kxxx]: mensagem
        saida.push_str(&format!("erro[{}]: {}\n", self.codigo, self.mensagem));
        // Localização: --> arquivo:linha:coluna
        saida.push_str(&format!(
            "{}--> {}:{}:{}\n",
            recuo, arquivo, self.span.linha, self.span.coluna
        ));
        saida.push_str(&format!("{} |\n", recuo));

        // Linha do código-fonte (se existir).
        if num_linha >= 1 && num_linha <= linhas.len() {
            let texto_linha = linhas[num_linha - 1];
            saida.push_str(&format!(
                "{:>largura$} | {}\n",
                num_linha,
                texto_linha,
                largura = largura_gutter
            ));

            // Linha dos ^^^^ apontando a coluna.
            let espacos = " ".repeat(self.span.coluna.saturating_sub(1));
            let setas = "^".repeat(self.span.comprimento.max(1));
            let rotulo = self
                .rotulo
                .as_ref()
                .map(|r| format!(" {}", r))
                .unwrap_or_default();
            saida.push_str(&format!("{} | {}{}{}\n", recuo, espacos, setas, rotulo));
        }

        saida.push_str(&format!("{} |\n", recuo));

        if let Some(nota) = &self.nota {
            saida.push_str(&format!("nota: {}\n", nota));
        }
        if let Some(ajuda) = &self.ajuda {
            saida.push_str(&format!("ajuda: {}\n", ajuda));
        }

        saida
    }
}

/// Distância de edição de Levenshtein — usada para sugerir "você quis dizer X?".
pub fn distancia_edicao(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut anterior: Vec<usize> = (0..=b.len()).collect();
    let mut atual = vec![0usize; b.len() + 1];

    for i in 1..=a.len() {
        atual[0] = i;
        for j in 1..=b.len() {
            let custo = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            atual[j] = (anterior[j] + 1)
                .min(atual[j - 1] + 1)
                .min(anterior[j - 1] + custo);
        }
        std::mem::swap(&mut anterior, &mut atual);
    }
    anterior[b.len()]
}

/// Dado um nome desconhecido e os nomes disponíveis, sugere o mais próximo (se houver).
pub fn sugerir_nome<'a>(alvo: &str, candidatos: &'a [String]) -> Option<&'a String> {
    candidatos
        .iter()
        .map(|c| (distancia_edicao(alvo, c), c))
        // só sugere se for razoavelmente próximo
        .filter(|(d, _)| *d > 0 && *d <= (alvo.chars().count() / 2).max(2))
        .min_by_key(|(d, _)| *d)
        .map(|(_, c)| c)
}
