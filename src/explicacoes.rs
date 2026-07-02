//! Explicações longas dos códigos de erro, exibidas por `kaju explique Kxxx`.
//! Inspirado no `rustc --explain`.

/// Devolve a explicação detalhada de um código de erro, se houver.
pub fn explicar(codigo: &str) -> Option<&'static str> {
    let texto = match codigo {
        "K001" => "\
K001 — variável não definida

Você usou um nome que não foi declarado (nem com 'var'/'constante', nem
como parâmetro de função). Em geral é um erro de digitação ou uma variável
usada antes de ser criada.

Errado:
    escreva(idde)

Correto:
    var idade = 20
    escreva(idade)

Quando a kaju encontra um nome parecido com um existente, ela sugere:
\"você quis dizer 'idade'?\".",

        "K005" => "\
K005 — esperava '{' ou '}'

Blocos em kaju usam chaves. Um 'se', 'enquanto', 'para', 'funcao', 'classe'
etc. precisa do corpo entre { e }.

Errado:
    se idade >= 18
        escreva(\"maior\")

Correto:
    se idade >= 18 {
        escreva(\"maior\")
    }",

        "K008" => "\
K008 — esperava uma expressão

O interpretador esperava um valor (número, texto, nome, chamada...) e
encontrou outra coisa (um operador solto, um parêntese de fechamento, o fim
da linha...).

Errado:
    escreva(2 + )
    var x =

Correto:
    escreva(2 + 3)
    var x = 10",

        "K009" => "\
K009 — reatribuição de constante

Valores declarados com 'constante' não podem ser alterados depois.

Errado:
    constante PI = 3.14
    PI = 4          // erro

Correto:
    var raio = 3.14
    raio = 4        // 'var' permite alterar",

        "K012" => "\
K012 — operação entre tipos incompatíveis

Operadores aritméticos ('-', '*', '/', '%') exigem números dos dois lados.
O '+' também concatena textos, mas '-' entre texto e número não faz sentido.

Errado:
    \"abc\" - 1

Correto:
    \"abc\" + paraTexto(1)   // junta textos -> \"abc1\"
    10 - 1                    // aritmética entre números",

        "K016" => "\
K016 — comando de fluxo fora de contexto

'pare' e 'continue' só fazem sentido dentro de um laço ('enquanto'/'para'),
e 'retorne' só dentro de uma função. Uma função também interrompe o alcance
dos laços: um 'pare' dentro de uma função (sem laço próprio) não afeta laços
de fora.

Errado:
    retorne 5          // fora de qualquer função

Correto:
    funcao f() {
        retorne 5
    }",

        "K020" => "\
K020 — divisão por zero

Dividir (ou tirar o resto) por zero não é definido.

Errado:
    var x = 10 / 0

Como evitar:
    se divisor != 0 {
        escreva(10 / divisor)
    } senao {
        escreva(\"não dá pra dividir por zero\")
    }",

        "K201" => "\
K201 — número de argumentos incorreto

Você chamou uma função (ou método/construtor) com uma quantidade de
argumentos diferente da que ela declara.

Errado:
    funcao soma(a, b) { retorne a + b }
    soma(1)            // faltou um argumento

Correto:
    soma(1, 2)",

        "K204" => "\
K204 — tentativa de chamar algo que não é função

Você usou '( )' de chamada em um valor que não é uma função.

Errado:
    var x = 10
    x()                // 10 não é função

Correto:
    var dobro = funcao(n) { retorne n * 2 }
    dobro(10)",

        "K206" => "\
K206 — índice fora da lista/texto

Você acessou uma posição que não existe. Os índices começam em 0 e vão até
tamanho - 1.

Errado:
    var l = [10, 20]
    escreva(l[5])      // só existem l[0] e l[1]

Correto:
    escreva(l[0], l[tamanho(l) - 1])",

        "K208" => "\
K208 — chave inexistente no dicionário

Você leu uma chave que não está no dicionário. Para evitar o erro, verifique
antes com '.tem(chave)' ou use '.obtem(chave, padrao)'.

Errado:
    var d = {\"a\": 1}
    escreva(d[\"b\"])   // não existe

Correto:
    escreva(d.obtem(\"b\", 0))   // 0 se não existir
    se d.tem(\"b\") { escreva(d[\"b\"]) }",

        "K211" => "\
K211 — método usado sem chamá-lo

Métodos precisam ser chamados com parênteses. Sem '( )', você está apenas
referenciando o nome.

Errado:
    escreva(lista.tamanho)

Correto:
    escreva(lista.tamanho())",

        "K212" => "\
K212 — método inexistente

O tipo (ou a classe do objeto) não tem um método com esse nome. Confira a
grafia e a lista de métodos daquele tipo na especificação (§10.4).

Errado:
    [1, 2].gire()

Correto:
    [1, 2].inverta()",

        "K214" => "\
K214 — 'isto' fora de um método

'isto' referencia a instância atual e só existe dentro de um 'metodo' ou
'construtor' de uma classe.

Correto:
    classe Gato {
        construtor(nome) { isto.nome = nome }
        metodo falar() { escreva(isto.nome) }
    }",

        "K218" => "\
K218 — 'novo' usado com algo que não é classe

O operador 'novo' cria uma instância e precisa de um nome de classe.

Errado:
    var x = 5
    novo x()

Correto:
    classe Ponto { construtor(a) { isto.a = a } }
    novo Ponto(1)",

        "K220" => "\
K220 — arquivo de módulo não encontrado

Um 'importe' aponta para um arquivo que não existe. O caminho é relativo ao
arquivo que importa.

Errado:
    importe \"naoexiste.kaju\"

Correto:
    importe \"utilidades.kaju\"   // arquivo ao lado deste",

        "K230" => "\
K230 — erro lançado e não capturado

Um 'lance' chegou ao topo do programa sem ser tratado. Envolva o código que
pode falhar em 'tente ... capture'.

Correto:
    tente {
        lance \"algo deu errado\"
    } capture (erro) {
        escreva(\"tratei:\", erro.mensagem)
    }",

        "K002" => "\
K002 — nome de variável esperado

Depois de 'var' ou 'constante' vem o nome da variável.

Errado:   var = 10
Correto:  var x = 10",

        "K003" => "\
K003 — esperava '=' na declaração

Toda variável recebe um valor inicial com '='.

Errado:   var x 10
Correto:  var x = 10",

        "K004" => "\
K004 — parênteses ou parâmetros

Falta '(' , ')' ou o nome de um parâmetro numa função, chamada ou indexação.

Errado:   funcao f a) { }
Correto:  funcao f(a) { }",

        "K006" => "\
K006 — laço 'para' mal formado

As formas válidas são:
    para i de 1 ate 10 { ... }
    para cada item em lista { ... }",

        "K007" => "\
K007 — atribuição inválida

Só dá para atribuir a uma variável, a um índice ('a[i] = x') ou a um campo
('obj.c = x'). O lado esquerdo de '=' precisa ser um desses.

Errado:   3 = x",

        "K010" => "\
K010 — dicionário mal formado

As chaves de um dicionário são textos, separadas do valor por ':'.

Errado:   {nome: \"Ana\"}
Correto:  {\"nome\": \"Ana\"}",

        "K011" => "\
K011 — nome esperado após '.'

Depois de '.' vem o nome de um campo ou método.

Errado:   obj.
Correto:  obj.nome",

        "K013" => "\
K013 — corpo de classe mal formado

Dentro de 'classe' só cabem 'construtor', 'metodo' e membros 'estatico'.

    classe Gato {
        construtor(nome) { isto.nome = nome }
        metodo falar() { escreva(isto.nome) }
    }",

        "K014" => "\
K014 — uso de 'novo' mal formado

'novo' cria uma instância: 'novo NomeDaClasse(argumentos)'.

Errado:   novo ()
Correto:  novo Gato(\"Félix\")",

        "K015" => "\
K015 — 'tente' mal formado

Um 'tente' precisa de um 'capture (erro) { ... }'; 'finalmente' é opcional.

    tente { ... } capture (erro) { ... } finalmente { ... }",

        "K017" => "\
K017 — 'importe' mal formado

O caminho é um texto entre aspas; o apelido (opcional) vem após 'como'.

    importe \"utilidades.kaju\"
    importe \"mat.kaju\" como mat",

        "K018" => "\
K018 — erro na interpolação de texto

A expressão dentro de {chaves} num texto $\"...\" é inválida. Verifique-a; para
uma chave literal, use '{{' e '}}'.

    escreva($\"total: {a + b}\")",

        "K019" => "\
K019 — operador condicional sem ':'

A forma é 'condicao ? valorSeVerdadeiro : valorSeFalso'.

Errado:   x > 0 ? \"pos\"
Correto:  x > 0 ? \"pos\" : \"neg\"",

        "K021" => "\
K021 — 'escolha' mal formado

Dentro de 'escolha' só cabem 'caso' (com um ou mais valores) e um 'padrao'.

    escolha n {
        caso 1 { ... }
        caso 2, 3 { ... }
        padrao { ... }
    }",

        "K022" => "\
K022 — desempacotamento incompatível

O número de nomes precisa bater com o de valores (ou com o tamanho da lista),
e os alvos devem ser nomes de variáveis.

Errado:   var a, b, c = [1, 2]
Correto:  var a, b = [1, 2]        ou      var a, b, c = 1, 2, 3",

        "K101" => "\
K101 — caractere inesperado

Apareceu um símbolo que não faz parte da kaju. Confira erros de digitação.
Para 'diferente de' use '!='; para negação lógica, 'nao'.",

        "K102" => "\
K102 — comentário de bloco não fechado

Um comentário '/*' precisa terminar com '*/'.",

        "K103" => "\
K103 — texto não fechado

Um texto \"...\" precisa fechar as aspas na mesma linha.

Errado:   escreva(\"olá)",

        "K104" => "\
K104 — texto interpolado não fechado

Um texto $\"...\" (ou uma chave '{' dentro dele) não foi fechado na mesma linha.",

        "K202" => "\
K202 — valor não iterável em 'para cada'

'para cada' percorre listas e dicionários.

Errado:   para cada x em 42 { ... }
Correto:  para cada x em [1, 2, 3] { ... }",

        "K203" => "\
K203 — erro numa função embutida

Uma função da biblioteca padrão recebeu algo inesperado (tipo ou quantidade de
argumentos errada). A mensagem detalha o que a função esperava.",

        "K205" => "\
K205 — laço 'para' precisa de números

Os limites de 'para i de A ate B' devem ser números.

Errado:   para i de \"a\" ate 10 { ... }",

        "K207" => "\
K207 — índice inválido

Índices de lista e texto devem ser inteiros não negativos (0, 1, 2, ...).

Errado:   lista[-1]     lista[1.5]",

        "K209" => "\
K209 — valor não indexável

Apenas listas, textos e dicionários aceitam indexação com [].

Errado:   42[0]",

        "K210" => "\
K210 — chave de dicionário deve ser texto

As chaves de um dicionário são textos.

Errado:   d[3]
Correto:  d[\"3\"]",

        "K213" => "\
K213 — membro inexistente no objeto

O objeto não tem esse campo nem método. Confira o nome e a classe do objeto.
Lembre-se de que campos nascem ao atribuir 'isto.campo = ...' no construtor.",

        "K215" => "\
K215 — uso inválido de 'base'

'base' só pode ser usado dentro de um método, para chamar a superclasse
('base.metodo()'), e a classe precisa ter uma superclasse (declarada com 'herda').",

        "K216" => "\
K216 — superclasse inválida

O nome após 'herda' precisa ser uma classe já definida.

    classe Animal { ... }
    classe Gato herda Animal { ... }",

        "K217" => "\
K217 — atribuição de campo em não-objeto

Só objetos (e classes, para campos estáticos) têm campos atribuíveis com '.'.

Errado:   42.x = 1",

        "K221" => "\
K221 — erro dentro de um módulo importado

O 'importe' encontrou o arquivo, mas ocorreu um erro ao executá-lo. A nota do
diagnóstico aponta a linha e o erro interno do módulo.",

        _ => return None,
    };
    Some(texto)
}

/// Lista os códigos que têm explicação detalhada.
pub fn codigos_conhecidos() -> &'static [&'static str] {
    &[
        "K001", "K002", "K003", "K004", "K005", "K006", "K007", "K008", "K009",
        "K010", "K011", "K012", "K013", "K014", "K015", "K016", "K017", "K018",
        "K019", "K020", "K021", "K022", "K101", "K102", "K103", "K104", "K201",
        "K202", "K203", "K204", "K205", "K206", "K207", "K208", "K209", "K210",
        "K211", "K212", "K213", "K214", "K215", "K216", "K217", "K218", "K220",
        "K221", "K230",
    ]
}
