# Orientação a objetos

Uma **classe** descreve um tipo de objeto: quais dados ele guarda e o que ele sabe fazer. A partir de uma classe você cria **objetos** (também chamados de instâncias), cada um com seus próprios dados. Este capítulo mostra como definir classes, criar objetos, reaproveitar comportamento com herança e compartilhar dados entre instâncias com membros estáticos.

## Definindo uma classe

Use `classe` seguida do nome. Dentro do bloco, o `construtor` roda uma vez no momento em que o objeto é criado, e os `metodo`s definem o comportamento:

```kaju
classe Animal {
    construtor(nome) {
        isto.nome = nome
    }

    metodo falar() {
        escreva(isto.nome + " faz um som")
    }
}
```

A palavra-chave `isto` refere-se ao objeto atual. Os **campos** de um objeto não são declarados à parte: eles nascem no momento em que você atribui a eles, como em `isto.nome = nome`. A partir daí, todo método pode ler e alterar esses campos através de `isto`.

## Criando e usando objetos

Crie um objeto com `novo`, passando os argumentos que o construtor espera. Depois, acesse campos e chame métodos com o operador `.`:

```kaju
var bicho = novo Animal("Rex")
bicho.falar()         // Rex faz um som
escreva(bicho.nome)   // Rex
```

Cada objeto é independente: dois animais têm cada um o seu próprio `nome`.

```kaju
var a = novo Animal("Rex")
var b = novo Animal("Luna")
a.falar()   // Rex faz um som
b.falar()   // Luna faz um som
```

O construtor é opcional. Se você não declarar um, `novo` cria um objeto sem campos iniciais, que você preenche depois atribuindo a eles.

## Herança

Uma classe pode ser definida como uma versão especializada de outra. Com `herda`, a nova classe recebe todos os métodos da classe-mãe (a **superclasse**) e pode acrescentar ou substituir o que precisar:

```kaju
classe Gato herda Animal {
    construtor(nome, cor) {
        base.construtor(nome)     // inicializa a parte de Animal
        isto.cor = cor
    }

    metodo falar() {
        base.falar()              // reaproveita o comportamento da superclasse
        escreva(isto.nome + " faz miau")
    }
}

var felix = novo Gato("Félix", "preto")
felix.falar()
// Félix faz um som
// Félix faz miau
```

Dentro de uma subclasse, `base` dá acesso à superclasse. `base.construtor(...)` executa o construtor da mãe — útil para reaproveitar a inicialização em vez de repeti-la. Da mesma forma, `base.falar()` chama a versão original de um método.

Quando a subclasse declara um método com o mesmo nome de um da superclasse, a versão da subclasse **substitui** a original para objetos daquele tipo. Foi o que fizemos com `falar`: o `Gato` tem o seu próprio, que por sua vez chama o da base.

Para descobrir a que classe um objeto pertence, use `classeDe`, que devolve o nome como texto:

```kaju
escreva(classeDe(felix))   // Gato
```

## Objetos são referências

Objetos têm semântica de **referência**. Uma variável não guarda o objeto em si, mas uma referência a ele. Ao passar um objeto para uma função, você passa essa referência — então alterações feitas lá dentro afetam o objeto original:

```kaju
funcao renomeie(animal, nome) {
    animal.nome = nome
}

var bicho = novo Animal("Rex")
renomeie(bicho, "Thor")
escreva(bicho.nome)   // Thor
```

O mesmo vale ao atribuir um objeto a outra variável: as duas passam a apontar para o mesmo objeto.

## Como o objeto vira texto: paraTexto

Por padrão, ao imprimir um objeto o kaju mostra algo como `<objeto Ponto>` — informativo, mas pouco amigável. Se a classe define um método chamado `paraTexto` (sem argumentos, devolvendo um texto), o kaju passa a usá-lo sempre que precisa converter o objeto em texto: em `escreva`, na concatenação com `+`, na interpolação `$"..."` e até quando o objeto aparece dentro de uma lista ou dicionário impressos.

```kaju
classe Ponto {
    construtor(x, y) {
        isto.x = x
        isto.y = y
    }

    metodo paraTexto() {
        retorne $"({isto.x}, {isto.y})"
    }
}

var p = novo Ponto(1, 2)
escreva(p)                 // (1, 2)
escreva("posição: " + p)   // posição: (1, 2)
```

## Comparando objetos: igual

O operador `==` entre dois objetos normalmente pergunta se são **o mesmo objeto** (a mesma instância na memória), não se têm os mesmos dados. Para comparar por conteúdo, defina um método `igual(outro)` que receba o outro objeto e devolva verdadeiro ou falso. A partir daí, `==` (e `!=`) entre objetos daquela classe passam a chamar esse método:

```kaju
classe Ponto {
    construtor(x, y) {
        isto.x = x
        isto.y = y
    }

    metodo igual(outro) {
        retorne isto.x == outro.x e isto.y == outro.y
    }
}

var a = novo Ponto(1, 2)
var b = novo Ponto(1, 2)
escreva(a == b)   // verdadeiro (mesmos dados, ainda que sejam objetos distintos)
```

## Membros estáticos

Os campos e métodos vistos até aqui pertencem a cada objeto. Um membro **estático**, marcado com `estatico`, pertence à classe como um todo — existe uma única cópia, compartilhada por todas as instâncias. Você o acessa pelo nome da classe, e não por um objeto.

Um uso comum é contar quantos objetos foram criados:

```kaju
classe Contador {
    estatico total = 0

    construtor() {
        Contador.total += 1
    }

    estatico metodo quantos() {
        retorne Contador.total
    }
}

novo Contador()
novo Contador()

escreva(Contador.total)      // 2
escreva(Contador.quantos())  // 2
```

O campo `total` não vive em nenhuma instância; ele vive na classe `Contador`, e por isso o construtor o incrementa através de `Contador.total`. Métodos estáticos, por sua vez, funcionam sem um objeto e servem para operações ligadas à classe em geral, não a uma instância específica.

## Registros: classes de dados prontas

Muitas vezes você só quer agrupar alguns valores — um ponto, uma cor, uma pessoa — sem escrever um construtor, um `paraTexto` e um `igual` na mão. Para isso existe o **registro**:

```kaju
registro Ponto(x, y)

var a = Ponto(1, 2)          // pode usar 'novo', mas não é obrigatório
escreva(a)                   // Ponto(1, 2)
escreva(a.x)                 // 1
escreva(a == Ponto(1, 2))    // verdadeiro — compara os campos, não a identidade
```

Um `registro Nome(campos...)` cria uma classe com três coisas prontas:

- um **construtor** que recebe os campos (por posição ou por nome: `Ponto(y: 2, x: 1)`);
- **igualdade estrutural**: dois registros do mesmo tipo são iguais quando todos os campos são iguais — o que também faz `lista.contem(...)` funcionar como esperado;
- um **`paraTexto`** que mostra `Nome(valor1, valor2, ...)`.

Todos os campos são obrigatórios na construção. Um registro guarda dados; quando você precisar de comportamento (métodos), use uma `classe`.

Com funções e objetos você já consegue estruturar programas completos. Quando algo dá errado durante a execução, é hora de tratar [erros e exceções](./erros.md).
