---
name: Grammar Quest
description: A synthwave-cyberpunk lab-to-maze UI where a stack-based grammar derivation is rendered as holographic gateways and glowing panels.
colors:
  signal-cyan: "#00F0FF"
  hologram-violet: "#C084FC"
  derivation-mint: "#34FFB4"
  stack-push-amber: "#FACC15"
  alert-rose: "#FF6384"
  regex-lilac: "#EEA6FF"
  void-ink: "#0B0814"
  deep-indigo: "#120C20"
  null-black: "#08050F"
  panel-plum: "#120816"
  violet-border: "#2D1C4B"
  moon-lilac: "#D7C8F5"
  dust-lilac: "#A08CC8"
  faded-lilac: "#8C78AA"
typography:
  display:
    fontFamily: "egui built-in proportional (no custom font loaded)"
    fontSize: "44-48px (RichText size, e.g. generated-sentence readout)"
    fontWeight: 700
    lineHeight: 1
    letterSpacing: "normal"
  title:
    fontFamily: "egui built-in proportional"
    fontSize: "14-18px"
    fontWeight: 700
    lineHeight: 1.1
    letterSpacing: "normal"
  body:
    fontFamily: "egui built-in proportional"
    fontSize: "11-13px"
    fontWeight: 400
    lineHeight: 1.3
    letterSpacing: "normal"
  code:
    fontFamily: "egui built-in monospace (Monospace TextStyle)"
    fontSize: "11-17px"
    fontWeight: 700
    lineHeight: 1.2
    letterSpacing: "normal"
rounded:
  sm: "4px"
  md: "6px"
  lg: "8px"
  xl: "12px"
  xxl: "16px"
spacing:
  xs: "4px"
  sm: "8px"
  md: "14px"
  lg: "16px"
  xl: "28px"
components:
  button-primary:
    backgroundColor: "{colors.signal-cyan}"
    textColor: "{colors.null-black}"
    rounded: "{rounded.lg}"
    padding: "46px height, full width"
  button-secondary:
    backgroundColor: "#201434"
    textColor: "{colors.moon-lilac}"
    rounded: "{rounded.lg}"
    padding: "38px height, full width"
  card-result:
    backgroundColor: "#120C22"
    textColor: "{colors.moon-lilac}"
    rounded: "{rounded.xxl}"
    padding: "28px"
  chip-nonterminal:
    backgroundColor: "#2D1446"
    textColor: "{colors.regex-lilac}"
    rounded: "{rounded.sm}"
    padding: "6px 2px"
  chip-terminal:
    backgroundColor: "#0F232D"
    textColor: "{colors.signal-cyan}"
    rounded: "{rounded.sm}"
    padding: "6px 2px"
---

# Design System: Grammar Quest

<!-- Extracted from incumbent code (Fase 3 polish pass): crates/grammar_quest/src/ui/theme.rs, ui/side_panel.rs, ui/editor.rs, ui/game_hud.rs, main.rs maze rendering. No previous DESIGN.md existed. -->

## Overview

**Creative North Star: "The Derivation Terminal"**

Grammar Quest presents a formal-languages proof as a piece of hostile-but-beautiful lab equipment: a dark synthwave/cyberpunk console where every visual element is doing double duty as an academic instrument. The laboratory screen is a "Grammar Studio" control panel; the maze is a holographic arena where gateways (doors) are literally the grammar's productions, rendered as glowing threshold portals with floating marquee signage. The vocabulary is explicit in the source itself — "Sleek dark cyber slate," "glassmorphism," "Holographic Summoning Sigil," "cyber grid lines," "Gateway threshold energy pad" — this is a committed identity, not incidental defaults.

Density is deliberately generous: egui's default dark visuals are fully overridden (`ui/theme.rs`) rather than left at stock, and every panel, chip, and card carries its own fill + 1–2.5px glow stroke rather than relying on borders alone. There are no photographic or illustrated assets driving the palette — color is 100% code-defined (`egui::Color32::from_rgb`, macroquad `Color::from_rgba`), which makes the token list below the actual, complete, and exhaustive palette rather than a sample.

