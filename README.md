# L-Load

A Smash Ultimate mod idea: press **L** on the character select screen to swap a
character for a linked one, with the links defined in a config file
(`sd:/ultimate/l_load.json`). See [docs/DESIGN.md](docs/DESIGN.md) for the full design.

## Status

Early scaffold. Only the platform-independent config logic exists so far.

| Piece | State |
|---|---|
| `core/` config merge, parse, link lookup | Written, unit tested |
| Plugin crate (hooks, CSS swap, badge) | Not started |

## Working in Codespaces

Open the repo in a Codespace. The dev container installs Rust plus a nightly
toolchain with `rust-src`. Then:

```bash
cargo test        # runs the core logic tests
```

## Open questions (resolve before writing the plugin crate)

These came out of reading the Ozone and Smashline repos and the design doc,
and have **not** been verified:

- **Toolchain.** Ozone's README says it is a drop-in Skyline replacement and that
  Skyline plugins work with it, and it builds itself with devkitPro and
  `cargo skyline`. Confirm the exact setup for building a *plugin*, and whether
  `cargo-skyline` installs cleanly in Codespaces.
- **Smashline.** Its wiki is the likely source for plugin and API docs. The design
  doc's claim that Ozone is required by Smashline 2.0+ is unconfirmed.
- **Crate names.** The doc mentions `prcx`; the usual `.prc` crate may be `prc-rs`.
  Check against existing plugins.
- **Hook points.** Finding the CSS input and character-select functions is the
  hardest part. Study existing CSS-related plugins' source for offsets and patterns.
- **Testing.** Codespaces can compile and unit test, but running the plugin needs
  a Switch or emulator with the game, which cannot be done in Codespaces.

## Suggested order

1. Config logic and tests (done, in `core/`).
2. Plugin crate that logs the roster and writes `l_load.json` on boot.
3. A hardcoded L-press swap (for example Mario to Link) before any config wiring.
4. Config-driven swap.
5. Hover badge last, since it is the most fragile.
