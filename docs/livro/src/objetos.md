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

Com funções e objetos você já consegue estruturar programas completos. Quando algo dá errado durante a execução, é hora de tratar [erros e exceções](./erros.md).