Confirmed visual rejection (from the incumbent system, not stated preference): no light theme, no flat/corporate egui defaults, no photographic imagery — the entire identity is built from color, glow, and geometric UI chrome.

**Key Characteristics:**
- Deep near-black violet/indigo base (`#0B0814`–`#120C20`) with neon accent glows, never a mid-tone gray.
- One accent per semantic role, not per decoration: cyan = primary action/terminal symbols, violet = non-terminal/structural, mint = success/completion, amber = "push" state/score, rose = error only.
- Borders act as light sources (1–2.5px saturated strokes) on top of flat, unlit fills — this is the system's only depth cue.
- Monospace type marks anything that is literally grammar notation (productions, stack contents, regex); proportional type marks narration and labels.

## Colors

The palette is a small, disciplined neon set on a near-black base — accent colors are assigned by meaning (terminal vs. non-terminal vs. success vs. warning vs. error), never by decoration, and each is reused identically across the laboratory panel, the HUD, and the maze.

### Primary
- **Signal Cyan** (`#00F0FF`): the primary interactive/brand color. Active egui widgets, the "▶ JOGAR NO LABIRINTO 2D" CTA, terminal-symbol accents (stack chips, word readout, normal maze gateways), "GRAMMAR STUDIO"/"GRAMMAR QUEST" wordmarks.

### Secondary
- **Hologram Violet** (`#C084FC`): non-terminal / structural accent. egui selection highlight, non-terminal stack chips, the regex-box border, puzzle-door gateways, the maze's central sigil ring.

### Tertiary
- **Derivation Mint** (`#34FFB4`): success/completion accent. "✓ DERIVAÇÃO CONCLUÍDA" banner, exit gateways, the victory modal's border and headline, regex-type checkmarks in the welcome card.
- **Stack Push Amber** (`#FACC15`): transient-state accent, used sparingly. The "↑ PUSH" animation label, the "✨ PILHA VAZIA" (empty stack) callout, the victory score readout.
- **Alert Rose** (`#FF6384` border / `#FF8CA5` text): the only error color. Grammar-parse error alert box in the editor — reserved exclusively for that failure state.
- **Regex Lilac** (`#EEA6FF`): dedicated to rendered regex/production text specifically (the regex chip value, derivation-trace rule text, elimination-step results), distinct from the violet used for chrome so formal notation always reads as its own color family.

