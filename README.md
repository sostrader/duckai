# duckai

DuckDuckGo AI to OpenAI

- Bearer authentication
- Support IP proxy pool
- Very small memory footprint

## Models

Model mapper, unsupported models default to `gpt-4o-mini`

- gpt-4o-mini -> `(gpt-4o-mini)`
- claude-3-haiku -> `(claude-3-haiku-20240307)`
- llama-3.1-70b -> `(meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo)`
- mixtral-8x7b -> `(mistralai/Mixtral-8x7B-Instruct-v0.1)`

## Conversation

```bash
curl --request POST 'http://127.0.0.1:8080/v1/chat/completions' \
  --header 'Content-Type: application/json' \
  --data '{
    "messages": [
      {
        "role": "user",
        "content": "你好！"
      }
    ],
    "model": "gpt-4o-mini",
    "stream": true
  }'
```

## Manual

```bash
$ duckai -h
DuckDuckGo AI to OpenAI

Usage: duckai
       duckai <COMMAND>

Commands:
  run      Run server
  start    Start server daemon
  restart  Restart server daemon
  stop     Stop server daemon
  log      Show the server daemon log
  ps       Show the server daemon process
  gt       Generate config template file (toml format file)
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

$ duckai run -h
Run server

Usage: duckai run [CONFIG_PATH]

Arguments:
  [CONFIG_PATH]  Configuration filepath [default: duckai.yaml]

Options:
  -h, --help  Print help
```

## Installation

<details>

<summary>If you need more detailed installation and usage information, please check here</summary>

1. Install

- cargo

```bash
cargo install vproxy
```

- Dokcer

```bash
docker run --rm -it ghcr.io/penumbra-x/duckai:latest run
```

</details>

## Contributing

If you would like to submit your contribution, please open a [Pull Request](https://github.com/penumbra-x/duckai/pulls).

## Getting help

Your question might already be answered on the [issues](https://github.com/penumbra-x/duckai/issues)

## Sponsor

If you find this project helpful, please consider sponsoring me to support ongoing development:

**USDT-TRC20**: TCwD8HfHnJ7236Hdj3HF5uZKR2keeWeqZe
