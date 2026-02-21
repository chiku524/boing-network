# Design Takeover Plan — Step 1: Audit Findings

**Date:** 2025-02-21  
**Scope:** Boing Network official website (Astro) — `website/`

---

## 1. Global stylesheet

| Location | Purpose |
|----------|---------|
| **`website/src/styles/boing-theme.css`** | Primary global theme: `:root` variables, fonts import, animations, card/link 3D motion, gradient utilities. Imported by `Layout.astro`. |
| **`website/src/styles/motion-config.css`** | Motion tuning variables (mascot, cards, links). Imported by `boing-theme.css`. |
| **`website/src/layouts/Layout.astro`** | `<style is:global>` for `html`, `body`, `main`, `a`, `code`, `.site-header`, `.site-footer`, `.btn-testnet`. |

**Conclusion:** The single injection point for design tokens and base styles is **`boing-theme.css`**. Layout.astro’s global block will be aligned with the new base body styles and tokens.

---

## 2. Component library

| Component type | Location | Notes |
|----------------|----------|--------|
| **Buttons** | `website/src/pages/index.astro` (scoped): `.btn`, `.btn-primary`, `.btn-secondary`, `.btn-outline`. Layout.astro: `.btn-testnet`, `.nav-cta`. | No shared Button.astro; classes used on `<a>` and `<button>`. |
| **Cards** | index.astro: `.stat-card`, `.chart-card`, `.app-card`, `.card`. testnet.astro: `.dedicated-card`. | Glass-style cards with backdrop-filter and border. |
| **Navigation** | Layout.astro: `<header class="site-header">`, `<nav>` with links and `.nav-cta`. docs/index.astro: sidebar `<nav>`. network pages: `.nav-links`. | Single main site header; no separate Nav.astro. |
| **Inputs / forms** | faucet.astro: `.faucet-form input`, `.faucet-form button`. | No shared Input or Form component. |

**Conclusion:** Styling is class-based in global/scoped CSS. Refactor will apply design-system button, card, nav, and input styles via these classes and global element rules.

---

## 3. Hardcoded colors (findings)

Search pattern: hex (`#...`), `rgb(...)`, `hsl(...)` in `.css`, `.scss`, `.tsx`, `.jsx`, `.html`, `.astro`.

| File | Occurrences |
|------|-------------|
| **website/src/pages/index.astro** | SVG strokes/fills: `#00E5CC`, `#00B4FF`, `#22c55e`. Inline `style="background:#00E5CC"` on legend dots. Hero orbs/gradients, hero-title, tagline, float-particle, shooting-star lines (teal/cyan). Timeline `.t-item.complete` / `.app-status.live`: `#34d399`. |
| **website/src/components/EnhancedAnimatedBackground.astro** | SVG circles: `#00E5CC`, `#00B4FF`, `#fff`. Gradients: `stop-color="#00E5CC"`, `#00B4FF`. CSS: `color: #00E5CC` on `.hex-grid-layer`; `var(--bg-primary, #0A0E1A)`; shooting-star `rgba(0,229,204,...)`, `rgba(0,180,255,...)`. |
| **website/src/styles/boing-theme.css** | Comment refs to hex; `:root` vars use hex/rgba (to be replaced by design token set). Gradient utilities: `#00E5CC`, `#00B4FF`. |
| **website/src/layouts/Layout.astro** | `<meta name="theme-color" content="#0A0E1A" />` (keep for meta tag; value matches `--bg-primary`). Header/footer: `rgba(0, 229, 204, ...)`, `rgba(250, 204, 21, ...)` — to use `var(--border-color)`, `var(--glow-*)`, etc. |
| **website/src/pages/network/faucet.astro** | JS: `'#ef4444'`, `'#94a3b8'` for message color. CSS fallbacks: `var(--accent, #22c55e)`, `var(--bg-primary, #0A0E1A)`. Form input border/background rgba. |
| **website/src/pages/network/testnet.astro** | `.card-arrow`, `.cta-link`: `var(--accent, #22c55e)`. |
| **website/src/pages/network/single-vs-multi.astro** | `.nav-links a`: `var(--accent, #22c55e)`. |
| **website/src/pages/network/bootnodes.astro** | `.nav-links a`: `var(--accent, #22c55e)`. |
| **website/src/components/BoingMascot.astro** | Scoped style: `rgba(0, 229, 204, 0.15)` / `0.25` for filter (use `var(--glow-cyan)` where applicable). |

