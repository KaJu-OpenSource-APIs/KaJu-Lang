//! Explicações longas dos códigos de erro, exibidas por `kaju explique Kxxx`.
//! Inspirado no `rustc --explain`.
//!
//! Os textos estão em ordem numérica de código. Todo código emitido pelo
//! interpretador precisa ter uma entrada aqui — o teste `todo_codigo_emitido_tem_explicacao`
//! garante isso.

/// Devolve a explicação detalhada de um código de erro, se houver.
pub fn explicar(codigo: &str) -> Option<&'static str> {
    let texto = match codigo {
        "K001" => {
            "\
K001 — variável não definida

Você usou um nome que não foi declarado (nem com 'var'/'constante', nem
como parâmetro de função). Em geral é um erro de digitação ou uma variável
usada antes de ser criada.

Errado:
    escreva(idde)

Correto:
    var idade = 20
    escreva(idade)

Quando o kaju encontra um nome parecido com um existente, ele sugere:
\"você quis dizer 'idade'?\".

Veja também: K002, K213."
        }

        "K002" => {
            "\
K002 — nome de variável esperado

Depois de 'var' ou 'constante' vem o nome da variável.

Errado:
    var = 10

Correto:
    var x = 10

Veja também: K003."
        }

        "K003" => {
            "\
K003 — esperava '=' na declaração

Toda variável recebe um valor inicial com '='. Não existe declaração sem valor.

Errado:
    var x 10

Correto:
    var x = 10

Veja também: K002."
        }

        "K004" => {
            "\
K004 — parênteses ou parâmetros

Falta '(', ')' ou o nome de um parâmetro numa função, chamada ou indexação.

Errado:
    funcao f a) { }

Correto:
    funcao f(a) { }

Veja também: K201."
        }

        "K005" => {
            "\
K005 — esperava '{' ou '}'

Blocos em kaju usam chaves. Um 'se', 'enquanto', 'para', 'funcao', 'classe'
etc. precisa do corpo entre { e }.

Errado:
    se idade >= 18
        escreva(\"maior\")

Correto:
    se idade >= 18 {
        escreva(\"maior\")
    }"
        }

        "K006" => {
            "\
K006 — laço 'para' mal formado

As duas formas válidas são a contagem numérica e a iteração sobre coleções.

Errado:
    para i 1 ate 10 { ... }

Correto:
    para i de 1 ate 10 { ... }
    para cada item em lista { ... }

Veja também: K205."
        }

        "K007" => {
            "\
K007 — atribuição inválida

Só dá para atribuir a uma variável, a um índice ('a[i] = x') ou a um campo
('obj.c = x'). O lado esquerdo de '=' precisa ser um desses.

Errado:
    3 = x

Correto:
    var a = 3
    a = x

Veja também: K009, K217."
        }

        "K008" => {
            "\
K008 — esperava uma expressão

O interpretador esperava um valor (número, texto, nome, chamada...) e
encontrou outra coisa (um operador solto, um parêntese de fechamento, o fim
da linha...).

Errado:
    escreva(2 + )
    var x =

Correto:
    escreva(2 + 3)
    var x = 10"
        }

        "K009" => {
            "\
K009 — reatribuição de constante

Valores declarados com 'constante' não podem ser alterados depois.

Errado:
    constante PI = 3.14
    PI = 4

Correto:
    var raio = 3.14
    raio = 4          // 'var' permite alterar

Veja também: K007."
        }

        "K010" => {
            "\
K010 — dicionário mal formado

As chaves de um dicionário são textos, separadas do valor por ':'.

Errado:
    {nome: \"Ana\"}

Correto:
    {\"nome\": \"Ana\"}

Veja também: K210."
        }

        "K011" => {
            "\
K011 — nome esperado após '.'

Depois de '.' vem o nome de um campo ou método.

Errado:
    obj.

Correto:
    obj.nome

Veja também: K212, K213."
        }

        "K012" => {
            "\
K012 — operação entre tipos incompatíveis

Operadores aritméticos ('-', '*', '/', '%') exigem números dos dois lados.
O '+' também concatena textos, mas '-' entre texto e número não faz sentido.

Errado:
    \"abc\" - 1

Correto:
    \"abc\" + paraTexto(1)   // junta textos -> \"abc1\"
    10 - 1                    // aritmética entre números

Veja também: K203."
        }

        "K013" => {
            "\
K013 — corpo de classe mal formado

Dentro de 'classe' só cabem 'construtor', 'metodo' e membros 'estatico'.

Errado:
    classe Gato {
        var nome = \"\"
    }

Correto:
    classe Gato {
        construtor(nome) { isto.nome = nome }
        metodo falar() { escreva(isto.nome) }
    }

Veja também: K214."
        }

        "K014" => {
            "\
K014 — uso de 'novo' mal formado

'novo' cria uma instância: 'novo NomeDaClasse(argumentos)'.

Errado:
    novo ()

Correto:
    novo Gato(\"Félix\")

Veja também: K218."
        }

        "K015" => {
            "\
K015 — 'tente' mal formado

Um 'tente' precisa de um 'capture (erro) { ... }'; 'finalmente' é opcional.

Errado:
    tente { ... }

Correto:
    tente { ... } capture (erro) { ... } finalmente { ... }

Veja também: K230."
        }

        "K016" => {
            "\
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
    }"
        }

        "K017" => {
            "\
K017 — 'importe' mal formado

O caminho é um texto entre aspas; o apelido (opcional) vem após 'como'.

Errado:
    importe utilidades

Correto:
    importe \"utilidades.kaju\"
    importe \"mat.kaju\" como mat

Veja também: K220."
        }

        "K018" => {
            "\
K018 — erro na interpolação de texto

A expressão dentro de {chaves} num texto $\"...\" é inválida. Verifique-a; para
uma chave literal, use '{{' e '}}'.

Errado:
    escreva($\"total: {a + }\")

Correto:
    escreva($\"total: {a + b}\")
    escreva($\"use {{chaves}} assim\")   // -> use {chaves} assim

Veja também: K104."
        }

        "K019" => {
            "\
K019 — operador condicional sem ':'

A forma é 'condicao ? valorSeVerdadeiro : valorSeFalso'.

Errado:
    x > 0 ? \"pos\"

Correto:
    x > 0 ? \"pos\" : \"neg\""
        }

        "K020" => {
            "\
K020 — divisão por zero

Dividir (ou tirar o resto) por zero não é definido.

Errado:
    var x = 10 / 0

Correto:
    se divisor != 0 {
        escreva(10 / divisor)
    } senao {
        escreva(\"não dá pra dividir por zero\")
    }"
        }

        "K021" => {
            "\
K021 — 'escolha' mal formado

Dentro de 'escolha' só cabem 'caso' (com um ou mais valores) e um 'padrao'.

Errado:
    escolha n {
        1 { ... }
    }

Correto:
    escolha n {
        caso 1 { ... }
        caso 2, 3 { ... }
        padrao { ... }
    }"
        }

        "K022" => {
            "\
K022 — desempacotamento incompatível

O número de nomes precisa bater com o de valores (ou com o tamanho da lista),
e os alvos devem ser nomes de variáveis.

Errado:
    var a, b, c = [1, 2]

Correto:
    var a, b = [1, 2]
    var a, b, c = 1, 2, 3"
        }

        "K023" => {
            "\
K023 — argumento posicional depois de nomeado

Numa chamada, os argumentos nomeados ('nome: valor') vêm sempre depois dos
posicionais. Não é possível voltar a um posicional depois de começar a nomear.

Errado:
    criar(nome: \"Ana\", 30)

Correto:
    criar(30, nome: \"Ana\")
    criar(nome: \"Ana\", idade: 30)"
        }

        "K101" => {
            "\
K101 — caractere inesperado

Apareceu um símbolo que não faz parte do kaju. Confira erros de digitação.
Para 'diferente de' use '!='; para negação lógica, use 'nao'.

Errado:
    se a <> b { ... }
    se @a { ... }

Correto:
    se a != b { ... }
    se nao a { ... }"
        }

        "K102" => {
            "\
K102 — comentário de bloco não fechado

Um comentário '/*' precisa terminar com '*/'.

Errado:
    /* isto é um comentário

Correto:
    /* isto é um comentário */

Veja também: K103."
        }

        "K103" => {
            "\
K103 — texto não fechado

Um texto \"...\" precisa fechar as aspas na mesma linha.

Errado:
    escreva(\"olá)

Correto:
    escreva(\"olá\")

Veja também: K104."
        }

        "K104" => {
            "\
K104 — texto interpolado não fechado

Um texto $\"...\" (ou uma chave '{' dentro dele) não foi fechado na mesma linha.

Errado:
    escreva($\"olá {nome)

Correto:
    escreva($\"olá {nome}\")

Veja também: K018, K103."
        }

        "K201" => {
            "\
K201 — número de argumentos incorreto

Você chamou uma função (ou método/construtor) com uma quantidade de
argumentos diferente da que ela declara.

Errado:
    funcao soma(a, b) { retorne a + b }
    soma(1)            // faltou um argumento

Correto:
    soma(1, 2)

Veja também: K204."
        }

        "K202" => {
            "\
K202 — valor não iterável em 'para cada'

'para cada' percorre listas e dicionários.

Errado:
    para cada x em 42 { ... }

Correto:
    para cada x em [1, 2, 3] { ... }

Veja também: K205."
        }

        "K203" => {
            "\
K203 — erro numa função embutida

Uma função da biblioteca padrão recebeu algo inesperado (tipo ou quantidade de
argumentos errada). A mensagem do diagnóstico detalha o que a função esperava.

Errado:
    raiz(\"nove\")       // raiz espera um número
    maiusculas()        // faltou o texto

Correto:
    raiz(9)
    \"ana\".maiusculas()

Veja também: K012, K201."
        }

        "K204" => {
            "\
K204 — tentativa de chamar algo que não é função

Você usou '( )' de chamada em um valor que não é uma função.

Errado:
    var x = 10
    x()                // 10 não é função

Correto:
    var dobro = funcao(n) { retorne n * 2 }
    dobro(10)

Veja também: K201, K211."
        }

        "K205" => {
            "\
K205 — laço 'para' precisa de números

Os limites de 'para i de A ate B' devem ser números. O passo (em
'para i de A ate B passo P') também precisa ser um número, e não pode ser
zero — um passo zero nunca terminaria o laço.

Errado:
    para i de \"a\" ate 10 { ... }
    para i de 1 ate 10 passo 0 { ... }

Correto:
    para i de 1 ate 10 { ... }
    para i de 10 ate 1 passo -1 { ... }   // contagem regressiva

Veja também: K006, K202."
        }

        "K206" => {
            "\
K206 — índice fora da lista/texto

Você acessou uma posição que não existe. Os índices começam em 0 e vão até
tamanho - 1.

Errado:
    var l = [10, 20]
    escreva(l[5])      // só existem l[0] e l[1]

Correto:
    escreva(l[0], l[tamanho(l) - 1])

Veja também: K207, K209."
        }

        "K207" => {
            "\
K207 — índice inválido

Índices de lista e texto devem ser inteiros não negativos (0, 1, 2, ...).

Errado:
    lista[-1]
    lista[1.5]

Correto:
    lista[0]
    lista[tamanho(lista) - 1]

Veja também: K206, K209."
        }

        "K208" => {
            "\
K208 — chave inexistente no dicionário

Você leu uma chave que não está no dicionário. Para evitar o erro, verifique
antes com '.tem(chave)' ou use '.obtem(chave, padrao)'.

Errado:
    var d = {\"a\": 1}
    escreva(d[\"b\"])   // não existe

Correto:
    escreva(d.obtem(\"b\", 0))       // 0 se não existir
    se d.tem(\"b\") { escreva(d[\"b\"]) }

Veja também: K210."
        }

        "K209" => {
            "\
K209 — valor não indexável

Apenas listas, textos e dicionários aceitam indexação com [].

Errado:
    42[0]

Correto:
    [10, 20, 30][0]
    \"abc\"[0]

Veja também: K206, K210."
        }

        "K210" => {
            "\
K210 — chave de dicionário deve ser texto

As chaves de um dicionário são textos.

Errado:
    var d = {\"3\": \"tres\"}
    escreva(d[3])

Correto:
    escreva(d[\"3\"])

Veja também: K010, K208."
        }

        "K211" => {
            "\
K211 — método usado sem chamá-lo

Métodos precisam ser chamados com parênteses. Sem '( )', você está apenas
referenciando o nome.

Errado:
    escreva(lista.tamanho)

Correto:
    escreva(lista.tamanho())

Veja também: K204, K212."
        }

        "K212" => {
            "\
K212 — método inexistente

O tipo (ou a classe do objeto) não tem um método com esse nome. Confira a
grafia e a lista de métodos daquele tipo na seção da biblioteca padrão da
especificação.

Errado:
    [1, 2].gire()

Correto:
    [1, 2].inverta()

Veja também: K211, K213."
        }

        "K213" => {
            "\
K213 — membro inexistente no objeto

O objeto não tem esse campo nem método. Confira o nome e a classe do objeto.
Lembre-se de que campos nascem ao atribuir 'isto.campo = ...' no construtor.

Errado:
    classe Gato { construtor(nome) { isto.nome = nome } }
    var g = novo Gato(\"Félix\")
    escreva(g.idade)     // 'idade' nunca foi definido

Correto:
    escreva(g.nome)

Veja também: K212, K214."
        }

        "K214" => {
            "\
K214 — 'isto' fora de um método

'isto' referencia a instância atual e só existe dentro de um 'metodo' ou
'construtor' de uma classe.

Errado:
    funcao falar() { escreva(isto.nome) }   // fora de uma classe

Correto:
    classe Gato {
        construtor(nome) { isto.nome = nome }
        metodo falar() { escreva(isto.nome) }
    }

Veja também: K213, K215."
        }

        "K215" => {
            "\
K215 — uso inválido de 'base'

'base' só pode ser usado dentro de um método, para chamar a superclasse
('base.metodo()'), e a classe precisa ter uma superclasse (declarada com 'herda').

Errado:
    classe Gato {                 // sem 'herda'
        metodo falar() { base.falar() }
    }

Correto:
    classe Animal { metodo falar() { escreva(\"...\") } }
    classe Gato herda Animal {
        metodo falar() { base.falar() }
    }

Veja também: K214, K216."
        }

        "K216" => {
            "\
K216 — superclasse inválida

O nome após 'herda' precisa ser uma classe já definida.

Errado:
    classe Gato herda Animal { ... }   // 'Animal' não existe

Correto:
    classe Animal { ... }
    classe Gato herda Animal { ... }

Veja também: K215, K218."
        }

        "K217" => {
            "\
K217 — atribuição de campo em não-objeto

Só objetos (e classes, para campos estáticos) têm campos atribuíveis com '.'.

Errado:
    var x = 42
    x.campo = 1

Correto:
    classe Ponto { construtor() {} }
    var p = novo Ponto()
    p.campo = 1

Veja também: K007, K213."
        }

        "K218" => {
            "\
K218 — 'novo' usado com algo que não é classe

O operador 'novo' cria uma instância e precisa de um nome de classe.

Errado:
    var x = 5
    novo x()

Correto:
    classe Ponto { construtor(a) { isto.a = a } }
    novo Ponto(1)

Veja também: K014, K216."
        }

        "K220" => {
            "\
K220 — arquivo de módulo não encontrado

Um 'importe' aponta para um arquivo que não existe. O caminho é relativo ao
arquivo que importa.

Errado:
    importe \"naoexiste.kaju\"

Correto:
    importe \"utilidades.kaju\"   // arquivo ao lado deste

Veja também: K017, K221."
        }

        "K221" => {
            "\
K221 — erro dentro de um módulo importado

O 'importe' encontrou o arquivo, mas ocorreu um erro ao executá-lo. A nota do
diagnóstico aponta a linha e o erro interno do módulo.

Errado:
    // em util.kaju:  var x = 10 / 0
    importe \"util.kaju\"       // falha ao rodar util.kaju

Correto:
    // corrija o erro dentro de util.kaju e importe de novo
    importe \"util.kaju\"

Veja também: K220."
        }

        "K222" => {
            "\
K222 — estouro de inteiro (histórico)

Este código não é mais emitido. Antes, uma operação com inteiros cujo resultado
passasse do alcance de 64 bits parava o programa. Hoje o kaju usa inteiros de
precisão arbitrária: quando o resultado excede esse alcance, ele é promovido
automaticamente, sem perda de precisão.

    var x = 9223372036854775807 + 1   // 9223372036854775808 (exato)

Os valores voltam a caber em 64 bits sempre que possível, então isso é
transparente. Números com casas decimais continuam sendo 'decimal' (f64)."
        }

        "K224" => {
            "\
K224 — parâmetro nomeado inexistente

Um argumento nomeado usa um nome que a função (ou o construtor) não tem entre
seus parâmetros. Confira a grafia do parâmetro. O parâmetro variádico ('...')
não pode ser passado por nome.

Errado:
    funcao saudar(nome) { ... }
    saudar(nomee: \"Ana\")

Correto:
    saudar(nome: \"Ana\")"
        }

        "K225" => {
            "\
K225 — argumento informado mais de uma vez

Um parâmetro recebeu valor duas vezes — por exemplo, uma vez por posição e de
novo por nome, ou o mesmo nome repetido.

Errado:
    funcao criar(nome, idade) { ... }
    criar(\"Ana\", nome: \"Bia\")

Correto:
    criar(\"Ana\", 30)
    criar(nome: \"Ana\", idade: 30)"
        }

        "K226" => {
            "\
K226 — argumentos nomeados não suportados aqui

Argumentos nomeados só funcionam com funções, métodos e construtores definidos
em kaju. Funções embutidas e métodos de coleção (como 'mapeie') recebem os
argumentos apenas por posição.

Errado:
    [1, 2, 3].mapeie(f: dobro)

Correto:
    [1, 2, 3].mapeie(dobro)"
        }

        "K227" => {
            "\
K227 — espalhamento inválido

O operador '...' (espalhamento) expande uma coleção dentro de um literal ou de
uma lista de argumentos. Em listas e argumentos ele espera uma 'lista'; em
literais de dicionário, um 'dicionario'.

Errado:
    var xs = [...10]
    maximo(...\"texto\")

Correto:
    var xs = [...[1, 2], 3]
    var ys = [...listaA, ...listaB]
    maximo(...numeros)
    var d = {...base, \"nova\": 1}"
        }

        "K230" => {
            "\
K230 — erro lançado e não capturado

Um 'lance' chegou ao topo do programa sem ser tratado. Envolva o código que
pode falhar em 'tente ... capture'.

Errado:
    lance \"algo deu errado\"    // ninguém captura

Correto:
    tente {
        lance \"algo deu errado\"
    } capture (erro) {
        escreva(\"tratei:\", erro.mensagem)
    }

Veja também: K015."
        }

        "K231" => {
            "\
K231 — afirmação falhou

A função 'afirme(condicao)' verifica que a condição é verdadeira; se for falsa,
o programa para com este erro. É útil para escrever testes na própria linguagem.
Você pode passar uma mensagem: 'afirme(condicao, \"mensagem\")'.

Errado:
    afirme(2 + 2 == 5)
    afirme(soma(1, 2) == 4, \"soma deu errado\")

Correto:
    afirme(2 + 2 == 4)
    afirme(soma(1, 2) == 3)

Como um erro de runtime, pode ser capturado com 'tente ... capture'.

Veja também: K230."
        }

        _ => return None,
    };
    Some(texto)
}

/// Lista os códigos que têm explicação detalhada.
pub fn codigos_conhecidos() -> &'static [&'static str] {
    &[
        "K001", "K002", "K003", "K004", "K005", "K006", "K007", "K008", "K009", "K010", "K011",
        "K012", "K013", "K014", "K015", "K016", "K017", "K018", "K019", "K020", "K021", "K022",
        "K023", "K101", "K102", "K103", "K104", "K201", "K202", "K203", "K204", "K205", "K206",
        "K207", "K208", "K209", "K210", "K211", "K212", "K213", "K214", "K215", "K216", "K217",
        "K218", "K220", "K221", "K222", "K224", "K225", "K226", "K227", "K230", "K231",
    ]
}

#[cfg(test)]
mod testes {
    use super::*;
    use std::collections::BTreeSet;

    /// Todo código listado em `codigos_conhecidos` deve ter um texto em `explicar`.
    #[test]
    fn todo_codigo_conhecido_tem_texto() {
        for codigo in codigos_conhecidos() {
            assert!(
                explicar(codigo).is_some(),
                "o código {codigo} está em codigos_conhecidos() mas não tem texto em explicar()"
            );
        }
    }

    /// Salvaguarda de paridade: varre o código-fonte em busca de códigos "Kxxx"
    /// emitidos e garante que cada um tenha explicação. Se alguém adicionar um
    /// erro novo sem explicá-lo, este teste falha.
    #[test]
    fn todo_codigo_emitido_tem_explicacao() {
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/src");
        let mut emitidos: BTreeSet<String> = BTreeSet::new();

        for entrada in std::fs::read_dir(dir).expect("ler src/") {
            let caminho = entrada.expect("entrada de src/").path();
            // Ignora este próprio arquivo (onde os códigos aparecem como chaves).
            if caminho.file_name().and_then(|n| n.to_str()) == Some("explicacoes.rs") {
                continue;
            }
            if caminho.extension().and_then(|e| e.to_str()) != Some("rs") {
                continue;
            }
            let fonte = std::fs::read_to_string(&caminho).expect("ler arquivo .rs");
            coletar_codigos(&fonte, &mut emitidos);
        }

        assert!(
            !emitidos.is_empty(),
            "nenhum código Kxxx encontrado no fonte — o teste está procurando no lugar errado?"
        );

        for codigo in &emitidos {
            assert!(
                explicar(codigo).is_some(),
                "o código {codigo} é emitido em src/ mas não tem explicação em 'kaju explique'.\n\
                 Adicione uma entrada em src/explicacoes.rs e inclua-o em codigos_conhecidos()."
            );
            assert!(
                codigos_conhecidos().contains(&codigo.as_str()),
                "o código {codigo} é emitido mas não está em codigos_conhecidos()."
            );
        }
    }

    /// Extrai ocorrências de códigos no formato "Kddd" de um trecho de fonte.
    fn coletar_codigos(fonte: &str, destino: &mut BTreeSet<String>) {
        let bytes = fonte.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            // procura o padrão: " K ddd " entre aspas
            if bytes[i] == b'"'
                && i + 5 < bytes.len()
                && bytes[i + 1] == b'K'
                && bytes[i + 2].is_ascii_digit()
                && bytes[i + 3].is_ascii_digit()
                && bytes[i + 4].is_ascii_digit()
                && bytes[i + 5] == b'"'
            {
                destino.insert(fonte[i + 1..i + 5].to_string());
                i += 6;
            } else {
                i += 1;
            }
        }
    }
}
