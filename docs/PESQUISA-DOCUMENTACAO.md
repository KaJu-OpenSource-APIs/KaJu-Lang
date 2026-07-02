# Pesquisa: como as linguagens de programação documentaram suas linguagens

> Levantamento feito para embasar a criação, do zero, da documentação da linguagem **kaju**.
> Cobre linguagens de baixo e alto nível, além de metodologias e ferramentas gerais.
> Data: 2026-07-01.

## Sumário

- [Parte 1 — Linguagens de baixo nível](#parte-1--linguagens-de-baixo-nível) (C, C++, Rust, Zig, Assembly)
- [Parte 2 — Linguagens dinâmicas de alto nível](#parte-2--linguagens-dinâmicas-de-alto-nível) (Python, JavaScript, Ruby, Lua)
- [Parte 3 — JVM e funcionais](#parte-3--jvm-e-funcionais) (Java, Go, Haskell, Elixir)
- [Parte 4 — Metodologias, frameworks e ferramentas](#parte-4--metodologias-frameworks-e-ferramentas)
- [Síntese final — recomendações para a documentação do kaju](#síntese-final--recomendações-para-a-documentação-do-kaju)

---

## Parte 1 — Linguagens de baixo nível

### C
- **Artefato fundador:** *The C Programming Language* (1978) de Kernighan & Ritchie ("K&R"). Serviu como **especificação informal**; o apêndice "C Reference Manual" era a referência de fato. O mesmo livro ensinava **e** definia a linguagem.
- **Evolução:** de spec informal para padrão *de jure* — ANSI X3.159-1989 (C89) → ISO/IEC 9899:1990 (C90) → C99, C11, C17, C23. A norma ISO é a autoridade final.
- **Estrutura:** a norma separa **cláusulas da linguagem** (sintaxe, tipos, semântica) da **biblioteca padrão**. Formaliza conceitos como *undefined behavior*, *implementation-defined behavior* e *sequence points*.
- **Ferramentas:** norma em PDF formal; docs práticas em **man pages** (troff/mandoc) e manuais GCC/glibc em **Texinfo**; cppreference cobre C também.
- **Cultura:** duas camadas — comitê **WG14** (formal, institucional) e comunidade prática (glibc, GCC, man pages POSIX).

### C++
- **Artefato fundador:** *The C++ Programming Language* (1985) de Stroustrup. Hoje a referência prática dominante é **cppreference.com** (wiki comunitária); a autoridade formal é a norma ISO.
- **Evolução:** ISO/IEC 14882:1998 (C++98) → C++03/11/14/17/20/23. Comitê **WG21** com **papers numerados** (`PxxxxRn`), sistema tipo RFC. Portal oficial: isocpp.org.
- **Estrutura:** cppreference separa **language reference** de **standard library reference**, anotando cada entrada por versão do padrão (C++11/14/17/20/23).
- **Ferramentas:** cppreference é MediaWiki com templates de versão; norma gerada de LaTeX; **Doxygen** é o padrão para docs de bibliotecas a partir de comentários.
- **Cultura:** tensão produtiva entre a norma paga/formal e a wiki gratuita que a maioria consulta.

### Rust ⭐ (modelo de referência)
- **Artefato fundador:** *The Rust Programming Language* ("**The Book**", tRPL). Complementado pelo **Rust Reference** (referência formal) e docs da std via **rustdoc**.
- **Evolução:** desde 1.0 (2015) usa processo **RFC** (`rust-lang/rfcs`); modelo de **editions** (2015/2018/2021/2024). Docs são cidadãs de primeira classe (equipe dedicada).
- **Estrutura:** múltiplos "books" versionados e independentes — The Book (tutorial), Reference, Rust by Example, std docs, Rustonomicon (unsafe), Cargo Book, rustc Book.
- **Ferramentas:** **mdBook** (Markdown→site) para os books; **rustdoc** gera API de doc-comments (`///`, `//!`) e **testa os exemplos** (doctests); crates publicadas aparecem em **docs.rs** automaticamente.
- **Cultura:** norma forte (`#![warn(missing_docs)]`), exemplos executáveis, contribuição via PR.

### Zig
- **Artefato canônico:** o **langref** (Language Reference), documento HTML de página única (`doc/langref.html.in`).
- **Evolução:** pré-1.0; docs versionadas junto ao compilador; mudanças via issues/proposals no GitHub (sem RFC pesado).
- **Estrutura:** langref (linguagem) + std docs (via autodoc) + guias comunitários (zig.guide).
- **Ferramentas:** o langref é template HTML e **todos os exemplos são compilados/testados na suíte do compilador**; **autodoc** é uma SPA em WebAssembly + Markdown gerada pelo próprio compilador (`zig std` sobe servidor HTTP que reconstrói on-the-fly de `///`).
- **Cultura:** núcleo pequeno em torno de Andrew Kelley; "docs verificados pela compilação".

### Assembly
- **Artefato canônico:** o **manual da arquitetura (ISA)** do fabricante — Intel SDM, AMD64 Programmer's Manual, ARM ARM, RISC-V ISA Specs. O assembler também tem seu manual (GNU `as`, NASM, MASM).
- **Evolução:** atrelada ao hardware (MMX/SSE/AVX; ARMv7/8/9). RISC-V trouxe modelo aberto (spec no GitHub).
- **Estrutura:** camadas separadas — **ISA** (instruções, registradores, opcodes) vs **assembler** (sintaxe AT&T/Intel, diretivas) vs **ABI/calling conventions**. Não há "stdlib": o equivalente é o ABI.
- **Ferramentas:** PDFs enormes de fabricante; docs GNU em Texinfo; RISC-V em LaTeX/AsciiDoc no GitHub. Redação manual, sem geração automática.
- **Cultura:** predominantemente corporativa; camada comunitária em wikis (OSDev, felixcloutier.com/x86). RISC-V é a exceção aberta.
- **Modelo notável:** a **tabela por-instrução** (uma página por instrução: forma, operandos, flags afetados, exceções) é um formato de referência muito eficaz.

---

## Parte 2 — Linguagens dinâmicas de alto nível

### Python ⭐
- **Artefato canônico:** `docs.python.org` combinando **The Python Tutorial** + **Standard Library** + **Language Reference**. Nasceu em LaTeX (Guido + Fred Drake), migrou para reStructuredText em 2002.
- **Processo formal:** **PEPs** (desde 2000; PEP 1 define o processo, PEP 8 o estilo). Cada decisão vira artefato durável e citável (ex.: PEP 484, type hints).
- **Estrutura:** **adotou Diátaxis formalmente** (~2022) — tutorial, how-to, reference, explanation.
- **Ferramentas:** **Sphinx** foi criado por Georg Brandl **especificamente para o CPython** (2008); usa **reStructuredText**, gera HTML/PDF/EPUB/man/Texinfo, extrai docstrings via `autodoc`. **Read the Docs** consolidou o hosting.
- **Comunidade:** fluxo via GitHub (`python/cpython`, `Doc/`), Documentation Working Group, traduções voluntárias.

### JavaScript
- **Artefato canônico:** duas camadas — **MDN Web Docs** (referência prática de facto) e a **spec ECMAScript (ECMA-262)** (definição normativa). MDN passou a ter governança compartilhada via **Open Web Docs** (2021).
- **Processo formal:** comitê **TC39** evolui por consenso em 6 estágios (0→4); exige spec text, testes (**test262**) e múltiplas implementações. Desde ES2016, aberto no GitHub (`tc39/ecma262`), cadência anual.
- **Estrutura:** MDN separa reference / guides-tutorials / explanation na prática; a spec é referência normativa pura (algoritmos passo a passo).
- **Ferramentas:** MDN em **Markdown versionado no GitHub** (`mdn/content`) + dados de compatibilidade em **BCD (browser-compat-data, JSON)**; a spec é escrita em **Ecmarkup**.
- **Comunidade:** PRs em `mdn/content`; propostas via `tc39/ecma262` decididas em plenárias.

### Ruby
- **Artefato canônico:** API core/stdlib em `docs.ruby-lang.org`, gerada **de comentários no fonte**; narrativa histórica no "Pickaxe" (*Programming Ruby*).
- **Processo formal:** informal — mudanças discutidas no bug tracker (`bugs.ruby-lang.org`, Redmine), decididas por Matz e o core; sem "proposals" numeradas fortes.
- **Estrutura:** predominantemente reference; tutoriais/explicações em guias externos.
- **Ferramentas:** **RDoc** (comentários → HTML + comando `ri` offline no terminal); marcação RDoc (planos de migrar para Markdown); **YARD** como alternativa com tags (`@param`, `@return`).
- **Comunidade:** contribuição = editar comentários no fonte (`ruby/ruby`); há Documentation Guide oficial.

### Lua
- **Artefato canônico:** o **Lua Reference Manual** — a definição oficial da linguagem num **único documento HTML** (sintaxe, semântica, stdlib e C API). Deliberadamente enxuto.
- **Processo formal:** modelo "catedral" — **sem proposals públicas nem comitê**; desenvolvido por time pequeno na PUC-Rio (Ierusalimschy). O livro **Programming in Lua (PiL)** é o tutorial canônico.
- **Estrutura:** separação **por artefato**: Manual (referência normativa) + PiL (tutorial/explanation).
- **Ferramentas:** **baixa tecnologia** — HTML mantido à mão (derivado de LaTeX/troff para PDF), sem gerador nem extração de docstrings.
- **Comunidade:** doc oficial **não colaborativa**; comunidade contribui em espaços paralelos (lua-users wiki, lista lua-l). Núcleo fechado, periferia aberta.

---

## Parte 3 — JVM e funcionais

### Java
- **Artefato canônico:** **Javadoc** (referência de API gerada de comentários `/** */`) + **The Java Language Specification (JLS)** (spec formal normativa) + The Java Tutorials.
- **Inovação:** o Javadoc (~1996) foi **pioneiro em gerar docs de referência a partir do código-fonte**, com tags semânticas (`@param`, `@return`, `@throws`, `@see`, `{@link}`). Influenciou Doxygen, JSDoc, phpDocumentor.
- **Estrutura:** referência gerada (Javadoc→HTML) + guias narrativos + spec formal (JLS + JVMS) como contrato entre implementações.
- **Cultura:** documentar APIs com Javadoc é convenção enraizada, cobrada em code review; IDEs exibem inline. Mais "obrigação de engenharia" que "prazer literário".

### Go ⭐
- **Artefato canônico:** **A Tour of Go** (interativo) + **Effective Go** (guia idiomático) + **godoc/pkg.go.dev** (referência de API) + a spec enxuta.
- **Inovação:** godoc levou "docs de comentários" ao minimalismo — **texto simples sem marcação** acima da declaração vira doc; **exemplos executáveis** (`func ExampleXxx`) rodam como testes. `gofmt` criou a cultura do "único jeito canônico".
- **Estrutura:** referência gerada + guias curtos e opinativos + spec legível, tudo em `go.dev/doc`.
- **Ferramentas:** `godoc` (→ pkgsite/pkg.go.dev), comando `go doc` no terminal, exemplos testáveis via `go test`.
- **Cultura:** referência em docs porque o ecossistema torna documentar **automático e uniforme** — todo pacote publicado ganha página em pkg.go.dev sem esforço.

### Haskell
- **Artefato canônico:** o **Haskell Report** (spec formal — Haskell 98 em 1999, Haskell 2010) como documento normativo; **Haddock** como gerador de API.
- **Inovação:** exemplo mais forte de **"especificação formal como artefato canônico"** — linguagem definida por comitê, independente de implementação, permitindo múltiplos compiladores conformes.
- **Estrutura:** forte polo de spec (Report) + referência gerada (Haddock→HTML no Hackage). Narrativa veio mais de livros da comunidade.
- **Ferramentas:** **Haddock** com markup própria (`-- |`, `-- ^`); integração com **Hackage**.
- **Cultura:** centrada em rigor e tipos (assinaturas já são meia-documentação); narrativa é o ponto mais fraco.

### Elixir ⭐
- **Artefato canônico:** **ExDoc** (API de `@doc`/`@moduledoc` em **Markdown**) + **cultura de doctests** (exemplos `iex>` executados como testes) + guias oficiais.
- **Inovação:** (a) **documentação como cidadã de primeira classe da linguagem** — `@doc`/`@moduledoc` são construções da linguagem, acessíveis em runtime (`h Enum.map`); (b) **doctests** garantem que exemplos nunca desatualizam; (c) ExDoc moderno com cheatsheets.
- **Estrutura:** Markdown embutido no código → referência (ExDoc no **HexDocs**) + guias + doctests ligando docs e testes.
- **Ferramentas:** **ExDoc** (HTML/EPUB, publica automaticamente no hexdocs.pm ao lançar no Hex), doctests via `ExUnit.DocTest`.
- **Cultura:** referência em docs por decisão de design de José Valim; publicar no Hex gera docs automaticamente; padrão altíssimo e consistente entre bibliotecas.

---

## Parte 4 — Metodologias, frameworks e ferramentas

### Diátaxis (arquitetura de conteúdo)
Criado por **Daniele Procida** (Django/Canonical). Organiza toda a doc por *"o que o leitor está tentando fazer?"* em quatro tipos distintos:

| Tipo | Orientação | Propósito |
|------|-----------|-----------|
| **Tutorials** | Aprendizado | Lição guiada e prática para o iniciante |
| **How-to guides** | Problema | Passos para uma tarefa específica (assume conhecimento) |
| **Reference** | Informação | Descrição técnica factual para consulta (API, flags, sintaxe) |
| **Explanation** | Entendimento | Conceitos, contexto, o "por quê" |

Adotado por Django, Cloudflare, Canonical, Python. É o vocabulário padrão de arquitetura de informação técnica.

### Programação Literária (Knuth, 1984)
WEB/CWEB: o programa é um *ensaio para humanos*; `tangle` extrai o código compilável e `weave` gera a doc tipográfica. Nunca virou mainstream para programar, mas semeou docstrings, Doxygen, Jupyter, R Markdown/knitr e os **doctests** modernos — a ideia de que "código e doc são um só artefato".

### Docs as Code
- **Texto plano legível:** Markdown/CommonMark/GFM, **reStructuredText** (rico, ecossistema Python) ou **AsciiDoc** (mais rico, docs longas/Antora).
- **Versionado em Git**, **revisado em PR**, **build em CI/CD** (linters como Vale/markdownlint, checagem de links, deploy automático).
- **Single-sourcing:** mesmo conteúdo gera HTML/PDF/man.

### Geradores (quando usar cada um)

| Ferramenta | Base | Ponto forte | Quando escolher |
|-----------|------|-------------|-----------------|
| **Sphinx** | Python/rST (+MyST) | Cross-ref profundo, `autodoc`, multi-formato, doctest | APIs grandes, referências cruzadas rigorosas |
| **MkDocs (Material)** | Python/Markdown | Simples, rápido, bonito, busca instantânea, i18n | Docs Markdown-first com bom visual sem complexidade |
| **Docusaurus** | JS/React (Meta) | Versionamento e **i18n** de 1ª classe, MDX, Algolia | SDKs/produtos modernos, times JS |
| **Doxygen** | C/C++/Java | Extrai de comentários, gera diagramas/call graphs | Bibliotecas de sistemas |
| **Antora** | AsciiDoc | **Multi-repositório** e versionamento robusto | Docs corporativas distribuídas em vários repos |
| **mdBook** | Rust/Markdown | Rápido, binário único, saída "livro" | Livros/tutoriais lineares (The Rust Book) |

### Man pages e especificações formais
- **man pages** (troff/groff, macros `man`/`mdoc`): seções numeradas, estrutura canônica (NAME, SYNOPSIS, DESCRIPTION, OPTIONS, EXAMPLES, SEE ALSO). Padrão de referência de CLI em *nix.
- **Texinfo** (GNU): fonte única → Info/HTML/PDF/man.
- **RFCs** (IETF): prosa normativa com terminologia **RFC 2119** (MUST/SHOULD/MAY).
- **ISO/ANSI:** padrões formais de linguagens (C, C++).
- **Gramáticas BNF/EBNF/ABNF:** notação formal de sintaxe. **BNF** (ALGOL 60), **EBNF** (ISO/IEC 14977), **ABNF** (RFC 5234). Reaproveitável para gerar parsers e diagramas railroad.

### Boas práticas modernas
- **Versionamento de docs** por release, com seletor (Read the Docs, `mike`, Docusaurus/Antora).
- **Exemplos executáveis/testáveis:** doctests (Python, rustdoc, Go Examples) falham no CI se quebram; playgrounds embutidos.
- **Acessibilidade (WCAG, dark mode), i18n/l10n (Crowdin, gettext), busca (Algolia DocSearch, lunr.js, busca IA).**
- **Qualidade automatizada:** linters de prosa (Vale), links mortos, style guide.

---

## Síntese final — recomendações para a documentação do kaju

Os padrões que **se repetem nas linguagens mais bem documentadas** (Rust, Go, Elixir, Python) e que devemos adotar desde o dia zero:

1. **Adotar Diátaxis como arquitetura.** Quatro trilhas explícitas: *Aprender* (um "The kaju Book"), *Guias* (how-to), *Referência* (linguagem + stdlib) e *Explicação* (conceitos, modelo de memória, rationale). Não criar um "manual" único que mistura tudo.

2. **Um artefato canônico ancora tudo.** Um livro/reference bem escrito dá identidade e ponto de entrada (K&R, The Rust Book, langref do Zig, Manual do Lua).

3. **Separar os três papéis, mas conectá-los:** especificação normativa (contrato, estilo JLS/Haskell Report/Manual Lua), referência de API gerada do código (rustdoc/godoc/ExDoc), e guias narrativos opinativos e curtos (Effective Go).

4. **Exemplos de código testados no CI — a lição mais forte de todas.** doctests do Rust/Elixir e exemplos compilados do langref do Zig garantem que a doc *nunca mente*. Implementar isso no próprio toolchain do kaju.

5. **Gerar a referência de API a partir de doc-comments** (fonte única de verdade), como rustdoc/godoc/autodoc/ExDoc. Decidir conscientemente entre **texto simples** (Go, baixo atrito) e **tags semânticas** (Java/Haddock, mais rico).

6. **Tornar a documentação recurso de primeira classe da linguagem/runtime**, não convenção de comentário — o modelo `@doc` acessível em runtime do Elixir e `h` no REPL muda a cultura.

7. **Publicação sem esforço com host central.** pkg.go.dev/docs.rs/HexDocs indexam docs automaticamente ao publicar um pacote — é o que cria cultura de documentar em toda a comunidade.

8. **Docs-as-code desde o início:** Markdown no repositório da linguagem, revisado em PR, build/deploy em CI, quebra de doc = quebra de build.

9. **Especificação formal com gramática (E)BNF**, terminologia normativa estilo RFC 2119, e definição explícita de *undefined/implementation-defined behavior* — evita fragmentação entre implementações.

10. **Processo tipo RFC** (Rust) ou papers numerados (C++/WG21), ou ao menos PEPs (Python): torna decisões de design auditáveis e a evolução legítima. Amarrar "feature nova" a "doc + testes" no processo.

11. **Versionar a doc junto com a linguagem** e anotar cada feature com "desde a versão X" (cppreference, editions do Rust).

12. **Planejar cedo:** i18n (Crowdin), busca (Algolia/client-side), acessibilidade (WCAG, dark mode), e man pages/`--help` estruturado para a CLI do toolchain.

### Escolha de ferramentas sugerida para o kaju
- **mdBook** para "The kaju Book" (tutorial linear) — padrão de facto de linguagens novas, simples, zero dependências.
- **Docusaurus** ou **MkDocs Material** para o portal principal, se i18n/versionamento/visual forem prioridade.
- **Sphinx** apenas se a referência da stdlib exigir cross-referencing pesado.
- **Gerador de API próprio no compilador** (estilo rustdoc/autodoc) que extrai doc-comments e **compila/testa os exemplos**.

### Espectro de governança a decidir
Aberto (MDN/Open Web Docs, PRs) escala via comunidade e tradução mas exige guias de contribuição e style guide (imposto por Vale no CI). Fechado (Lua, PUC-Rio) garante coesão e precisão mas depende de poucos mantenedores. Definir isso cedo.

---

### Fontes principais
- **Baixo nível:** [ANSI C](https://en.wikipedia.org/wiki/ANSI_C) · [cppreference](https://cppreference.com/) · [mdBook](https://rust-lang.github.io/mdBook/) · [rustdoc](https://github.com/rust-lang/rust/tree/main/src/doc/rustdoc) · [Zig langref](https://github.com/ziglang/zig/blob/master/doc/langref.html.in)
- **Dinâmicas:** [Sphinx](https://en.wikipedia.org/wiki/Sphinx_(documentation_generator)) · [Diátaxis no Python](https://discuss.python.org/t/adopting-the-diataxis-framework-for-python-documentation/15072) · [Lua Manual](https://www.lua.org/manual/) · [tc39/ecma262](https://github.com/tc39/ecma262) · [RDoc](https://ruby.github.io/rdoc/)
- **JVM/funcionais:** [Javadoc](https://www.oracle.com/java/technologies/javase/javadoc-tool.html) · [Godoc](https://go.dev/blog/godoc) · [Effective Go](https://go.dev/doc/effective_go) · [Haskell 2010 Report](https://www.haskell.org/onlinereport/haskell2010/) · [Haddock (Marlow)](https://simonmar.github.io/bib/papers/haddock.pdf) · [ExDoc features](https://elixir-lang.org/blog/2022/12/22/cheatsheets-and-8-other-features-in-exdoc-that-improve-the-developer-experience/)
- **Metodologias:** [Diátaxis](https://diataxis.fr/) · [Read the Docs — doctools](https://docs.readthedocs.com/platform/stable/intro/doctools.html) · [Markdown vs AsciiDoc vs rST](https://www.dewanahmed.com/markdown-asciidoc-restructuredtext/)
