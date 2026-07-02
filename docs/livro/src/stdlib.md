# Biblioteca padrão

A kaju vem com um conjunto de funções sempre disponíveis — os **embutidos**. Você não precisa importar nada para usá-las: elas fazem parte da linguagem. Este capítulo é uma referência rápida, agrupada por tema. Os métodos de listas, textos e dicionários (aqueles chamados com `.`, como `lista.mapeie(...)`) são cobertos nos capítulos de [Coleções](./colecoes.md) e [Textos](./textos.md).

## Entrada e saída

| Função | O que faz |
|--------|-----------|
| `escreva(...)` | Imprime os argumentos separados por espaço e quebra a linha ao final. |
| `escrevaSemQuebra(...)` | Imprime sem quebrar a linha — útil para montar prompts. |
| `leia()` | Lê uma linha da entrada e a devolve como `texto`. |
| `pergunte(texto)` | Mostra `texto` (sem quebra de linha) e devolve a linha digitada. |

```kaju
var nome = pergunte("Como você se chama? ")
escreva("Olá,", nome)
```

## Texto, tipos e conversões

| Função | O que faz |
|--------|-----------|
| `tamanho(x)` | Comprimento de um `texto`, `lista` ou `dicionario`. |
| `tipo(x)` | Nome do tipo de `x` como `texto` (ex.: `"numero"`, `"texto"`). |
| `classeDe(x)` | Nome da classe de um objeto como `texto`. |
| `paraTexto(x)` | Converte qualquer valor para `texto`. |
| `paraNumero(x)` | Converte um `texto` ou `logico` para `numero`. |
| `paraInteiro(x)` | Converte para inteiro, truncando a parte decimal. |

```kaju
var idade = paraNumero(pergunte("Idade: "))
escreva("Tipo:", tipo(idade))     // Tipo: numero
```

## Números e matemática

| Função | O que faz |
|--------|-----------|
| `raiz(x)` | Raiz quadrada de `x` (decimal). |
| `absoluto(x)` | Valor absoluto, preservando o tipo. |
| `potencia(base, exp)` | `base` elevado a `exp`. |
| `piso(x)` | Arredonda para baixo (inteiro). |
| `teto(x)` | Arredonda para cima (inteiro). |
| `arredonde(x)` | Arredonda para o inteiro mais próximo. |
| `arredondePara(n, casas)` | Arredonda `n` com o número de casas decimais indicado. |
| `formateDecimal(n, casas)` | Formata `n` como `texto` com um número fixo de casas. |
| `aleatorio()` | Um decimal aleatório entre 0 e 1. |
| `minimo(...)` | O menor dos argumentos. |
| `maximo(...)` | O maior dos argumentos. |
| `seno(x)`, `cosseno(x)` | Seno e cosseno de `x` (em radianos). |
| `log(x)` | Logaritmo natural de `x`. |
| `PI` | A constante do círculo, `3.14159...`. |

```kaju
escreva(raiz(144))              // 12.0
escreva(arredondePara(PI, 2))   // 3.14
escreva(maximo(3, 9, 1))        // 9
```

Para a divisão inteira, combine `piso` com a divisão: `piso(7 / 2)` resulta em `3`.

## Coleções

| Função | O que faz |
|--------|-----------|
| `tamanho(x)` | Número de elementos de uma lista ou dicionário. |
| `intervalo(inicio, fim)` | Lista de inteiros de `inicio` (incluído) até `fim` (excluído). |

```kaju
para cada i em intervalo(1, 5) {
    escreva(i)     // 1, 2, 3, 4
}
```

## Tempo

| Função | O que faz |
|--------|-----------|
| `agora()` | Segundos inteiros desde 1º de janeiro de 1970 (tempo Unix). |
| `relogio()` | Milissegundos desde 1970 — bom para medir durações. |
| `formatarData(seg)` | Recebe segundos Unix e devolve `"AAAA-MM-DD HH:MM:SS"` em UTC. |

```kaju
var inicio = relogio()
// ... algum trabalho ...
escreva("levou", relogio() - inicio, "ms")

escreva(formatarData(agora()))
```

## Arquivos

| Função | O que faz |
|--------|-----------|
| `leiaArquivo(caminho)` | Lê todo o conteúdo do arquivo como `texto`. |
| `escrevaArquivo(caminho, conteudo)` | Grava `conteudo` no arquivo, criando-o se necessário. |
| `existeArquivo(caminho)` | Devolve `verdadeiro` se o arquivo existe. |

```kaju
se nao existeArquivo("notas.txt") {
    escrevaArquivo("notas.txt", "primeira nota\n")
}
escreva(leiaArquivo("notas.txt"))
```

Uma leitura pode falhar (arquivo inexistente, sem permissão). Envolva a chamada em `tente`/`capture` quando quiser tratar isso sem parar o programa — veja [Erros e exceções](./erros.md).

## JSON

| Função | O que faz |
|--------|-----------|
| `paraJSON(valor)` | Serializa um valor kaju em um `texto` no formato JSON. |
| `deJSON(texto)` | Converte um `texto` JSON de volta em um valor kaju. |

```kaju
var pessoa = {"nome": "Ana", "idade": 30}
var texto = paraJSON(pessoa)
escreva(texto)                  // {"nome":"Ana","idade":30}

var lida = deJSON(texto)
escreva(lida.obtem("nome", "?"))   // Ana
```

---

A seguir: [Suporte a editor](./editor.md).
