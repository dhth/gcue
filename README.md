<p align="center">
  <h1 align="center">grafq</h1>
  <p align="center">
    <a href="https://github.com/dhth/grafq/actions/workflows/main.yml"><img alt="GitHub release" src="https://img.shields.io/github/actions/workflow/status/dhth/grafq/main.yml?style=flat-square"></a>
  </p>
</p>

`grafq` (short for "graph query") lets you query Neo4j/AWS Neptune databases via
an interactive console.

It is a Rust port of [graphc].

> [!NOTE]
> grafq is alpha software. Its interface and behaviour might change in the near
> future.

💾 Installation
---

```sh
cargo install --git https://github.com/dhth/grafq
```

⚡️ Usage
---

`grafq` operates in two modes: "console" and "query".

```text
Usage: grafq [OPTIONS] <COMMAND>

Commands:
  console  Open grafq's console
  query    Execute a one-off query
  help     Print this message or the help of the given subcommand(s)

Options:
      --debug  Output debug information without doing anything
  -h, --help   Print help
```

Console Mode
---

"console mode" lets you execute queries repeatedly via an interactive console.
You can either print the results in the console, or have them piped through a
pager. Additionally, you can also write the results to your local filesystem.

```bash
grafq console -h
```

```text
Open grafq's console

Usage: grafq console [OPTIONS]

Options:
  -p, --page-results             Display results via a pager ("less", by default, can be overridden by $GRAFQ_PAGER)
  -w, --write-results            Write results to filesystem
  -d, --results-dir <DIRECTORY>  Directory to write results in [default: .grafq]
      --debug                    Output debug information without doing anything
  -f, --results-format <FORMAT>  Format to write results in [default: json] [possible values: csv, json]
  -h, --help                     Print help
```

### Console Commands

| Command                        | Description                     |
|--------------------------------|---------------------------------|
| `clear`                        | clear screen                    |
| `format`                       | csv/json                        |
| `help` / `:h`                  | show help                       |
| `output`                       | <PATH> / reset                  |
| `page`                         | on/off                          |
| `quit` / `exit` / `bye` / `:q` | quit                            |
| `write`                        | on/off                          |
| `@<path>`                      | execute query from a local file |

### Console Keymaps

| Keybinding | Description                              |
|------------|------------------------------------------|
| `↑` / `k`  | scroll up in query history               |
| `↓` / `j`  | scroll down in query history             |
| `tab`      | cycle through path suggestions           |
| `ctrl+c`   | clear input / exit                       |
| `ctrl+d`   | exit immediately                         |
| `ctrl+u`   | clear input from start of line to cursor |

Query Mode
---

"query mode" is for running one-off queries or for benchmarking them.

```bash
grafq query -h
```

```text
Execute a one-off query

Usage: grafq query [OPTIONS] <QUERY>

Arguments:
  <QUERY>  Cypher query to execute

Options:
  -p, --page-results                    Display results via a pager ("less", by default, can be overridden by $GRAFQ_PAGER)
  -b, --bench                           Whether to benchmark the query
      --debug                           Output debug information without doing anything
  -n, --bench-num-runs <NUMBER>         Number of benchmark runs [default: 5]
  -W, --bench-num-warmup-runs <NUMBER>  Number of benchmark warmup runs [default: 3]
  -P, --print-query                     Print query
  -w, --write-results                   Write results to filesystem
  -d, --results-dir <DIRECTORY>         Directory to write results in [default: .grafq]
  -f, --results-format <FORMAT>         Format to write results in [default: json] [possible values: csv, json]
  -h, --help                            Print help
```

[graphc]: https://github.com/dhth/graphc
