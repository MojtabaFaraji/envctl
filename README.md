# envctl

Manage named shell environments (variables, PATH, aliases, functions, cwd)
stored as TOML profiles under XDG directories. Apply them to the current
shell with `eval "$(envctl activate NAME)"`.

## 60-second quickstart

```sh
envctl init
envctl add work --var FOO=bar --path-prepend /opt/work/bin
eval "$(envctl activate work)"
echo $FOO
envctl deactivate
```

## Install

From source: `make build && make install PREFIX=/usr` (or use .deb/.rpm).

## Commands

See `man envctl` or `envctl --help`.

## Shell compatibility

Generated activate code is tested in bash, zsh, and fish.

## Migration

Import existing `.env` files: `envctl import .env --name myenv`.

## License

MIT OR Apache-2.0
