# L-Load: Design

Revised 2026-11-23. Supersedes the first draft. Changes are based on
[RESEARCH.md](RESEARCH.md); anything marked **(unverified)** has not been tested
against the game and may change.

## 1. Concept

On the character select screen (CSS), pressing **L** swaps the character a
player is holding or hovering for a linked character, defined in a config file.
Think of it as the L-load shortcut from Smash Remix, but user-configurable.

### 1.1 Config file: `sd:/ultimate/l_load.json`

A flat JSON object. Each key is a character ID, each value is the character that
L switches to, or `""` for no link.

```json
{
  "ui_chara_goku": "ui_chara_vegeta",
  "ui_chara_vegeta": "ui_chara_goku",
  "ui_chara_mario": "",
  "ui_chara_link": ""
}
```

**Rules**

- **One target per character.** That single rule covers every linking style:
  - *One-way:* A points to B, B is `""`.
  - *Two-way toggle:* A points to B and B points to A.
  - *Cycle:* A to B, B to C, C to A.
- **Unassigned (`""`) and invalid targets are ignored**, so a typo can never
  swap to nothing. Costume-slot targets like `ui_chara_x:c120` are validated on
  the base ID. (Implemented in `core/`.)
- **Non-destructive merging.** On boot, newly detected characters are appended
  as `""`, existing links are never changed, and characters whose mods were
  removed keep their entries so reinstalling doesn't lose your links.
  (Implemented in `core/`.)
- **Costume-slot syntax** (`ui_chara_id:c120`) is optional and comes last, since
  mapping a costume number to the game's color index is not yet worked out
  **(unverified)**.

### 1.2 Behavior

- Pressing L while hovering a linked character, or holding one, swaps that
  player's selection to the target and refreshes the panel (portrait, name,
  stock icon, announcer).
- L normally cycles costumes backward. L-Load only takes L for characters that
  have a valid link; unlinked characters keep normal behavior. R is untouched.
- **Offline only.** The swap is disabled online to avoid desyncs and surprises.
- Optional later: a small on-screen badge showing the L glyph and the target
  character's stock icon.

## 2. Platform and toolchain

| Piece | Choice | Status |
|---|---|---|
| Language | Rust, `cdylib` crate | Verified (same as Smashline and ARCropolis) |
| Plugin framework | `skyline-rs` (`skyline` crate) | Verified; pick crates.io `0.6` or the `ultimate-research/skyline-rs` git version |
| Build | `cargo skyline build --release` | Verified (Smashline's `build.sh`) |
| Runtime loader | Ozone | Verified (README: drop-in Skyline replacement) |
| File and `ui_chara_db` merging | ARCropolis (must load before L-Load's boot step) | Verified role |
| Cargo profiles | `panic = "abort"`, release `lto = true` | Verified (both reference projects) |
| Smashline | **Not used.** It replaces fighter/weapon scripts, nothing UI-related | Inference from its wiki |

Development happens in GitHub Codespaces. Codespaces can compile and run the
unit tests for `core/`; it **cannot run the plugin**. Real testing needs a
Switch or an emulator with the game, Ozone, and ARCropolis.

## 3. Architecture

Two crates in one workspace.

**`core/`** (exists, tested). Platform-independent logic with no game
dependencies, so it builds and tests anywhere.

- `parse_config`, `merge_roster`, `link_target`.

**`plugin/`** (not started). The Skyline plugin. Modules:

| Module | Job |
|---|---|
| `roster.rs` | Read the character list from the game's database after ARCropolis has merged mods. |
| `hash.rs` | Convert `ui_chara_*` names to hash40 and back. See 4.1. |
| `config_io.rs` | Read and write `l_load.json` on the SD card, using `core/`. |
| `css_input.rs` | Catch L on the CSS, check the hovered or selected character against the config, suppress normal costume cycling when a swap happens. |
| `css_swap.rs` | Change the player's selected character and refresh the panel. |
| `css_badge.rs` | Optional on-screen hint. Last, most fragile. |

### 3.1 Hook points (all unverified for our use)

These are what ARCropolis (GPLv3) already hooks on the CSS. They're leads, not
confirmed solutions.

- **`css_set_selected_character_ui(param_1, chara_hash_1, chara_hash_2, color,
  unk1, unk2)`**: sets what the CSS panel shows for a character. A candidate
  for refreshing the panel. Its exact role in committing a choice is unclear.
- **`change_color_l` / `change_color_r`** (inline hooks): the costume-cycling
  handlers. `change_color_l` is the natural place to catch L. Unknown whether
  it fires while merely hovering, before a token is placed.
- **`chara_select_scene_destructor`**: tells you when the CSS closes. Useful for
  clearing state.
- **`is_online`**: ARCropolis already has a way to check this; use it to
  disable swapping online.
- Finding functions by **byte-pattern scan** (as ARCropolis does) is more
  robust across game updates than hardcoded offsets.

## 4. Known problems

### 4.1 Characters are hashes, not names

The game identifies characters by `u64` hash40 values. Going from a name in the
config to a hash is easy. Going from the game's database to readable names, to
*generate* the config, is not: the database holds hashes. Options:

1. Bundle a hash-to-name label list for vanilla characters and let modded
   characters show as their hex hash until the user names them.
2. Read labels from an existing label file if one ships with the loader.
3. Generate the config with hex keys only and let users rename.

Needs a decision before Phase 1 is finished.

### 4.2 The swap function is unidentified

Nobody here has confirmed which function actually commits a character change.
Expect to spend real time disassembling or experimenting. This is the riskiest
part of the project, which is why the roadmap proves it with a hardcoded swap
before anything else is built on top.

### 4.3 License

ARCropolis is GPLv3. Copying its code or patterns can put GPL terms on this
mod. Decide on a license for L-Load *before* copying anything, and prefer
writing your own hooks after reading theirs.

## 5. Roadmap

Each phase has a check for "done". Don't start the next phase until it passes.

1. **Core logic.** Done: config parse, merge, link lookup, with tests in `core/`.
2. **Plugin skeleton.** `plugin/` crate builds with `cargo skyline`, loads under
   Ozone, and logs one line on boot. *Done when:* the log line appears on a
   test setup.
3. **Roster + config generation.** Read the character list after mods merge, run
   it through `merge_roster`, write `l_load.json`. Requires the 4.1 decision.
   *Done when:* a fresh boot creates the file, and a second boot after installing
   a character appends it without touching existing links.
4. **Hardcoded swap proof.** Pressing L on a hardcoded character (for example
   Mario) switches to another (for example Link). No config involved.
   *Done when:* it works for one player and the panel refreshes correctly.
5. **Config-driven swap.** Replace the hardcoded pair with `link_target`, and
   support all four player slots first. *Done when:* one-way, two-way, and cycle
   links all behave, and unlinked characters keep normal L behavior.
6. **Safeguards.** Offline-only gating, invalid or missing target handling,
   CPU tokens, more than four ports if the game exposes them.
7. **Costume-slot syntax** (`ui_chara_id:c120`). Only after the swap is stable.
8. **Badge (optional).** The L glyph and target stock icon. Requires UI layout
   manipulation and is the most likely to break across versions.

## 6. Open questions

- Switch or emulator for testing, and which game, Ozone, and ARCropolis versions.
- Hash-to-name source for generated configs (4.1).
- Whether `change_color_l` is the right L hook (3.1).
- Which function commits a swap (4.2).
- License for the project (4.3).
