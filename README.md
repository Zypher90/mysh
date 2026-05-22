# mysh

A Unix shell implementation written in Rust, built from scratch. This project covers the full pipeline from raw input parsing to process execution, pipelines, I/O redirection, built-in commands, and job control.

---

## Project status

| Phase | Feature | Status |
|---|---|---|
| 1 | REPL loop | 🔨 In progress |
| 2 | Lexer & parser | 🔨 In progress |
| 3 | Command execution | 🔨 In progress |
| 4 | Pipelines & I/O redirection | 📋 Planned |
| 5 | Built-in commands | 📋 Planned |
| 6 | Job control & signals | 📋 Planned |
| 7 | History, completion, scripting | 📋 Planned |

---

## Features

### Implemented
- Interactive REPL with a custom prompt
- Graceful exit on `Ctrl-D` (EOF)
- Lexer with full quoting support: `'single'`, `"double"`, and `\backslash` escapes
- Operator recognition: `|`, `>`, `>>`, `<`, `&`
- Parser producing a structured `Pipeline` of `Command`s with redirect metadata

### Planned
- External command execution via `PATH` resolution
- Pipelines connecting multiple commands
- I/O redirection (`>`, `>>`, `<`)
- Built-in commands: `cd`, `pwd`, `exit`, `export`, `echo`
- Background job execution (`cmd &`)
- Job control: `fg`, `bg`, signal handling (`Ctrl-C`, `Ctrl-Z`)
- Command history and tab completion
- Shell variables and `$VAR` expansion
- Multiple OS support

---

## Getting started

### Prerequisites

- Rust toolchain (`rustc`, `cargo`) — install via [rustup.rs](https://rustup.rs)
- A Unix-like system (Linux or macOS)

### Build and run

```bash
git clone https://github.com/yourname/mysh
cd mysh
cargo build
cargo run
```

You should see the shell prompt:

```
mysh> _
```

### Run tests

```bash
cargo test
```

---

## Project structure

```
mysh/
├── Cargo.toml
└── src/
    ├── main.rs        # REPL loop — reads input, drives the pipeline
    ├── lexer.rs       # Tokenizer: raw &str → Vec<Token>
    ├── parser.rs      # Parser: Vec<Token> → Pipeline of Commands
    ├── executor.rs    # Command execution (Phase 3+)
    └── builtins.rs    # Built-in command implementations (Phase 5+)
```

> Note: the module structure above reflects the planned layout. Early phases may keep everything in `main.rs` until there is enough code to split out.

---

## Architecture

### Phase 1 — REPL loop

The entry point is a `loop` in `main.rs` that:

1. Prints the prompt and flushes stdout explicitly (using `io::Write::flush`)
2. Calls `stdin().read_line()` into a cleared buffer each iteration
3. Returns `Ok(0)` bytes → `Ctrl-D` → clean exit
4. Passes the trimmed, non-empty input to the lexer

### Phase 2 — Lexer & parser

**Lexer** (`lexer.rs`)

The lexer walks input as a `Peekable<Chars>` iterator and produces a `Vec<Token>`:

```rust
pub enum Token {
    Word(String),
    Pipe,
    RedirectOut,
    RedirectIn,
    Append,
    Background,
}
```

Quote handling follows POSIX rules:
- `'single quotes'` — everything literal, no escape mechanism
- `"double quotes"` — `\"` and `\\` are the only escape sequences recognised
- `\x` outside quotes — next character is always literal

**Parser** (`parser.rs`)

The parser walks the token slice with a manual index (to support consuming two tokens at once for redirects) and produces:

```rust
pub struct Command {
    pub argv: Vec<String>,
    pub stdin_file: Option<String>,
    pub stdout_file: Option<String>,
    pub append: bool,
}

pub struct Pipeline {
    pub commands: Vec<Command>,
    pub background: bool,
}
```

### Phase 3 — Execution (planned)

Each `Command` in the `Pipeline` will be executed using `std::process::Command`. `PATH` is resolved manually by splitting `$PATH` and checking each directory for the executable. The executor will wait on the child process and forward its exit code.

### Phase 4 — Pipelines & redirection (planned)

Pipelines connect `stdout` of one process to `stdin` of the next using OS pipes, wired through `std::process::Stdio`. File redirection replaces `stdin`/`stdout`/`stderr` before exec using the `Command::stdin`, `Command::stdout`, and `Command::stderr` builder methods.

### Phase 5 — Built-ins (planned)

Commands like `cd` must run inside the shell process itself — a child process changing its working directory has no effect on the parent. Built-ins are matched before any attempt to exec externally.

Planned built-ins: `cd`, `pwd`, `exit`, `export`, `unset`, `echo`.

### Phase 6 — Job control (planned)

Background jobs (`cmd &`), foreground/background switching (`fg`/`bg`), and signal handling (`SIGCHLD`, `SIGINT`, `SIGTSTP`) via the `nix` crate for safe bindings to `setpgid`, `waitpid`, and `kill`.

---

## Key design decisions

**`Peekable<Chars>` for the lexer** — operators like `>>` require one character of lookahead without consuming. `Peekable` provides `.peek()` for this without needing an `unget` mechanism.

**Index-based parsing** — redirect handling consumes two tokens at once (the operator and the filename). A standard `for` loop doesn't support this cleanly; a `while i < tokens.len()` loop with manual `i += 2` does.

**`Word(String)` owns its content** — quoted strings like `"hello world"` are assembled character by character and cannot borrow from the original input slice. Owned `String`s inside `Token::Word` avoid lifetime complexity throughout the pipeline.

**`background` on `Pipeline`, not `Command`** — `cmd1 | cmd2 &` runs the entire pipeline in the background, not just the final stage.

---

## References

- [POSIX Shell Command Language](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html) — the authoritative spec for quoting, operators, and grammar
- [Writing a Shell in C — Stephen Brennan](https://brennan.io/2015/01/16/write-a-shell-in-c/) — language-agnostic concepts that transfer directly to Rust
- [The Linux Programming Interface, ch. 44–45](https://man7.org/tlpi/) — pipes and FIFOs
- [GNU libc — Job Control](https://www.gnu.org/software/libc/manual/html_node/Job-Control.html) — process groups, sessions, signals
- [`nix` crate](https://docs.rs/nix/latest/nix/) — safe Rust bindings to Unix syscalls
- [`std::process::Command`](https://doc.rust-lang.org/std/process/struct.Command.html) — Rust's process spawning API
- [`std::iter::Peekable`](https://doc.rust-lang.org/std/iter/struct.Peekable.html) — lookahead iterator used in the lexer
- [CodeCrafters — Build your own shell](https://app.codecrafters.io/courses/shell/overview) — test-driven milestones for shell implementation

---

## License

MIT