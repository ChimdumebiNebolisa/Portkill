# portkill

A cross-platform developer CLI that finds and kills the process currently using a given TCP or UDP port.

## Why

When a port is in use (e.g. 3000 by a stuck dev server), you need to free it. `portkill` finds the PID(s) on that port and terminates them, with optional confirmation and dry-run.

## Install

Requires [Rust](https://rustup.rs) (stable). Then:

```bash
cargo install --path .
```

Or build and run:

```bash
cargo build --release
# Binary: target/release/portkill (Windows: portkill.exe)
```

## Usage

```text
portkill <PORT> [OPTIONS]
```

### Examples

```bash
portkill 3000
```
Find and kill process(es) on port 3000. Prompts for confirmation unless `--yes` or `--force` is used.

```bash
portkill 3000 --dry-run
```
Show which process(es) would be killed without killing or prompting.

```bash
portkill 3000 --yes
```
Kill without prompting (graceful terminate).

```bash
portkill 3000 --force
```
Force kill (SIGKILL / taskkill /F) and skip confirmation.

### Flags

| Flag | Short | Description |
|------|--------|-------------|
| `--dry-run` | | Show what would be killed; never prompt or kill |
| `--yes` | `-y` | Skip confirmation prompt |
| `--force` | `-f` | Forceful termination; skips confirmation |

## Exit behavior

- **0** – Success (processes killed, or dry-run showed targets, or user declined and exited cleanly).
- **Non-zero** – Error: no process on port, command not found, kill failed, or partial kill failure.

## Platform notes

- **Windows** – Uses `netstat -ano`, `tasklist`, and `taskkill`. No extra install.
- **macOS / Linux** – Uses `lsof` and `kill`. `lsof` is usually pre-installed on macOS; on some Linux distros you may need `apt install lsof` (or equivalent).

If `lsof` is missing on Unix, the tool prints a clear error with install hints.

## Limitations

- Relies on platform CLI tools (`netstat`, `tasklist`, `taskkill` on Windows; `lsof`, `ps`, `kill` on Unix). Path and behavior depend on the system.
- Both TCP and UDP listeners on the given port are included where the underlying tools report them.
- Process name lookup can fail; the PID is always shown and used for killing.
