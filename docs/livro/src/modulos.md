# Módulos

À medida que um programa cresce, mantê-lo num único arquivo fica desconfortável. Os **módulos** permitem espalhar o código por vários arquivos `.kaju` e reaproveitá-lo: você escreve funções, constantes e classes num arquivo e as usa em outro com o comando `importe`.

## Um módulo é apenas um arquivo

Qualquer arquivo `.kaju` pode ser um módulo. Suponha um arquivo `geometria.kaju` com algumas utilidades:

```kaju
constante PI = 3.14159

funcao areaCirculo(raio) {
    retorne PI * raio * raio
}

classe Ponto {
    construtor(x, y) {
        isto.x = x
        isto.y = y
    }
}
```

Não é preciso marcar nada como "exportável": as definições de nível superior — funções, constantes e classes — ficam disponíveis para quem importar o arquivo.

## Trazendo os nomes para o escopo

A forma mais direta de `importe` traz os nomes do módulo para o arquivo atual, como se tivessem sido definidos ali:

```kaju
importe "geometria.kaju"

escreva(areaCirculo(2))       // 12.56636
var p = novo Ponto(3, 4)
escreva(p.x)                  // 3
```

Depois do `importe`, `areaCirculo`, `PI` e `Ponto` são usados diretamente, sem prefixo.

## Importando sob um nome

Quando você importa vários módulos, dois deles podem ter uma função com o mesmo nome. Para evitar o conflito, importe o módulo sob um nome com `como`. Todos os seus membros passam a ser acessados com esse prefixo:

```kaju
importe "geometria.kaju" como geo

escreva(geo.PI)               // 3.14159
escreva(geo.areaCirculo(5))   // 78.53975
```

Essa forma deixa claro de onde cada nome vem e mantém o escopo do arquivo limpo. O prefixo dá acesso às funções e aos valores do módulo. Para **criar objetos** de uma classe do módulo, use o `importe` simples (sem `como`), que traz a classe para o escopo e permite `novo Ponto(1, 2)` diretamente.

## Como os caminhos são resolvidos

O caminho passado ao `importe` é **relativo ao arquivo que faz o importe**, não ao diretório de onde você executa o kaju. Se `principal.kaju` e `geometria.kaju` estão na mesma pasta, `importe "geometria.kaju"` funciona independentemente de onde você rode o programa. Para módulos em subpastas, use o caminho relativo correspondente, como `importe "utilidades/texto.kaju"`.

## Cada módulo roda uma vez

Ao ser importado pela primeira vez, o módulo é **executado** de cima para baixo — é assim que suas funções e classes passam a existir. O resultado fica em **cache**: se o mesmo módulo for importado de novo, em qualquer parte do programa, ele não roda outra vez. Isso evita trabalho repetido e efeitos colaterais duplicados, e garante que dois arquivos que importam o mesmo módulo compartilhem exatamente as mesmas definições.

## Erros comuns ao importar

Dois códigos de erro aparecem com frequência ao trabalhar com módulos:

- **K220** — o arquivo indicado no `importe` não foi encontrado (confira o caminho e se ele é relativo ao arquivo atual);
- **K221** — ocorreu um erro *dentro* do módulo enquanto ele era executado.

Como sempre, `kaju explique K220` traz uma explicação detalhada.

---

A seguir, a [Biblioteca padrão](./stdlib.md).