**Conclusion:** All listed hardcoded colors will be replaced by design token variables (or removed in favor of global rules) in Steps 2–5 and Step 8.

---

## 4. Font imports

| Location | Current |
|----------|---------|
| **website/src/styles/boing-theme.css** (line 4) | `@import url('https://fonts.googleapis.com/css2?family=Comfortaa:wght@300;400;500;600;700&family=JetBrains+Mono:wght@400;500;600&display=swap');` |
| **website/public/favicon.svg** | `font-family="Comfortaa,system-ui,sans-serif"` (asset; no change). |
| **website/public/logo.svg** | `font-family="Comfortaa,system-ui,sans-serif"` (asset; no change). |

**Conclusion:** Comfortaa and JetBrains Mono are already imported. **Orbitron** (design system display font) will be added in Step 3 and assigned to `--font-display` for headings.

---

## 5. Assets for background styles (Step 6)

| Asset | Status |
|------|--------|
| **Landing:** `/assets/boing-aquatic-space-bg.webp` | Not present. CSS will reference this path; add image when available. |
| **App:** `/assets/hex-grid.svg` | Not present. Will be created as a single hexagon outline with `stroke: rgba(0,229,204,0.2)`. |

---

## Step 8: Final Audit & Quality Check (post-implementation)

### Hardcoded colors

- **Intentional (single source of truth):** All hex/rgba in `website/src/styles/boing-theme.css` `:root` are the design token definitions. No replacement.
- **Meta tag:** `Layout.astro` `<meta name="theme-color" content="#0A0E1A" />` must remain hex (meta tags do not resolve CSS variables). Value matches `--bg-primary`.
- **SVG in `hex-grid.svg`:** Single hex outline uses `stroke="rgba(0,229,204,0.2)"` to match `--border-color`; file is static so no variable resolution.
- **All other style and component files:** Hardcoded colors replaced with `var(--…)` references (index.astro, faucet, testnet, bootnodes, single-vs-multi, Layout, EnhancedAnimatedBackground, BoingMascot).

### Fonts

- **Comfortaa** (body): Loaded via Google Fonts in `boing-theme.css`; `--font-sans` applied to `body`.
- **Orbitron** (headings): Added to Google Fonts import; `--font-display` applied to `h1–h6` in global styles.
- **JetBrains Mono** (code): Loaded; `--font-mono` applied to `code`, `pre`, `.monospace`.

### Interactive states

- Buttons: `.btn-primary`, `.btn-secondary`, `.btn-outline` and `button[data-variant="primary|secondary"]` use design tokens; hover uses `transform`, `box-shadow` with `--glow-cyan` / `--glow-blue`.
- Links: Global `a` and `a:hover` use `--accent-cyan`, `--glow-blue`, `var(--motion-transition)`.
- Cards: `.card`, `.stat-card`, `.chart-card`, `.app-card` use `--bg-card`, `--border-color`, `--border-hover`, `--glow-cyan` on hover.
- Inputs: Global `input`, `textarea`, `select` focus use `--accent-teal`, `--glow-cyan`.
- Nav: `.site-header`, `.nav-cta` use `--border-color`, `--accent-teal`, `--accent-gold`, `--glow-gold`.

### Backgrounds

- **Landing:** Homepage uses `pageClass="page-landing"`. Body gets `.page-landing` with `url('/assets/boing-aquatic-space-bg.webp')` and overlay. Add `boing-aquatic-space-bg.webp` to `website/public/assets/` when the asset is ready.
- **App:** Faucet and Testnet use `pageClass="page-app"`. Body gets `.page-app` with `url('/assets/hex-grid.svg')` and `--bg-primary`. `hex-grid.svg` created in `website/public/assets/`.
- When `pageClass` is set, `EnhancedAnimatedBackground` is not rendered so the body background is visible.

### Contrast & responsiveness

- Primary text `#F0FFFE` on `#0A0E1A` exceeds WCAG AAA. Secondary/tertiary and accent usage kept consistent with tokens.
- No structural or breakpoint changes; existing responsive rules and media queries retained.

### Constraints

- No new colors introduced outside design tokens.
- HTML structure of components unchanged; styling-only refactor.
- Accessibility: contrast and focus states preserved; reduced-motion respected in keyframes and utilities.

---

*Design takeover Steps 1–8 complete.*
