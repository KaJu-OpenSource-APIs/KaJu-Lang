# Changelog

Todas as mudanças notáveis neste projeto são documentadas neste arquivo.

O formato é baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/)
e o projeto adere ao [Versionamento Semântico](https://semver.org/lang/pt-BR/).

## [Não lançado]

### Documentação

- README: seção "Destaques" cita os testes embutidos (`afirme` e `kaju teste`).
- Instalação por linha de comando (download do binário via `curl`) no README e no
  Livro, útil para servidores/VPS.
- Seção de instalação reordenada (binário pronto primeiro, sem clonar; o `git clone`
  passa a aparecer só na parte de compilar da fonte).

### Adicionado

- **Operador de encadeamento `|>`** (pipe): `x |> f(a)` passa `x` como primeiro
  argumento (`f(x, a)`). Entende funções livres e métodos — se o nome à direita
  não for uma função em escopo, vira chamada de método (`x |> maiusculas` ≡
  `x.maiusculas()`), permitindo encadear os métodos de coleção.
- **Argumentos nomeados** em chamadas: `f(nome: valor)`, também em construtores
  (`novo Classe(x: 1)`) e métodos definidos em kaju. Podem ser reordenados e
  combinam com parâmetros padrão; posicionais sempre vêm antes dos nomeados.
  Novos erros K023 (posicional após nomeado), K224 (parâmetro inexistente),
  K225 (argumento informado duas vezes) e K226 (nomeados não aceitos aqui).
- **Fatiamento com colchetes**: `lista[a:b]`, `lista[a:]`, `lista[:b]` e `lista[:]`
  para listas e textos, com limites opcionais e índices negativos (contados a
  partir do fim). Limites fora da faixa são ajustados sem erro.
- Operador **`??` (coalescência de nulo)**: `a ?? b` devolve `a` se não for nulo,
  senão `b`. Reage apenas a `nulo` (`0`, `""` e `falso` passam intactos).
- Operador **`?.` (acesso opcional)**: `a?.membro` e `a?.metodo(...)` resultam em
  `nulo` quando `a` é nulo, sem erro — encadeável (`a?.b?.c`).
- Novos **métodos de lista de alta ordem**: `encontre(f)` (primeiro elemento que
  satisfaz `f`), `algum(f)` (algum satisfaz), `todos(f)` (todos satisfazem) e
  `agrupe(f)` (agrupa num dicionário pela chave devolvida por `f`).
- Novos **métodos de lista**: `achate()` (achata um nível de sublistas),
  `combine(outra)` (zíper, formando pares `[a, b]`) e `enumere()` (pares
  `[indice, valor]`).
- `ROADMAP.md` com as direções futuras do projeto (extraído da §14 da
  especificação e reescrito em tom de direções, não de fases internas).
- Extensão de VS Code 1.0.0: **snippets** para as construções da linguagem,
  auto-indentação ao abrir blocos, dobra por regiões e metadados do repositório.
  Instala-se pelo repositório com `editores/vscode-kaju/instalar.sh` (sem
  Marketplace).

### Corrigido (documentação)

- `tipos.md` dizia que estouro de inteiro "promove para decimal"; na verdade dá o
  erro K222. Texto corrigido.
- `stdlib.md`: saída de exemplo do `paraJSON` estava com as chaves fora da ordem
  (a saída real é ordenada alfabeticamente).
- `erros.md`: descrição das faixas de código de erro estava rígida demais e se
  contradizia (K020/K001 são de execução mas ficam em K0xx).
- README: `kaju --versao` faltava na lista de comandos da seção "Uso".
- Especificação: descrições de `K211`/`K213` estavam restritas ao caso "estático";
  agora refletem o uso geral.

## [1.0.0] - 2026-07-03

### Adicionado

- **`kaju teste`**: executor de testes na própria linguagem — roda as funções
  globais `teste*` de um arquivo ou pasta e reporta ✓/✗ com resumo e código de
  saída.
- **`afirme(condicao[, mensagem])`**: função embutida que falha (erro K231) se a
  condição for falsa; base para escrever testes.
- **Método especial `paraTexto()`**: uma classe pode definir como suas instâncias
  viram texto (usado por `escreva`, `+`, interpolação e coleções).
- **Método especial `igual(outro)`**: uma classe pode definir a igualdade de suas
  instâncias para o operador `==`.
- **Laço `para ... passo P`**: incremento arbitrário, incluindo contagem
  regressiva com passo negativo. Nova palavra reservada `passo`.
- **`novo m.Classe(...)`**: instanciar uma classe de um módulo importado com
  `importe ... como m`.
- **`kaju --versao`**: mostra a versão instalada.
- **Integração contínua** (GitHub Actions): build e testes em Linux, Windows e
  macOS, além de `cargo fmt` e `cargo clippy`.
- **Release automatizado**: ao publicar uma tag `v*`, binários pré-compilados
  para Linux, Windows e macOS são anexados ao GitHub Release.
- Script `install.sh` para Linux e macOS.

### Alterado

- **Igualdade estrutural**: `==` entre listas e entre dicionários agora compara
  por conteúdo, não por referência.
- **Estouro de inteiro** em `+`, `-` e `*` vira o erro K222 em vez de virar
  decimal silenciosamente, evitando perda de precisão sem aviso.
- **Códigos de erro de métodos** corrigidos: aridade usa K201, tipo de argumento
  usa K203 e método inexistente usa K212 (antes tudo virava K212).

### Documentação

- Especificação sincronizada com a implementação; novo capítulo "Testando" no
  Livro; instruções de instalação para Linux, Windows e macOS.

## [0.1.0] - 2026-07-02

Primeira versão do kaju: uma linguagem interpretada, de uso geral, com sintaxe e
biblioteca padrão totalmente em português.

### Adicionado

- **Tipos**: `numero`, `texto`, `logico`, `lista`, `dicionario`, `funcao`, `nulo`.
  O tipo `numero` distingue internamente inteiro exato (i64) e decimal (f64).
- **Variáveis**: `var` e `constante`, com desempacotamento (`var a, b = 1, 2`) e
  troca (`a, b = b, a`).
- **Controle de fluxo**: `se` / `senaose` / `senao`, `enquanto`,
  `para X de A ate B`, `para cada X em ...`, `pare`, `continue`, `retorne`,
  `escolha` / `caso` / `padrao` e operador ternário `cond ? a : b`.
- **Funções** de primeira classe com closures, parâmetros padrão e variádicos.
- **Orientação a objetos**: `classe`, `construtor`, `metodo`, `novo`, `isto`,
  herança (`herda`), chamadas à superclasse (`base.metodo()`) e membros
  estáticos.
- **Exceções**: `tente` / `capture` / `finalmente` e `lance`.
- **Módulos**: `importe "arquivo.kaju"` e `importe "arquivo.kaju" como m`,
  com cache.
- **Operadores**: aritméticos, comparação, lógicos (`e`, `ou`, `nao`) com
  curto-circuito, atribuição composta (`+=`, `-=`, `*=`, `/=`, `%=`), operadores
  de bits (`& | ^ ~ << >>`) e interpolação de strings (`$"olá {nome}"`).
- **Coleções**: indexação `a[i]`, dicionários `{"chave": valor}` e métodos
  encadeáveis para listas, textos e dicionários.
- **Biblioteca padrão**: E/S (`escreva`, `escrevaSemQuebra`, `leia`, `pergunte`),
  conversões, matemática, data/hora, arquivos e JSON (`paraJSON`, `deJSON`).
- **Diagnósticos ricos** em português no estilo do Rust, com código de erro,
  trecho do código, `nota:` e `ajuda:`.
- **CLI**: execução de arquivos `.kaju` / `.kj`, REPL interativo com histórico e
  entrada multilinha, `kaju explique <codigo>` e `kaju --ajuda`.
- **Ferramentas de editor**: extensão de VS Code e registro de tipo MIME/ícone
  no Linux.


