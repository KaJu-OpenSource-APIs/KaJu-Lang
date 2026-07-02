# Introdução

A **kaju** é uma linguagem de programação de alto nível, **interpretada** e de **uso geral**, cuja sintaxe e biblioteca são **totalmente em português**. Seu interpretador é escrito em Rust e executa o código diretamente, sem etapa de compilação para o usuário.

O propósito da kaju é ser uma linguagem moderna e agradável de ler, em que as palavras-chave e as funções fazem sentido em português — sem abrir mão dos recursos que uma linguagem séria oferece: orientação a objetos, exceções, módulos, funções de primeira classe, números exatos e mensagens de erro claras.

## Um gostinho

```kaju
funcao saudar(nome) {
    retorne $"Olá, {nome}!"
}

var frutas = ["caju", "manga", "acerola"]
para cada fruta em frutas {
    escreva(saudar(fruta))
}
```

Ao rodar este programa, a saída é:

```
Olá, caju!
Olá, manga!
Olá, acerola!
```

Repare em alguns elementos que aparecem já nesse trecho curto: `funcao` declara uma função, `$"..."` interpola valores dentro de um texto, e `para cada ... em` percorre uma lista. Tudo se lê quase como uma frase.

## O que a kaju oferece

- **Português de verdade:** as palavras-chave são `se`, `senao`, `enquanto`, `para cada`, `funcao`, `classe`, `retorne`, `verdadeiro`/`falso`, `e`/`ou`/`nao`. O código descreve o que faz na sua própria língua.
- **Números exatos:** inteiros nunca perdem precisão e convivem com decimais sob um único tipo `numero`.
- **Erros que ensinam:** cada erro tem um código, aponta o trecho exato do programa e sugere a correção. Você pode pedir uma explicação detalhada com `kaju explique K016`.
- **Recursos modernos:** interpolação de textos (`$"{x}"`), desempacotamento (`a, b = 1, 2`), atribuição composta (`+=`), operador ternário e o comando `escolha`/`caso`.
- **Orientação a objetos completa:** classes, herança, membros estáticos e exceções (`tente`/`capture`).

## Como ler este livro

Os capítulos seguem uma ordem de aprendizado: comece pela instalação e avance daí. Cada capítulo traz exemplos que você pode rodar por conta própria, e ao final você conseguirá escrever programas completos em kaju.

Se procura a definição formal e exaustiva da linguagem — gramática, semântica, lista completa de embutidos —, consulte a **especificação** (`ESPECIFICACAO.md`). Este livro é o caminho de *aprendizado*; a especificação é a *referência*.

Próximo: [Instalação e primeiros passos](./instalacao.md)
