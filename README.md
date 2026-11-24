# L-Load

A Smash Ultimate mod idea: press **L** on the character select screen to swap a
character for a linked one, with the links defined in a config file
(`sd:/ultimate/l_load.json`). See [docs/DESIGN.md](docs/DESIGN.md) for the design
and [docs/RESEARCH.md](docs/RESEARCH.md) for what the reference projects showed.

## Status

| Piece | State |
|---|---|
| `core/` config merge, parse, link lookup | Written, unit tested |
| `plugin/` skeleton (loads, logs one line) | Written, **not yet built** (see below) |
| Roster, config file I/O, CSS hooks, swap, badge | Not started |

The plugin skeleton's manifest resolves (`skyline 0.6.0`) and the
`#[skyline::main(name = "...")]` form it uses exists in the crate source, but it
has never been compiled for the Switch target. The environment it was written in
could not download the Rust toolchain that `cargo-skyline` needs.

## Working in Codespaces

The dev container installs `cargo-skyline`. Then:

```bash
# Core logic (host build, no game needed)
cargo test

# Plugin (Switch build)
cd plugin
cargo skyline update-std        # one time: installs the Switch toolchain; slow
cargo skyline build --release
```

If `cargo skyline build` asks whether to install the toolchain, answer `Y`.
The output should be an `.nro` under `plugin/target/aarch64-skyline-switch/release/`
(likely `libl_load.nro`). If the build fails, paste the first error into the
chat, because this step has not been tested yet.

Running the plugin needs a Switch or emulator with the game, Ozone and ARCropolis.
Codespaces can build it but cannot run it.

## Layout

- `core/`: platform-independent logic. Builds with plain `cargo`.
- `plugin/`: the Skyline plugin. Builds with `cargo skyline`. It is excluded from
  the root workspace on purpose, since it targets the Switch.
- `docs/`: design and research notes.

## Suggested order

1. Config logic and tests (done, in `core/`).
2. Plugin skeleton that builds and logs on boot (written, needs a first build).
3. Plugin writes `l_load.json` on boot.
4. A hardcoded L-press swap (for example Mario to Link) before any config wiring.
5. Config-driven swap.
6. Hover badge last, since it is the most fragile.
