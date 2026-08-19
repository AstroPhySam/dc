# DC — Dev Container CLI (Rust) — Plan v1

A CLI that manages Dev Environment Dockerfile Collections. Templates are stored as
Dockerfiles (optionally with a human-readable `details.txt`) under a global
directory, split into `local/` (user-managed) and `remote/` (downloaded from the
project's GitHub repo). All template interaction is prompt-driven.

## Layout

```
~/.dc/
├── config.toml          # remote repo (hardcoded = this project's GitHub repo) + last_seen_sha
└── templates/
    ├── local/           # your templates — dc local * operates here
    │   ├── bash/Dockerfile
    │   ├── python/basic/Dockerfile
    │   └── rust/Dockerfile
    └── remote/          # populated by dc remote get — dc remote launch/delete operate here
```

## Command reference

| Command            | Source                         | Behavior                                                                                                                                    |
| ------------------ | ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------- |
| `dc init`          | —                              | Create `~/.dc/` + `templates/local/` + `templates/remote/` if missing. Print help banner: Name, Description, full templates path, commands. |
| `dc local list`    | `templates/local/`             | Walk dir, print template paths                                                                                                              |
| `dc local info`    | `templates/local/`             | Single-select → print `details.txt` (fallback: base image / labels from Dockerfile)                                                         |
| `dc local launch`  | `templates/local/`             | Single-select → build image if missing → `docker run -it` with cwd mounted                                                                  |
| `dc local delete`  | `templates/local/`             | Single-select → Yes/No confirm → remove                                                                                                     |
| `dc remote list`   | upstream repo                  | Tree API → print available templates; badges for new/updated vs `last_seen_sha`                                                             |
| `dc remote info`   | upstream repo                  | Single-select from upstream → fetch only that `details.txt` → print                                                                         |
| `dc remote get`    | upstream → `templates/remote/` | Multi-select → fetch each selected template's files (from tree) → save                                                                      |
| `dc remote launch` | `templates/remote/`            | Single-select (already-fetched only) → build if missing → `docker run -it`                                                                  |
| `dc remote delete` | `templates/remote/`            | Single-select → Yes/No confirm → remove                                                                                                     |

## Network cost per command

- `list` / `info` / `get` / `launch`: 0–2 GitHub API calls + raw fetches for
  **only** the files needed
- `launch` (local or remote): **zero** network
- `get`: re-fetch skips unchanged files via compare API (`last_seen_sha`)

## Rules

- **Identity** = `<source>/<relpath>`; **slug** = identity with `/`→`-`; image
  `dc/<slug>`, container `dc-<slug>`
- **Launch**: `docker run -it --rm -v <cwd>:/workspace -w /workspace dc/<slug>`
- **details.txt**: optional, shown verbatim by `info`
- **Pure HTTP** via `reqwest` — no git binary, no clone, no auth (public repo)
- **Prompts** via `dialoguer`: Select / MultiSelect / Confirm

## Rust structure

```
Cargo.toml
src/
├── main.rs        # dispatch
├── cli.rs         # clap subcommands
├── init.rs        # first-run setup + banner
├── templates.rs   # local discovery, info, delete
├── remote.rs      # tree API, compare, get, info
├── launch.rs      # docker build/run (shared by local & remote)
└── config.rs      # paths + config.toml + last_seen_sha
```

Deps: `clap`, `dialoguer`, `serde`, `serde_json`, `toml`, `dirs`, `reqwest`,
`anyhow`. Docker shelled out via `Command`.

## Distribution

GitHub Actions + cargo-dist → Windows/macOS/Linux binaries on release. Bundled
starter templates (bash/python/rust) in the repo, seeded to `templates/local/`
only on install.

## Milestones

1. Skeleton + `dc init` + banner + config paths
2. `dc local list / info / launch / delete`
3. `dc remote list / info / get / launch / delete`
4. Bundled templates, cargo-dist workflow, docs
