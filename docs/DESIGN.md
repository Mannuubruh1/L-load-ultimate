1. Mod Concept & Core Features


A. Auto-Generated & Self-Updating Config (sd:/ultimate/l_load.json)

• First-Boot Generation: When Smash Ultimate finishes loading its character database (ui_chara_db.prc), the plugin scans every vanilla and modded fighter (including CSK Collection characters) and writes a clean JSON template where every character defaults to "" (unassigned).
• Non-Destructive Updates: On future boots, if you install or remove character mods, the plugin reads your existing l_load.json, preserves all of your custom links, and appends any newly detected characters with "".
• Flexible Linking Modes:
	• One-Way Link: "ui_chara_goku": "ui_chara_vegeta" (pressing L on Goku switches to Vegeta).
	• Two-Way Toggle: Pairing both entries switches back and forth between Goku and Vegeta.
	• Multi-Character Cycle: Chaining A -> B -> C -> A lets you cycle through three or more variants from a single CSS slot.
	• Optional Costume-Slot Support: Supporting "ui_chara_id" as well as "ui_chara_id:c120" lets you link both standalone CSK fighters and movesets hidden on extra costume slots (c08–c255).

B. Dynamic CSS Hover & Selection Badge

• Context-Aware Prompt: When a player's cursor hovers over (or selects) a character with an active link in l_load.json, a small overlay badge appears on that player's bottom CSS panel (chara_4 / chara_3 window).
• Visuals: The badge displays Smash's built-in L button glyph next to the target character's stock icon (chara_2), giving instant visual feedback on who L will load.
• Auto-Hide & Dynamic Flip: Moving the cursor onto an unlinked character hides the badge immediately; pressing L to swap characters flips the badge to show the previous character's stock icon (if two-way linked).

C. Instant L-Press Character Swap on the CSS

• No Holding Through Loading Screens: Pressing L while hovering over or holding a linked character swaps the player's active fighter (ui_chara_id and color_id) directly on the CSS.
• Full UI & Audio Refresh: Swapping updates the player panel's portrait (chara_4), name plate, stock icon, and plays the new character's announcer voice clip.
• Smart L Input Handling: Because L normally cycles costume colors backward once a token is placed, the plugin intercepts L on linked characters to trigger the character swap (while leaving R for costume cycling, or only overriding L while hovering/before token placement).

2. Everything You Need to Build It


Development Environment & Toolchain

• Rust Toolchain (rustup): Installed with the Nintendo Switch target (aarch64-nintendo-switch-freestanding).
• cargo-skyline: Command-line tool used to build, package, and deploy .nro plugins for Skyline/Ozone.
• Target Runtime — Ozone + ARCropolis:
	• Ozone (exefs runtime replacement required by Smashline 2.0+).
	• ARCropolis (handles modded file loading and ui_chara_db.prc merging).
	• CSK Collection (optional, if your custom characters use added ui_chara_db slots).
• Testing Setup: A modded Nintendo Switch running Atmosphère (with FTP/USB transfer for quick .nro iteration) or a Switch emulator configured with Ozone, ARCropolis, and Smash Ultimate 13.0.5.

Key Rust Crates & Libraries

• skyline (skyline-rs): Provides memory hooking (#[skyline::hook]), symbol lookup, and Switch OS filesystem access (std::fs mapped to sd:/).
• nnsdk: Rust bindings for Nintendo's internal SDK types—specifically nn::ui2d (for manipulating CSS layout panes, textures, and visibility) and nn::hid (for controller button checks).
• smash (smash-rs): Bindings for Super Smash Bros. Ultimate's internal classes, IDs, and UI/database structs.
• serde & serde_json: Serializes and deserializes sd:/ultimate/l_load.json cleanly, preserving order (pair with indexmap if you want the JSON to stay sorted by CSS roster order).
• prcx / param-xml (or ui_chara_db memory structs): Reads the loaded ui_chara_db entries in memory after ARCropolis finishes mounting mods.

3. Technical Architecture (How the .nro Fits Together)


Module	Responsibility	How It Hooks Into Smash Ultimate
config.rs	Loads, merges, and saves sd:/ultimate/l_load.json	Hooks right after ui_chara_db finishes initializing on boot; reads all active ui_chara_id entries, merges with existing JSON on sd:/, and stores the lookup map in memory (RwLock<IndexMap<String, String>>).
css_input.rs	Detects L button presses per player (P1–P8)	Hooks the CSS frame/input update loop; checks L just_pressed for each active player port, looks up their hovered/selected character in the config map, and suppresses default L costume-cycling when a swap triggers.
css_swap.rs	Changes the active character on the CSS	Calls Smash's internal CSS character-selection setter with the target ui_chara_id and color_id, triggering the built-in portrait (chara_4), name text, and announcer voice refresh.
css_badge.rs	Renders the L glyph + target stock icon (chara_2)	Hooks the CSS player-panel (nn::ui2d::Layout) update; locates or clones an icon pane on each player's panel, binds the target character's chara_2 BNTX texture, and toggles visibility based on whether the hovered fighter has a valid link.


4. Recommended Development Roadmap

1. Phase 1 — Boot & Config Generator: Set up a cargo-skyline project, hook ui_chara_db initialization, and verify that booting the game generates sd:/ultimate/l_load.json with all vanilla and modded fighters.
2. Phase 2 — L-Press Character Swap: Hook the CSS input loop so pressing L on a linked character swaps the player's selected fighter and refreshes the portrait and announcer call.
3. Phase 3 — Hover Badge UI (nn::ui2d): Hook the CSS panel layout to display the L button icon and load the target character's chara_2 stock texture when hovering over a linked slot.
4. Phase 4 — Polish & Edge Cases: Add support for costume-slot syntax (ui_chara_mario:c120), multi-player ports (P1–P8), CPU tokens, and invalid/missing target ID safeguards in l_load.json.
If you're ready to start coding, let me know:
• Whether you want me to generate the starter Cargo.toml and lib.rs template for Phase 1 (auto-generating l_load.json on boot)
• If you'll be testing on real Switch hardware or an emulator so I can include the right build/deploy commands
