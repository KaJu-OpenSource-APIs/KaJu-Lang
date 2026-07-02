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

        _ => return None,
    };
    Some(texto)
}

/// Lista os códigos que têm explicação detalhada.
pub fn codigos_conhecidos() -> &'static [&'static str] {
    &[
        "K001", "K005", "K008", "K009", "K012", "K016", "K020", "K201", "K204",
        "K206", "K208", "K211", "K212", "K214", "K218", "K220", "K230",
    ]
}
