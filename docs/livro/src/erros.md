# Erros e exceções

Programas falham: um arquivo não existe, um texto não vira número, uma conta divide por zero. O kaju trata esses casos de dois modos que se complementam. Primeiro, ele **relata** o que deu errado com um diagnóstico detalhado, em português, que aponta o trecho do código e sugere a correção. Segundo, ele deixa você **capturar** um erro em tempo de execução para reagir a ele em vez de deixar o programa parar.

## A anatomia de um diagnóstico

Quando um erro impede o programa de continuar, o kaju o descreve em um formato consistente. Cada diagnóstico tem um **código** (por exemplo, `K001`), uma mensagem curta, a localização exata e, quando possível, uma dica de como corrigir:

```
erro[K001]: a variável 'idde' não foi definida
 --> programa.kaju:3:9
  |
3 | escreva(idde)
  |         ^^^^ não existe nenhuma variável com este nome
  |
ajuda: você quis dizer 'idade'?
```

As partes são sempre as mesmas:

- o **cabeçalho** traz a severidade, o código do erro e a mensagem;
- a **localização** aponta `arquivo:linha:coluna`;
- o **trecho** mostra a linha envolvida, com `^^^^` marcando o intervalo exato;
- as linhas `nota:` e `ajuda:` explicam o porquê e sugerem uma correção.

Os códigos são organizados por categoria: os léxicos começam em `K1xx`, os de sintaxe em `K0xx` e os de execução em `K2xx`.

## Consultando um código de erro

Cada código tem uma explicação longa, com exemplos, acessível pelo terminal:

```bash
kaju explique K001
```

Use isso sempre que a dica da tela não bastar: a explicação detalha a causa comum daquele erro e como resolvê-la.

## Capturando erros com tente / capture

Alguns erros só aparecem durante a execução — ler um arquivo que não existe, converter um texto inválido, dividir por zero. Em vez de deixar o programa parar, você pode envolver o trecho arriscado em um bloco `tente` e tratar a falha em `capture`:

```kaju
tente {
    var x = 10 / 0
    escreva(x)
} capture (erro) {
    escreva("Falhou:", erro.mensagem, "| código:", erro.codigo)
}
```

Se qualquer comando dentro de `tente` falhar, a execução salta imediatamente para o bloco `capture`. O identificador entre parênteses (aqui, `erro`) recebe um **objeto de erro** com dois campos garantidos:

- `.mensagem` — o texto que descreve a falha;
- `.codigo` — o código do erro (por exemplo, `"K020"` para divisão por zero).

O `capture` pega tanto os erros do próprio interpretador quanto os que você lança (veja a seção seguinte).

## O bloco finalmente

Depois de `capture`, você pode adicionar um bloco `finalmente`. Ele executa **sempre** — tenha ocorrido erro ou não — e é o lugar ideal para liberar recursos ou registrar o fim de uma operação:

```kaju
tente {
    var dados = leiaArquivo("config.kaju")
    escreva(dados)
} capture (erro) {
    escreva("não consegui ler:", erro.mensagem)
} finalmente {
    escreva("tentativa de leitura encerrada")
}
```

O `finalmente` é opcional. Quando presente, roda por último, mesmo que o `tente` termine sem falhas.

## Lançando os seus próprios erros

Nem todo erro vem do interpretador. Quando uma função recebe um valor inválido para a sua lógica, ela pode interromper o fluxo com `lance`:

```kaju
funcao sacar(saldo, valor) {
    se valor > saldo {
        lance "saldo insuficiente"
    }
    retorne saldo - valor
}

tente {
    sacar(100, 150)
} capture (erro) {
    escreva(erro.mensagem)   // saldo insuficiente
}
```

Quando você lança um texto, o kaju o transforma em um objeto de erro cuja `.mensagem` é aquele texto.

## Erros personalizados com objetos

Um texto informa *o que* deu errado, mas às vezes você quer carregar mais dados junto. Para isso, defina uma classe de erro e lance uma instância dela:

```kaju
classe ErroDeSaldo {
    construtor(faltam) {
        isto.mensagem = "saldo insuficiente"
        isto.faltam = faltam
    }
}

tente {
    lance novo ErroDeSaldo(50)
} capture (falha) {
    escreva(falha.mensagem + ", faltam " + falha.faltam)
}
```

O objeto lançado chega intacto ao `capture`, com todos os campos que você definiu. Assim quem trata o erro pode decidir o que fazer com base em dados concretos, não apenas em uma mensagem.

## Quando ninguém captura

Um `lance` que não é envolvido por nenhum `tente` sobe até o topo do programa e vira o erro **K230**, exibido no mesmo formato detalhado dos demais diagnósticos. Isso é útil durante o desenvolvimento: um erro esquecido aparece com clareza, em vez de passar despercebido.

---

Adiante: [Módulos](./modulos.md).
