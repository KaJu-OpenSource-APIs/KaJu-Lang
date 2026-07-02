# Ícone e tipo MIME da kaju no Linux

Registra o tipo MIME `text/x-kaju` e associa o ícone do caju aos arquivos `.kaju`
no gerenciador de arquivos (padrão freedesktop: Nautilus, Thunar, Nemo, Dolphin,
PCManFM etc.).

## Instalar (só para o seu usuário, sem sudo)

```bash
./instalar.sh
```

Isso copia o tipo MIME para `~/.local/share/mime` e o ícone para
`~/.local/share/icons`, e atualiza os caches. Pode ser preciso reiniciar o
gerenciador de arquivos (ou fazer logout/login) para o ícone aparecer.

## Verificar

Use o `gio`, que é o backend dos gerenciadores de arquivos:

```bash
gio info -a standard::content-type algum_arquivo.kaju   # -> text/x-kaju
```

> Obs.: `xdg-mime query filetype` pode responder `text/plain` — ele usa o
> `file`/libmagic, que ignora o banco freedesktop. Isso não é um problema:
> os gerenciadores de arquivos usam o GLib/`gio`, que respeita o glob `*.kaju`.

## Desinstalar

```bash
./desinstalar.sh
```

## Arquivos

- `kaju-mime.xml` — definição do tipo MIME (glob `*.kaju`, subclasse de `text/plain`)
- `text-x-kaju.png` — ícone (o caju), nomeado conforme o tipo MIME
- `instalar.sh` / `desinstalar.sh`
