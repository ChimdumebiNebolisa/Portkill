# portkill

Find and kill the process using a given TCP or UDP port. Cross-platform CLI for when a port is in use (e.g. a stuck dev server on 3000) and you need to free it.

## What it does

- Resolves which process(es) are bound to the given port
- Prints PID and process name
- Optionally prompts for confirmation, then terminates them
- Supports dry-run (no kill, no prompt), skip prompt (`--yes`), and force kill (`--force`)

## Install

[Rust](https://rustup.rs) (stable) required.

```bash
cargo install --path .
```

Or build the binary:

```bash
cargo build --release
```

Binary: `target/release/portkill` (Windows: `target/release/portkill.exe`).

## Usage

```text
portkill <PORT> [OPTIONS]
```

Port must be 1–65535.

### Examples

```bash
portkill 3000
```

Finds process(es) on port 3000 and prompts to confirm before killing (unless `--yes` or `--force` is used).

```bash
portkill 3000 --dry-run
```

Shows what would be killed. Does not prompt and does not kill.

```bash
portkill 3000 --yes
```

Kills process(es) on port 3000 without prompting. Skips confirmation only; uses graceful termination (no force).

```bash
portkill 3000 --force
```

Force kill (SIGKILL on Unix, `taskkill /F` on Windows) and skip confirmation.

### Flags

| Flag       | Short | Description                                                       |
| ---------- | ----- | ----------------------------------------------------------------- |
| `--dry-run`| —     | Show what would be killed; never prompt or kill                   |
| `--yes`    | `-y`  | Skip confirmation only; graceful terminate (does not force)       |
| `--force`  | `-f`  | Forceful termination; skips confirmation                          |

## Exit codes

- **0** — Success: processes killed, or dry-run completed, or user declined and exited cleanly.
- **Non-zero** — Failure: no process on port, required command missing, or kill failed (including partial failure).

## Platform behavior

- **Windows** — Uses `netstat -ano`, `tasklist`, and `taskkill`. No extra install. Some processes do not respond to normal `taskkill` and only terminate with forceful termination; use `--force` in that case.
- **macOS / Linux** — Uses `lsof` (port → PID), `ps` (process names), and `kill`. On some systems `lsof` is not installed by default; install it if needed (e.g. `apt install lsof` on Debian/Ubuntu). If `lsof` is missing, portkill prints an error with a hint.

## Limitations

- Depends on platform CLI tools; behavior and paths depend on the system.
- TCP and UDP listeners on the given port are both considered when the underlying tools report them.
- Process name may be unknown; PID is always shown and used for killing.
- Non-force termination may fail for processes that only respond to forceful termination (e.g. some Python servers on Windows).

## Smoke tests

Verified on Windows: dry-run reports PIDs without killing; force kill terminates the process and frees the port; unused port exits non-zero with a clear message. Graceful kill (`--yes`) can fail when the target process requires forceful termination.
