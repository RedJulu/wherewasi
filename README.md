
# 🛌 wherewasi

A simple CLI tool written in Rust to remember what you were doing while you were away.

```
     🛌 WHEREWASI (2 Notes)

┌────┬──────────────────────────────┬──────────────┬────────┬───────────┐
│ ID ┆             Text             ┆    Created   ┆ Manual ┆ Dismissed │
╞════╪══════════════════════════════╪══════════════╪════════╪═══════════╡
│  1 ┆ Finish API implementation    ┆ 2 hours ago  ┆ No     ┆ No        │
├╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│  2 ┆ Fix database migration       ┆ 1 day ago    ┆ Yes    ┆ No        │
└────┴──────────────────────────────┴──────────────┴────────┴───────────┘

🛌 » wherewasi
```

wherewasi stores notes in a `.wherewasi` file inside the current directory, so every project keeps its own notes.

## Installation

### From crates.io

```bash
cargo install wherewasi
```

### From GitHub

```bash
cargo install --git https://github.com/RedJulu/wherewasi
```

## Usage

### Add a note

```bash
wherewasi note "Finish API implementation"
```

Notes are shown automatically when entering the directory.

To create a manual note that won't be shown automatically:

```bash
wherewasi note "Maybe refactor this later" --manual
```

### List notes

```bash
wherewasi
```

Show all notes, including dismissed ones:

```bash
wherewasi --all
```

### Complete a note

```bash
wherewasi done 1
```

Remove all notes:

```bash
wherewasi done --all
```

### Dismiss a note

Dismiss a note without deleting it:

```bash
wherewasi dismiss 1
```

Dismiss all notes:

```bash
wherewasi dismiss --all
```

### Shell integration

wherewasi can automatically display sticky notes whenever you enter a directory.

#### Bash

Add this to your `~/.bashrc`:

```bash
eval "$(wherewasi init bash)"
```

#### Zsh

Add this to your `~/.zshrc`:

```bash
eval "$(wherewasi init zsh)"
```

After changing your shell configuration, restart your shell or reload the configuration.

## Commands

| Command         | Description                             |
| --------------- | --------------------------------------- |
| `note <TEXT>`   | Create a new note                       |
| `done <ID>`     | Delete a note                           |
| `done --all`    | Delete all notes                        |
| `dismiss <ID>`  | Dismiss a note                          |
| `dismiss --all` | Dismiss all notes                       |
| `init <SHELL>`  | Print the shell integration             |
| `--all`         | Show all notes including dismissed ones |

### Options

| Flag              | Description                                                   |
| ----------------- | ------------------------------------------------------------- |
| `--manual`        | Don't show the note automatically when entering the directory |
| `-a`, `--all`     | Show all notes including dismissed ones                       |
| `-h`, `--help`    | Print help                                                    |
| `-V`, `--version` | Print version                                                 |

## How it works

Each directory gets its own `.wherewasi` file:

```text
my-project/
├── src/
├── Cargo.toml
└── .wherewasi
```

This keeps notes local to the project and makes them easy to inspect, back up, or remove.

## License

[MIT](LICENSE)