### Neutral
- **Void Ink** (`#0B0814`) / **Deep Indigo** (`#120C20`): base window/panel fills — the app never shows a lighter chrome surface than these.
- **Null Black** (`#08050F`): `extreme_bg_color`, used for text-input backgrounds and the darkest cards (e.g. victory modal's inner result card).
- **Violet Border** (`#2D1C4B`, also seen as `#4A2D6E` for window strokes): the standard 1px hairline/stroke color separating panels from the void background.
- **Moon Lilac** (`#D7C8F5`): primary body/label text on dark fills (inactive widget foreground, button labels).
- **Dust Lilac** (`#A08CC8`) / **Faded Lilac** (`#8C78AA`): secondary and tertiary text — captions, step counts, hint copy — in descending emphasis.

### Named Rules
**The One Accent Per Meaning Rule.** Cyan, violet, mint, amber, and rose are never interchangeable — each maps to exactly one semantic role (terminal/primary, non-terminal/structural, success, transient-state, error) everywhere in the app, laboratory and maze alike. Introducing a new accent color always means introducing a new meaning, not decorating an existing one.

## Typography

**Body/UI Font:** egui's built-in default proportional font (no custom font family is loaded via `egui::FontDefinitions` anywhere in the codebase).
**Code/Formal-Notation Font:** egui's built-in Monospace `TextStyle`, used wherever grammar productions, stack contents, or regex are literally displayed.
**In-World Font:** macroquad's built-in default font (`draw_text`/`measure_text` called with `None` font), used for all maze-canvas text — sigil labels, gateway signage, floating combat-text.

**Character:** No branded typeface has been chosen yet — this is an honest gap, not a hidden decision. Hierarchy currently comes entirely from size, weight (`.strong()`), and color, layered on stock fonts.

### Hierarchy
- **Display** (strong, 44–48px): the generated sentence itself — the single largest text anywhere in the UI, always in Signal Cyan, on both the laboratory result card and the victory modal.
- **Headline/Title** (strong, 14–24px): screen and section titles ("⚡ GRAMMAR STUDIO", "🏆 VITÓRIA NO LABIRINTO!", "✓ DERIVAÇÃO CONCLUÍDA").
- **Body** (regular, 12–13px): descriptive copy, hints, button labels.
- **Label** (strong, 11px, often uppercase Portuguese phrases like "PRODUÇÕES (P)", "EXEMPLOS PRONTOS"): section eyebrows in Faded/Dust Lilac.
- **Code** (strong monospace, 11–17px): productions, stack symbols, regex — always monospace so formal notation never gets mistaken for prose, and always in Regex Lilac.

### Named Rules
**The Notation-Is-Monospace Rule.** Anything that is literally part of the formal grammar (a production, a stack symbol, a regex) renders in the Monospace `TextStyle` and Regex Lilac; narration and UI chrome never do, even when quoting the same string.

## Layout

Two-screen shell reusing one shared side panel component (per `AGENTS.md`'s "don't duplicate the derivation panel between screens" rule):

- **Laboratory:** a left `egui` form panel (`show_editor`, unspecified width, natural egui flow) plus the fixed-width right side panel (`show_side_panel`, 310px, non-resizable) — a two-column desktop layout with no responsive breakpoints (native window, not a browser viewport).
- **Maze:** full-window macroquad canvas for the 2D arena, with a `TopBottomPanel` HUD bar docked to the top (glassmorphism, ~semi-opaque `rgba(10,7,20,240)`) and the same 310px side panel optionally toggled on the right (`state.show_side_panel`).
- **Modal overlay:** the victory screen is a centered, non-resizable, title-bar-less `egui::Window`, anchored via `Align2::CENTER_CENTER`.
- **Rhythm:** `ui.add_space` calls cluster around 4/8/14/16/28px, i.e. the spacing scale in the frontmatter — tight (4–8px) between a label and its value, generous (14–28px) between distinct sections/cards.
- **Maze grid:** the arena floor is scored with a 48px cyber grid (`grid_sz = 48.0`), giving the world its own spatial rhythm independent of the UI panel scale.

## Elevation & Depth

Flat-by-default with glow-as-elevation: there are no drop shadows anywhere in the codebase. Depth and hierarchy come from (1) layered fill darkness — void/indigo base, slightly lighter panel fill, slightly lighter-still card fill — and (2) saturated 1–2.5px border strokes that read as light emission rather than a physical edge, reinforced in the maze by literal alpha-blended radial glow (`draw_circle` with low-alpha fills stacked at decreasing radius, e.g. the arena's central sigil at 240px/120px, then 140px/90px alpha).

### Named Rules
**The Border-Is-Light Rule.** A stroke is never a neutral gray outline — it is always a saturated accent color at full or near-full alpha, functioning as the surface's only light source. Where the system needs ambient glow instead of a hard edge (the maze sigil, ambient dust particles), it stacks 2–3 low-alpha fills of decreasing radius rather than using a blur/shadow primitive.

## Shapes

Corner radius scales with a surface's size and importance: 4px for the smallest inline chips (stack symbols), 6px for compact frames (the regex value box, HUD word readout), 8px for buttons and derivation-step cards, 12px for the egui window default and the victory-modal inner card, 16px for the two largest result cards (laboratory welcome/result panel, victory modal itself). In the maze world, geometry is otherwise rectilinear (walls, gateway thresholds, floating sign cards) punctuated by circular motifs reserved for energy/sigil elements (pylon end-caps, the central summoning ring, particle sparks) — circles always read as "active/energy," rectangles always read as "structure."

## Components

### Buttons
- **Shape:** 8px corner radius (`{rounded.lg}`) on every button, no exceptions observed.
- **Primary:** Signal Cyan fill, Null Black bold text, full-width, 46px tall — reserved for the single most important action per screen ("▶ JOGAR NO LABIRINTO 2D" in the lab, "🔄 Jogar Novamente" on the victory modal).
- **Secondary:** dark violet fill (`#201434`/`#231C3C`), Moon Lilac or white text, full-width or fixed 170px, 38–42px tall — used for the paired/lesser action next to a primary button ("⚡ Gerar no Laboratório", "⚙ Voltar ao Laboratório").
- **HUD toggle buttons:** unfilled-radius egui default buttons whose fill color itself communicates state (e.g. the "Ver/Ocultar Trilha" toggle switches between `#1A112C` inactive and `#32195B` active fill, rather than adding a checkmark or border change).

### Cards / Containers
- **Corner Style:** 16px for hero result cards, 8–12px for nested/secondary cards, 6px for inline value chips.
- **Background:** one step lighter than the surrounding panel (`#120C22` on `#0B0814`/`#120C20`), never a flat white or gray.
- **Shadow Strategy:** none — see Elevation & Depth; depth comes from the border-stroke and fill-layering instead.
- **Border:** always present, always a saturated accent color at 1–2px, chosen per the card's semantic role (mint for the "derivation complete" card, violet for the plain welcome/idle card, cyan for the HUD's word-so-far readout).
- **Internal Padding:** 28–32px for hero cards, 8–24px for nested cards, 4–10px for inline chips.

### Chips (stack symbol tokens)
- **Non-terminal chip:** dark violet fill (`#2D1446`), violet (`#C084FC`) 1px border, Regex Lilac monospace text, 4px radius.
- **Terminal chip:** dark teal fill (`#0F232D`), Signal Cyan 1px border, cyan-tinted monospace text, 4px radius.
- **State:** during a push/pop animation, chip text alpha is driven by `ctx.animate_value_with_time`/a tween progress value rather than swapping color — the fade-in itself is the "just pushed" signal.

### Inputs / Fields
- **Style:** the grammar-production textarea uses egui's default dark text-edit chrome against the theme's `extreme_bg_color` (Null Black), monospace font, no custom border beyond egui's stock focus ring.
- **Error:** a dedicated rose-bordered (`#FF6384`) card appears below the input on parse failure — the input field itself does not change color, keeping the error signal in one place.

### Navigation
No traditional nav bar; the "HUD bar" (`show_hud`) functions as navigation/status combined — brand mark, live word/stack readout, and the "⚙ Laboratório [Esc]" back-button live in one glassmorphism-style top strip, always present during maze play.

### Gateway Signage (signature component)
The maze's doors are not plain sprites: each gets an animated forcefield threshold (scanline sweep in the door's theme color), pylon end-caps with glowing circular terminals, a soft downward light-beam triangle, and a floating holographic marquee sign above it whose border/text color matches the gateway's semantic role (mint = exit, violet = puzzle, cyan = normal production). This is the single most distinctive, non-generic component in the system and the clearest expression of "a production is a real, inhabitable gateway."

## Do's and Don'ts

### Do:
- **Do** keep the five-color semantic mapping fixed: cyan = terminal/primary action, violet = non-terminal/structural, mint = success, amber = transient/score, rose = error-only.
- **Do** render any literal grammar notation (productions, stack, regex) in the Monospace `TextStyle` and Regex Lilac, never in the proportional/narration font.
- **Do** build depth from layered fills + saturated 1–2.5px glow strokes; when more ambient depth is needed, stack 2–3 low-alpha fills at decreasing radius (as the maze sigil does) rather than reaching for a blur/shadow.
- **Do** scale corner radius with surface importance: 4px chips → 6px inline value boxes → 8px buttons/step-cards → 12–16px hero cards and modals.

### Don't:
- **Don't** introduce a drop shadow, gradient text, or a light/corporate egui-default theme anywhere — the system is committed to flat, near-black, glow-lit dark mode.
- **Don't** reuse Hologram Violet for a new "success" state or Derivation Mint for a new "structural" one — accent colors are semantic, not decorative, per the One Accent Per Meaning rule.
- **Don't** load a custom font as a drive-by fix for one screen; typography currently runs entirely on egui/macroquad defaults, and that's a tracked gap (see Typography's Character note), not an invitation to patch it inconsistently per component.
