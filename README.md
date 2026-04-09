# portkill

Find and kill the process using a given TCP or UDP port. Cross-platform CLI for when a port is in use (for example, a stuck dev server on 3000) and you need to free it.

## What it does

- Resolves which process or processes are bound to the given port
- Prints PID and process name
- Optionally prompts for confirmation, then terminates them
- Supports dry-run (no kill, no prompt), skip prompt (`--yes`), and force kill (`--force`)

## Install

[Rust](https://rustup.rs) stable is required.

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

Port must be between 1 and 65535.

### Examples

```bash
portkill 3000
```

Finds processes on port 3000 and prompts to confirm before killing, unless `--yes` or `--force` is used.

```bash
portkill 3000 --dry-run
```

Shows what would be killed. Does not prompt and does not kill.

```bash
portkill 3000 --yes
```

Kills processes on port 3000 without prompting. Skips confirmation only and uses graceful termination.

```bash
portkill 3000 --force
```

Force kills processes (`SIGKILL` on Unix, `taskkill /F` on Windows) and skips confirmation.

### Flags

| Flag | Short | Description |
| --- | --- | --- |
| `--dry-run` | none | Show what would be killed; never prompt or kill |
| `--yes` | `-y` | Skip confirmation only; graceful terminate (does not force) |
| `--force` | `-f` | Forceful termination; skips confirmation |

## Exit codes

- `0` - Success: processes killed, dry-run completed, or user declined and exited cleanly
- Non-zero - Failure: no process on port, required command missing, or kill failed (including partial failure)

## Platform behavior

- Windows - Uses `netstat -ano`, `tasklist`, and `taskkill`. No extra install. Some processes do not respond to normal `taskkill` and only terminate with forceful termination; use `--force` in that case.
- macOS / Linux - Uses `lsof` (port to PID), `ps` (process names), and `kill`. On some systems `lsof` is not installed by default; install it if needed, for example `apt install lsof` on Debian or Ubuntu. If `lsof` is missing, portkill prints an error with a hint.

## Limitations

- Depends on platform CLI tools; behavior and paths depend on the system
- TCP and UDP listeners on the given port are both considered when the underlying tools report them
- Process name may be unknown; PID is always shown and used for killing
- Non-force termination may fail for processes that only respond to forceful termination, for example some Python servers on Windows

## Smoke tests

Verified on Windows: dry-run reports PIDs without killing, force kill terminates the process and frees the port, and an unused port exits non-zero with a clear message. Graceful kill (`--yes`) can fail when the target process requires forceful termination.

## Contributing

- Run `cargo fmt`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` before submitting changes
- Ensure `cargo build --release` succeeds. Platform-specific behavior lives under `src/platform/`
