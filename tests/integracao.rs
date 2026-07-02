//! Testes de integração: rodam programas .kaju de verdade e conferem a saída.
//! Esta é a garantia de que os exemplos da documentação nunca ficam desatualizados.

use std::io::Write;
use std::process::{Command, Stdio};

/// Roda uma fonte kaju gravando num arquivo temporário e devolve (stdout, stderr, sucesso).
fn rodar(fonte: &str) -> (String, String, bool) {
    let bin = env!("CARGO_BIN_EXE_kaju");
    let dir = std::env::temp_dir();
    // nome de arquivo único por conteúdo, sem depender de aleatoriedade
    let nome = format!("kaju_teste_{:x}.kaju", hash(fonte));
    let caminho = dir.join(nome);
    std::fs::write(&caminho, fonte).unwrap();

    let saida = Command::new(bin).arg(&caminho).output().unwrap();
    let _ = std::fs::remove_file(&caminho);

    (
        String::from_utf8_lossy(&saida.stdout).to_string(),
        String::from_utf8_lossy(&saida.stderr).to_string(),
        saida.status.success(),
    )
}

/// Roda uma fonte kaju alimentando `entrada` no stdin.
fn rodar_com_entrada(fonte: &str, entrada: &str) -> (String, bool) {
    let bin = env!("CARGO_BIN_EXE_kaju");
    let dir = std::env::temp_dir();
    let caminho = dir.join(format!("kaju_in_{:x}.kaju", hash(fonte)));
    std::fs::write(&caminho, fonte).unwrap();
    let mut filho = Command::new(bin)
        .arg(&caminho)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    filho
        .stdin
        .take()
        .unwrap()
        .write_all(entrada.as_bytes())
        .unwrap();
    let saida = filho.wait_with_output().unwrap();
    let _ = std::fs::remove_file(&caminho);
    (
        String::from_utf8_lossy(&saida.stdout).to_string(),
        saida.status.success(),
    )
}

fn hash(s: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
}

/// Cria um diretório temporário com vários arquivos .kaju e roda o `principal`.
fn rodar_projeto(arquivos: &[(&str, &str)], principal: &str) -> (String, String, bool) {
    let bin = env!("CARGO_BIN_EXE_kaju");
    let combinado: String = arquivos.iter().map(|(n, c)| format!("{n}{c}")).collect();
    let dir = std::env::temp_dir().join(format!("kaju_proj_{:x}", hash(&combinado)));
    std::fs::create_dir_all(&dir).unwrap();
    for (nome, conteudo) in arquivos {
        std::fs::write(dir.join(nome), conteudo).unwrap();
    }
    let saida = Command::new(bin).arg(dir.join(principal)).output().unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    (
        String::from_utf8_lossy(&saida.stdout).to_string(),
        String::from_utf8_lossy(&saida.stderr).to_string(),
        saida.status.success(),
    )
}

#[test]
fn comando_explique() {
    let bin = env!("CARGO_BIN_EXE_kaju");
    let saida = Command::new(bin).args(["explique", "K016"]).output().unwrap();
    assert!(saida.status.success());
    let texto = String::from_utf8_lossy(&saida.stdout);
    assert!(texto.contains("K016"), "stdout: {texto}");
    assert!(texto.contains("fluxo"), "stdout: {texto}");

    // forma curta e maiúscula/minúscula
    let s2 = Command::new(bin).args(["explique", "k020"]).output().unwrap();
    assert!(String::from_utf8_lossy(&s2.stdout).contains("divisão por zero"));
}

#[test]
fn io_terminal_pergunte_e_sem_quebra() {
    let fonte = r#"
        escrevaSemQuebra("a", "b")
        escreva("c")
        var nome = pergunte("nome? ")
        escreva("oi " + nome)
    "#;
    let (out, ok) = rodar_com_entrada(fonte, "Ana\n");
    assert!(ok);
    // "a b" sem quebra, depois "c\n"; a pergunta imprime "nome? " sem quebra
    assert_eq!(out, "a bc\nnome? oi Ana\n");
}

