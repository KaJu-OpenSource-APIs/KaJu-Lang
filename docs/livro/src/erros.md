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

Os códigos são agrupados em faixas: `K0xx` (análise e semântica), `K1xx` (léxico) e `K2xx` (execução). A fronteira não é rígida — alguns erros que só aparecem ao rodar, como divisão por zero (`K020`) e estouro de inteiro (`K222`), ficam na faixa `K0xx`/`K2xx` conforme a natureza. Todos têm explicação em `kaju explique <codigo>`.

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

## Afirmando expectativas com afirme

Quando você quer garantir que uma condição é verdadeira num certo ponto do programa, use `afirme`. Se a condição for verdadeira, ele não faz nada; se for falsa, interrompe com o erro **K231** ("afirmação falhou"). Você pode anexar uma mensagem explicando o que se esperava:

```kaju
afirme(saldo >= 0)
afirme(tamanho(lista) > 0, "a lista não pode estar vazia")
```

`afirme` é a ferramenta natural para escrever **testes** em kaju: cada teste afirma um resultado esperado, e a suíte só passa se nenhuma afirmação falhar.

```kaju
funcao soma(a, b) { retorne a + b }

afirme(soma(2, 3) == 5)
afirme(soma(-1, 1) == 0, "soma com negativo falhou")
escreva("todos os testes passaram")
```

Por ser um erro como os demais, uma afirmação falha também pode ser capturada com `tente`/`capture` — útil, por exemplo, para um executor de testes que reporta cada falha e segue adiante:

```kaju
tente {
    afirme(2 + 2 == 5, "matemática quebrou")
} capture (erro) {
    escreva("teste falhou:", erro.mensagem)   // teste falhou: afirmação falhou: matemática quebrou
}
```

## Quando ninguém captura

Um `lance` que não é envolvido por nenhum `tente` sobe até o topo do programa e vira o erro **K230**, exibido no mesmo formato detalhado dos demais diagnósticos. Isso é útil durante o desenvolvimento: um erro esquecido aparece com clareza, em vez de passar despercebido.

---

Adiante: [Módulos](./modulos.md).
