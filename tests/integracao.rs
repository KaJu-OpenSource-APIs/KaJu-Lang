//! Testes de integração: rodam programas .kaju de verdade e conferem a saída.
//! Esta é a garantia de que os exemplos da documentação nunca ficam desatualizados.

use std::process::Command;

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
fn erro_importe_arquivo_inexistente() {
    let (_, err, ok) = rodar(r#"importe "naoexiste.kaju""#);
    assert!(!ok);
    assert!(err.contains("erro[K220]"), "stderr: {err}");
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