#[test]
fn ola_mundo() {
    let (out, _, ok) = rodar(r#"escreva("Olá, mundo!")"#);
    assert!(ok);
    assert_eq!(out.trim(), "Olá, mundo!");
}

#[test]
fn aritmetica_e_concatenacao() {
    let (out, _, ok) = rodar(
        r#"
        escreva(2 + 3 * 4)
        escreva("total: " + 7)
        escreva(10 % 3)
    "#,
    );
    assert!(ok);
    assert_eq!(out, "14\ntotal: 7\n1\n");
}

#[test]
fn parser_relata_varios_erros() {
    // dois erros independentes; o parser deve relatar ambos, não só o primeiro
    let (_, err, ok) = rodar("var a = )\nvar b = )");
    assert!(!ok);
    let ocorrencias = err.matches("erro[K008]").count();
    assert!(ocorrencias >= 2, "esperava 2+ erros, stderr: {err}");
    assert!(err.contains("erros encontrados"), "stderr: {err}");
}

#[test]
fn erro_pare_fora_de_laco() {
    let (_, err, ok) = rodar("escreva(1)\npare");
    assert!(!ok);
    assert!(err.contains("erro[K016]"), "stderr: {err}");
}

#[test]
fn erro_retorne_fora_de_funcao() {
    let (_, err, ok) = rodar("retorne 5");
    assert!(!ok);
    assert!(err.contains("erro[K016]"), "stderr: {err}");
}

#[test]
fn pare_dentro_de_funcao_sem_laco_falha() {
    let (_, err, ok) = rodar("funcao f() { pare }\nf()");
    assert!(!ok);
    assert!(err.contains("erro[K016]"), "stderr: {err}");
}

#[test]
fn interpolacao_de_texto() {
    let (out, err, ok) = rodar(
        r#"
        var nome = "Ana"
        var n = 3
        escreva($"Oi {nome}, {n} + 1 = {n + 1}")
        var d = {"c": "Recife"}
        escreva($"cidade {d["c"]} e chaves {{}}")
    "#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "Oi Ana, 3 + 1 = 4\ncidade Recife e chaves {}\n");
}

#[test]
fn escolha_caso() {
    let (out, err, ok) = rodar(
        r#"
        funcao classifica(n) {
            escolha n {
                caso 1 { retorne "um" }
                caso 2, 3 { retorne "dois ou tres" }
                padrao { retorne "outro" }
            }
        }
        escreva(classifica(1), classifica(3), classifica(9))
    "#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "um dois ou tres outro\n");
}

#[test]
fn operador_ternario() {
    let (out, err, ok) = rodar(
        r#"
        escreva(20 >= 18 ? "adulto" : "menor")
        var x = 0
        escreva(x > 0 ? "pos" : x < 0 ? "neg" : "zero")
    "#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "adulto\nzero\n");
}

#[test]
fn atribuicao_composta() {
    let (out, err, ok) = rodar(
        r#"
        var x = 10
        x += 5
        x *= 2
        x -= 1
        escreva(x)
        var l = [1, 2, 3]
        l[1] += 10
        escreva(l)
        var d = {"n": 0}
        d["n"] += 7
        escreva(d["n"])
    "#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "29\n[1, 12, 3]\n7\n");
}

#[test]
fn operadores_de_bits() {
    let (out, err, ok) = rodar(
        r#"
        escreva(5 & 3, 5 | 2, 5 ^ 1)
        escreva(~5)
        escreva(1 << 4, 256 >> 2)
        escreva(1 | 2 & 3)
    "#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "1 7 4\n-6\n16 64\n3\n");
}

#[test]
fn senaose_palavra_unica_e_legado() {
    let (out, err, ok) = rodar(
        r#"
        funcao c(n) {
            se n >= 90 { retorne "A" }
            senaose n >= 80 { retorne "B" }
            senao { retorne "C" }
        }
        escreva(c(95), c(85), c(50))
        var x = 5
        se x > 10 { escreva("g") } senao se x > 3 { escreva("m") } senao { escreva("p") }
    "#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "A B C\nm\n");
}

#[test]
fn condicional_e_logica() {
    let (out, _, ok) = rodar(
        r#"
        var idade = 20
        se idade >= 18 e nao (idade > 65) {
            escreva("adulto")
        } senao {
            escreva("outro")
        }
    "#,
    );
    assert!(ok);
    assert_eq!(out.trim(), "adulto");
}

#[test]
fn lacos_e_controle() {
    let (out, _, ok) = rodar(
        r#"
        var soma = 0
        para i de 1 ate 5 { soma = soma + i }
        escreva(soma)
        para cada x em [10, 20, 30] { escreva(x) }
    "#,
    );
    assert!(ok);
    assert_eq!(out, "15\n10\n20\n30\n");
}

#[test]
fn desempacotamento() {
    let (out, err, ok) = rodar(
        r#"
        var a, b = 1, 2
        escreva(a, b)
        a, b = b, a
        escreva(a, b)
        var x, y, z = [10, 20, 30]
        escreva(x, y, z)
    "#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "1 2\n2 1\n10 20 30\n");
}

#[test]
fn erro_desempacotamento_tamanho() {
    let (_, err, ok) = rodar("var a, b, c = [1, 2]");
    assert!(!ok);
    assert!(err.contains("erro[K022]"), "stderr: {err}");
}

#[test]
fn parametros_padrao_e_variadicos() {
    let (out, err, ok) = rodar(
        r#"
        funcao saudar(nome, saud = "Olá") { retorne saud + ", " + nome }
        escreva(saudar("Ana"))
        escreva(saudar("Beto", "Oi"))
        funcao soma(...ns) {
            var t = 0
            para cada n em ns { t += n }
            retorne t
        }
        escreva(soma(), soma(1, 2, 3, 4))
    "#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "Olá, Ana\nOi, Beto\n0 10\n");
}

#[test]
fn erro_argumentos_faltando() {
    let (_, err, ok) = rodar("funcao f(a, b) { retorne a }\nf(1)");
    assert!(!ok);
    assert!(err.contains("erro[K201]"), "stderr: {err}");
}

#[test]
fn funcoes_e_closures() {
    let (out, _, ok) = rodar(
        r#"
        funcao criar() {
            var t = 0
            retorne funcao() { t = t + 1  retorne t }
        }
        var p = criar()
        escreva(p(), p(), p())
    "#,
    );
    assert!(ok);
    assert_eq!(out.trim(), "1 2 3");
}

#[test]
fn indexacao_lista_e_texto() {
    let (out, _, ok) = rodar(
        r#"
        var l = [10, 20, 30]
        escreva(l[0], l[2])
        l[1] = 99
        escreva(l)
        var p = "caju"
        escreva(p[0])
    "#,
    );
    assert!(ok);
    assert_eq!(out, "10 30\n[10, 99, 30]\nc\n");
}

#[test]
fn dicionarios() {
    let (out, _, ok) = rodar(
        r#"
        var d = {"nome": "Ana", "idade": 30}
        escreva(d["nome"])
        d["idade"] = 31
        d["cidade"] = "Recife"
        escreva(tamanho(d))
        para cada c em d { escreva(c) }
    "#,
    );
    assert!(ok);
    // chaves ordenadas: cidade, idade, nome
    assert_eq!(out, "Ana\n3\ncidade\nidade\nnome\n");
}

#[test]
fn metodos_de_lista() {
    let (out, _, ok) = rodar(
        r#"
        var l = [3, 1, 2]
        l.adicione(4)
        escreva(l.tamanho())
        escreva(l.contem(2))
        l.inverta()
        escreva(l.junte("-"))
    "#,
    );
    assert!(ok);
    assert_eq!(out, "4\nverdadeiro\n4-2-1-3\n");
}

#[test]
fn metodos_de_texto() {
    let (out, _, ok) = rodar(
        r#"
        escreva("  oi  ".apara().maiusculas())
        escreva("a,b,c".divida(",").junte("+"))
        escreva("caju".contem("aj"))
    "#,
    );
    assert!(ok);
    assert_eq!(out, "OI\na+b+c\nverdadeiro\n");
}

#[test]
fn metodos_de_dicionario() {
    let (out, _, ok) = rodar(
        r#"
        var d = {"b": 2, "a": 1}
        escreva(d.chaves())
        escreva(d.valores())
        escreva(d.tem("a"))
        d.remova("b")
        escreva(d.tamanho())
    "#,
    );
    assert!(ok);
    assert_eq!(out, "[a, b]\n[1, 2]\nverdadeiro\n1\n");
}

#[test]
fn metodos_lista_ordem_superior() {
    let (out, err, ok) = rodar(
        r#"
        escreva([1,2,3,4].mapeie(funcao(x) { retorne x * 2 }))
        escreva([1,2,3,4,5,6].filtre(funcao(x) { retorne x % 2 == 0 }))
        escreva([1,2,3,4,5].reduza(0, funcao(a, x) { retorne a + x }))
    "#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "[2, 4, 6, 8]\n[2, 4, 6]\n15\n");
}

#[test]
fn json_ida_e_volta() {
    let (out, err, ok) = rodar(
        r#"
        var d = {"nome": "Ana", "idade": 30, "tags": ["a", "b"]}
        var t = paraJSON(d)
        escreva(t)
        var v = deJSON(t)
        escreva(v["nome"], v["idade"], v["tags"][1])
    "#,
    );
    assert!(ok, "stderr: {err}");
    // serde_json usa chaves ordenadas por padrão
    assert_eq!(
        out,
        "{\"idade\":30,\"nome\":\"Ana\",\"tags\":[\"a\",\"b\"]}\nAna 30 b\n"
    );
}

#[test]
fn formatar_data_utc() {
    let (out, err, ok) = rodar(
        r#"
        escreva(formatarData(0))
        escreva(formatarData(1700000000))
    "#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "1970-01-01 00:00:00\n2023-11-14 22:13:20\n");
}

#[test]
fn stdlib_intervalo_soma_arredonde() {
    let (out, err, ok) = rodar(
        r#"
        escreva(intervalo(0, 5))
        escreva(intervalo(1, 101).soma())
        escreva(arredondePara(3.14159, 2))
        escreva(agora() > 0)
    "#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "[0, 1, 2, 3, 4]\n5050\n3.14\nverdadeiro\n");
}

#[test]
fn ordene_por_e_obtem() {
    let (out, err, ok) = rodar(
        r#"
        var xs = [{"n": "b", "v": 3}, {"n": "a", "v": 1}, {"n": "c", "v": 2}]
        xs.ordenePor(funcao(x) { retorne x["v"] })
        escreva(xs.mapeie(funcao(x) { retorne x["n"] }).junte(""))
        var d = {"x": 10}
        escreva(d.obtem("x", 0), d.obtem("y", -1))
    "#,
    );
    assert!(ok, "stderr: {err}");
    // ordenado por v: a(1), c(2), b(3)
    assert_eq!(out, "acb\n10 -1\n");
}

#[test]
fn metodos_lista_ordene_fatie_indice() {
    let (out, err, ok) = rodar(
        r#"
        var n = [3,1,2]
        n.ordene()
        escreva(n)
        escreva([10,20,30,40].fatie(1, 3))
        escreva([10,20,30].indiceDe(30), [10,20,30].indiceDe(99))
    "#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "[1, 2, 3]\n[20, 30]\n2 -1\n");
}

#[test]
fn metodos_de_texto_novos() {
    let (out, err, ok) = rodar(
        r#"
        var s = "programação"
        escreva(s.fatie(0, 4))
        escreva(s.indiceDe("gra"), s.indiceDe("xyz"))
        escreva(s.comecaCom("prog"), s.terminaCom("ção"))
        escreva("ab".repita(3))
        escreva("café".fatie(0, 3))
    "#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "prog\n3 -1\nverdadeiro verdadeiro\nababab\ncaf\n");
}

#[test]
fn comparacao_de_texto() {
    let (out, err, ok) = rodar(
        r#"
        escreva("abacaxi" < "caju")
        escreva("z" > "a")
        escreva("igual" == "igual")
    "#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "verdadeiro\nverdadeiro\nverdadeiro\n");
}

#[test]
fn inteiro_e_decimal() {
    let (out, err, ok) = rodar(
        r#"
        escreva(2 + 2)
        escreva(10 / 2)
        escreva(10 / 3)
        escreva(5 + 2.5)
        escreva(9007199254740993)
        escreva(potencia(2, 10))
        escreva(10 % 3)
        escreva(5 == 5.0)
        escreva(tipo(5), tipo(5.0))
    "#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(
        out,
        "4\n5.0\n3.3333333333333335\n7.5\n9007199254740993\n1024\n1\nverdadeiro\nnumero numero\n"
    );
}

#[test]
fn arquivos_escreve_e_le() {
    let (out, err, ok) = rodar(
        r#"
        var caminho = "/tmp/kaju_io_teste_integracao.txt"
        escrevaArquivo(caminho, "a\nb\nc")
        escreva(existeArquivo(caminho))
        escreva(tamanho(leiaArquivo(caminho).divida("\n")))
    "#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "verdadeiro\n3\n");
    let _ = std::fs::remove_file("/tmp/kaju_io_teste_integracao.txt");
}

#[test]
fn numeros_e_matematica_extra() {
    let (out, err, ok) = rodar(
        r#"
        escreva(paraInteiro(3.9))
        escreva(paraInteiro("42"))
        escreva(minimo(5, 2, 8, 1))
        escreva(maximo(5, 2, 8, 1))
        escreva(seno(0), cosseno(0))
    "#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "3\n42\n1\n8\n0.0 1.0\n");
}

#[test]
fn matematica() {
    let (out, _, ok) = rodar(
        r#"
        escreva(raiz(16))
        escreva(piso(3.7), teto(3.2))
        escreva(potencia(2, 10))
        escreva(absoluto(0 - 5))
        var r = aleatorio()
        escreva(r >= 0 e r < 1)
    "#,
    );
    assert!(ok);
    // raiz -> decimal (4.0); piso/teto -> inteiro; potencia int -> inteiro
    assert_eq!(out, "4.0\n3 4\n1024\n5\nverdadeiro\n");
}

#[test]
fn classes_e_metodos() {
    let (out, _, ok) = rodar(
        r#"
        classe Contador {
            construtor() { isto.valor = 0 }
            metodo incremente() {
                isto.valor = isto.valor + 1
                retorne isto.valor
            }
        }
        var c = novo Contador()
        escreva(c.incremente(), c.incremente(), c.incremente())
        escreva(classeDe(c))
    "#,
    );
    assert!(ok);
    assert_eq!(out, "1 2 3\nContador\n");
}

#[test]
fn heranca_e_base() {
    let (out, _, ok) = rodar(
        r#"
        classe Animal {
            construtor(nome) { isto.nome = nome }
            metodo falar() { escreva(isto.nome + " faz um som") }
        }
        classe Gato herda Animal {
            construtor(nome) { base.construtor(nome) }
            metodo falar() {
                base.falar()
                escreva(isto.nome + " faz miau")
            }
        }
        var g = novo Gato("Felix")
        g.falar()
    "#,
    );
    assert!(ok);
    assert_eq!(out, "Felix faz um som\nFelix faz miau\n");
}

#[test]
fn objetos_sao_referencias() {
    let (out, _, ok) = rodar(
        r#"
        classe Caixa { construtor(v) { isto.v = v } }
        funcao muda(c) { c.v = 99 }
        var caixa = novo Caixa(1)
        muda(caixa)
        escreva(caixa.v)
    "#,
    );
    assert!(ok);
    assert_eq!(out.trim(), "99");
}

#[test]
fn captura_erro_de_runtime() {
    let (out, _, ok) = rodar(
        r#"
        tente {
            var x = 10 / 0
        } capture (erro) {
            escreva(erro.mensagem, erro.codigo)
        }
        escreva("continuou")
    "#,
    );
    assert!(ok);
    assert_eq!(out, "divisão por zero K020\ncontinuou\n");
}

#[test]
fn lance_captura_e_finalmente() {
    let (out, _, ok) = rodar(
        r#"
        tente {
            lance "boom"
            escreva("não roda")
        } capture (erro) {
            escreva("peguei:", erro.mensagem)
        } finalmente {
            escreva("fim")
        }
    "#,
    );
    assert!(ok);
    assert_eq!(out, "peguei: boom\nfim\n");
}

#[test]
fn lance_objeto_personalizado() {
    let (out, _, ok) = rodar(
        r#"
        classe MeuErro {
            construtor(m) { isto.mensagem = m  isto.grave = verdadeiro }
        }
        tente {
            lance novo MeuErro("falhou feio")
        } capture (erro) {
            escreva(erro.mensagem, erro.grave)
        }
    "#,
    );
    assert!(ok);
    assert_eq!(out, "falhou feio verdadeiro\n");
}

#[test]
fn lance_nao_capturado_falha() {
    let (_, err, ok) = rodar(r#"lance "erro solto""#);
    assert!(!ok);
    assert!(err.contains("erro[K230]"), "stderr: {err}");
}

#[test]
fn importe_traz_nomes_e_alias() {
    let modulo = r#"
        constante PASSO = 10
        funcao dobro(x) { retorne x * 2 }
        classe Caixa { construtor(v) { isto.v = v } }
    "#;
    let principal = r#"
        importe "mod.kaju"
        escreva(dobro(21))
        escreva(PASSO)
        var c = novo Caixa(7)
        escreva(c.v)

        importe "mod.kaju" como m
        escreva(m.dobro(5), m.PASSO)
    "#;
    let (out, err, ok) = rodar_projeto(
        &[("mod.kaju", modulo), ("principal.kaju", principal)],
        "principal.kaju",
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "42\n10\n7\n10 10\n");
}

#[test]
fn novo_com_classe_de_modulo_com_alias() {
    let modulo = r#"
        classe Ponto { construtor(x, y) { isto.x = x  isto.y = y } }
    "#;
    let principal = r#"
        importe "mod.kaju" como geo
        var p = novo geo.Ponto(3, 4)
        escreva(p.x, p.y)
    "#;
    let (out, err, ok) = rodar_projeto(
        &[("mod.kaju", modulo), ("principal.kaju", principal)],
        "principal.kaju",
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "3 4\n");
}

#[test]
fn erro_importe_arquivo_inexistente() {
    let (_, err, ok) = rodar(r#"importe "naoexiste.kaju""#);
    assert!(!ok);
    assert!(err.contains("erro[K220]"), "stderr: {err}");
}

#[test]
fn membros_estaticos() {
    let (out, err, ok) = rodar(
        r#"
        classe Contador {
            estatico total = 0
            construtor() { Contador.total += 1 }
            estatico metodo quantos() { retorne Contador.total }
        }
        novo Contador()
        novo Contador()
        escreva(Contador.total, Contador.quantos())
        classe M { estatico PI = 3 }
        escreva(M.PI)
    "#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "2 2\n3\n");
}

#[test]
fn erro_metodo_objeto_inexistente() {
    let (_, err, ok) = rodar("classe A { }\nvar a = novo A()\na.voar()");
    assert!(!ok);
    assert!(err.contains("erro[K212]"), "stderr: {err}");
}

#[test]
fn erro_novo_em_nao_classe() {
    let (_, err, ok) = rodar("var x = 5\nnovo x()");
    assert!(!ok);
    assert!(err.contains("erro[K218]"), "stderr: {err}");
}

#[test]
fn erro_metodo_inexistente() {
    let (_, err, ok) = rodar("var l = [1]\nl.gire()");
    assert!(!ok);
    assert!(err.contains("erro[K212]"), "stderr: {err}");
}

#[test]
fn erro_indice_fora_da_lista() {
    let (_, err, ok) = rodar("var l = [1, 2]\nescreva(l[5])");
    assert!(!ok);
    assert!(err.contains("erro[K206]"), "stderr: {err}");
}

#[test]
fn erro_chave_inexistente() {
    let (_, err, ok) = rodar(r#"var d = {"a": 1}
escreva(d["b"])"#);
    assert!(!ok);
    assert!(err.contains("erro[K208]"), "stderr: {err}");
}

#[test]
fn erro_variavel_indefinida_sugere() {
    let (_, err, ok) = rodar("var idade = 1\nescreva(idde)");
    assert!(!ok);
    assert!(err.contains("erro[K001]"), "stderr: {err}");
    assert!(err.contains("você quis dizer 'idade'"), "stderr: {err}");
}

#[test]
fn erro_tipos_incompativeis() {
    let (_, err, ok) = rodar(r#""abc" - 1"#);
    assert!(!ok);
    assert!(err.contains("erro[K012]"), "stderr: {err}");
}

#[test]
fn erro_divisao_por_zero() {
    let (_, err, ok) = rodar("escreva(1 / 0)");
    assert!(!ok);
    assert!(err.contains("erro[K020]"), "stderr: {err}");
}

#[test]
fn erro_constante_reatribuida() {
    let (_, err, ok) = rodar("constante PI = 3.14\nPI = 4");
    assert!(!ok);
    assert!(err.contains("erro[K009]"), "stderr: {err}");
}

#[test]
fn para_com_passo_regressivo_e_positivo() {
    let (out, err, ok) = rodar(
        r#"para i de 3 ate 1 passo -1 { escrevaSemQuebra(paraTexto(i)) }
escreva("")
para i de 0 ate 6 passo 2 { escrevaSemQuebra(paraTexto(i)) }
escreva("")"#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "321\n0246\n");
}

#[test]
fn erro_para_com_passo_zero() {
    let (_, err, ok) = rodar("para i de 1 ate 3 passo 0 { escreva(i) }");
    assert!(!ok);
    assert!(err.contains("erro[K205]"), "stderr: {err}");
}

#[test]
fn igualdade_estrutural_de_listas_e_dicionarios() {
    let (out, err, ok) = rodar(
        r#"escreva([1, 2] == [1, 2])
escreva([1, 2] == [1, 3])
escreva({"a": 1, "b": 2} == {"b": 2, "a": 1})
escreva([[1], [2]] == [[1], [2]])"#,
    );
    assert!(ok, "stderr: {err}");
    assert_eq!(out, "verdadeiro\nfalso\nverdadeiro\nverdadeiro\n");
}

#[test]
fn erro_estouro_de_inteiro() {
    let (_, err, ok) = rodar("escreva(9223372036854775807 + 1)");
    assert!(!ok);
    assert!(err.contains("erro[K222]"), "stderr: {err}");
}
