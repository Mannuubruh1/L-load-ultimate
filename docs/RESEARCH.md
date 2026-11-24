# Research notes: plugin setup and CSS hook points

Written 2026-11-23 from reading the source of the repos linked at the bottom
(shallow clones, current as of that date). "Verified" means I read it in the
repo. "Inference" means I'm drawing a conclusion from what I read and it still
needs testing.

## Toolchain (verified)

- **Plugin code is still skyline-rs.** Both Smashline and ARCropolis are Rust
  `cdylib` crates using the `skyline` crate, with `[package.metadata.skyline]`
  holding Smash's title ID (`01006A800016E000`) and `#[skyline::hook(...)]` for
  hooks. Smashline builds with `cargo skyline build --release` (see its
  `build.sh`).
- **Ozone is the runtime.** Its README calls it a drop-in Skyline replacement
  and says Skyline plugins work with it. Smashline 2.x requires it.
- `skyline` is pulled two ways: ARCropolis uses `skyline = "0.6"` from
  crates.io, Smashline uses the `ultimate-research/skyline-rs` git repo. Pick
  one deliberately; the git version is probably newer.
- Both set `panic = "abort"` in their profiles; copy that.

## Smashline is not what this mod needs (inference)

Its wiki describes it as a fighter/weapon script (ACMD and status script)
replacement framework. It does not mention UI or character select. L-Load does
not replace fighter scripts, so I'd drop Smashline from the dependency list in
`DESIGN.md`. It also says Smashline can't add characters or costume slots;
that's ARCropolis's job.

## Bindings (verified)

The `smash-rs` fork Smashline uses has no character select or `ui_chara`
bindings (I searched its source). Expect to write raw hooks and function
declarations yourself.

## ARCropolis already hooks the CSS (verified)

`src/fixes.rs` and `src/lib.rs` in ARCropolis are the best working example. It:

- **Hooks `css_set_selected_character_ui(param_1, chara_hash_1, chara_hash_2,
  color, unk1, unk2)`** (a normal hook, with `call_original!`).
- **Hooks `chara_select_scene_destructor`**, which is how it knows when the CSS
  has closed.
- **Has inline hooks on `change_color_l` and `change_color_r`**, the costume
  cycling handlers. In those, `ctx.registers[8]` is the color. It uses them to
  stop online play from picking costume slots 8 or higher.
- **Reaches the character database** through a "parameters cache" pointer
  (`ParametersCache.databases.character`).
- **Finds functions by byte pattern, not fixed offsets** (`get_offset_neon` in
  `src/offsets.rs`), which survives game updates better. A few things in it are
  still hardcoded offsets, so check each one.

## What this means for L-Load

1. **Characters are `u64` hash40 values in the game, not strings.** The CSS
   functions above take `chara_hash` arguments. So `l_load.json` keys like
   `ui_chara_goku` must be converted to hash40 before swapping. Going the other
   way is the harder problem: to *generate* the JSON from the game's database
   you need hash-to-name labels. How to get readable names is unresolved.
2. **L interception (inference).** The `change_color_l` inline hook is the
   natural place to catch L on the CSS. It's unconfirmed whether it fires while
   only hovering, before a token is placed.
3. **The swap (inference).** `css_set_selected_character_ui` looks like a
   candidate for refreshing what the panel shows, but ARCropolis uses it to
   load portrait images, so its exact role in committing a fighter choice is
   unclear. Needs disassembly or experiments.
4. **Reusing ARCropolis's code or byte patterns.** ARCropolis is licensed
   GPLv3 (verified from its `LICENSE` file). Copying its code into this repo
   can carry GPL obligations for your mod, so decide on a license before
   copying anything, and credit it. Reading it to learn the approach and
   writing your own hooks is the safer route. I haven't tested whether its
   patterns match your game version.
5. **`ui_chara_db` timing.** The design doc's Phase 1 says to hook after
   `ui_chara_db` loads. ARCropolis does the merging of mod entries, so L-Load
   must run after it. The right hook point isn't determined.

## Still open

- Which game version and Ozone/ARCropolis versions you're targeting.
- Whether you test on a Switch or an emulator. Codespaces can compile and unit
  test only.
- Hash-to-name source for generating `l_load.json`.
- Whether `change_color_l` is the right L hook (needs testing).

## Sources

- [HDR-Development/smashline](https://github.com/HDR-Development/smashline) and its wiki
- [WuBoytH/ozone](https://github.com/WuBoytH/ozone)
- [Raytwo/ARCropolis](https://github.com/Raytwo/ARCropolis)
- [WuBoytH/smash-rs](https://github.com/WuBoytH/smash-rs)
