# Boing Network Design System — Compliance Checklist

This document cross-references the official PDFs (Design System, Visual Design System, Cursor Agent Prompt, Visual Notes, AI Prompt) with the current website implementation. All styling is driven by `website/src/styles/boing-theme.css` and design tokens in `:root`.

---

## 1. Color Palette

| Token / Use | PDF Spec | Implemented | Notes |
|-------------|----------|-------------|--------|
| **Backgrounds** | | | |
| Primary (Deep Space Navy) | #0A0E1A | ✅ `--bg-primary` | |
| Secondary | #121829 (Official) / #0D1B2A (visual notes) | ✅ `--bg-secondary` #121829 | Official Visual Design System is source of truth |
| Tertiary | #1A2235 | ✅ `--bg-tertiary` | |
| Card | rgba(18, 24, 41, 0.7) | ✅ `--bg-card` | |
| **Text** | | | |
| Primary | #F0FFFE | ✅ `--text-primary` | |
| Secondary | #B8E6E3 | ✅ `--text-secondary` | |
| Tertiary | #7EB8B5 | ✅ `--text-tertiary` | |
| **Accents** | | | |
| Boing Teal | #00E5CC | ✅ `--accent-teal` | Icons, borders, circuit patterns |
| Electric Cyan | #00B4FF (Official) / #00D4FF (some docs) | ✅ `--accent-cyan` #00B4FF | Official doc Section 10 |
| Quality Gold | #FACC15 (Official) / #FFD700 (notes) | ✅ `--accent-gold` #FACC15 | |
| Authenticity Purple | #9B59B6 | ✅ `--accent-purple` | |
| Mascot Yellow | #FFE000 | ✅ `--mascot-yellow` | |
| **Borders** | rgba(0, 229, 204, 0.2) | ✅ `--border-color` | Teal per Official |
| **Glows & Shadows** | As in Section 10 | ✅ `--glow-cyan`, `--glow-blue`, `--glow-gold`, `--glow-purple`, `--shadow` | |

---

## 2. Typography

| Element | PDF Spec | Implemented |
|---------|----------|--------------|
| Body / UI | Comfortaa, Regular/Bold | ✅ `--font-sans` |
| H1 (page title) | Orbitron, Bold, 3.5–5rem, cyan text-shadow | ✅ `--font-display`, 0 0 20px var(--glow-cyan) |
| H2 (section title) | Orbitron, Bold, 2–2.5rem, subtle cyan glow | ✅ 0 0 12px var(--glow-cyan) |
| H3 (card title) | Comfortaa, 700, 1.25rem | ✅ h3–h6 use `--font-sans`, 700, 1.25rem base |
| Display (high-impact) | Cinzel or similar serif | ✅ `--font-display-serif` (Cinzel), `.display-title` |
| Code | JetBrains Mono | ✅ `--font-mono` |
| Font import | Comfortaa, Orbitron, JetBrains Mono, Cinzel | ✅ All in boing-theme.css |

---

## 3. UI Components

| Component | PDF Spec | Implemented |
|-----------|----------|-------------|
| Primary button | Gradient teal→cyan, dark text, glow, hover lift | ✅ `.btn-primary`, gradient, 0 0 16px/28px glow |
| Secondary button | Transparent, cyan border/text, hover glow | ✅ `.btn-secondary` |
| Cards / panels | Glassmorphism, blur(12px), rgba(18,24,41,0.7), border, hover glow | ✅ `.card`, `--bg-card`, backdrop-filter |
| Navigation | Dark semi-transparent, blur(16px), border, sticky | ✅ `nav`, `.navbar`, `header` |
| Nav links | text-secondary → accent-teal on hover | ✅ |
| Inputs | bg-tertiary, border, focus teal + glow | ✅ Global input/textarea/select |

---

## 4. Backgrounds & Environment

| Context | PDF Spec | Implemented |
|---------|----------|-------------|
| Landing / marketing | Full Aquatic-Space image, fixed, overlay gradient | ✅ `.page-landing`, overlay 0.3→0.6 (Official) |
| App / dashboard | Hex grid over --bg-primary, 60×60 | ✅ `.page-app`, `/assets/hex-grid.svg` |
| Hex grid stroke | rgba(0, 229, 204, 0.06) | ✅ hex-grid.svg updated to 0.06 |

---

## 5. Motion & Animation

| Animation | PDF Spec | Implemented |
|-----------|----------|-------------|
| Float (mascot) | 9s ease-in-out infinite | ✅ `--motion-float-duration`, `boing-float` |
| Glow pulse | 5s, 8px→20px+40px | ✅ `boing-glow-pulse`, keyframes aligned |
| Fade-in-up | 0.6s ease-out | ✅ `boing-fade-in-up`, `.animate-fade-in-up` |
| Shoot star | Keyframe defined | ✅ `boing-shoot-star` |
| Hover (buttons/cards) | translateY(-2px), transition 0.3s | ✅ Buttons and cards |

---

## 6. Mascot (Boing Bot)

| Guideline | Implemented |
|-----------|-------------|
| Floating animation | ✅ BoingMascot.astro uses motion-config float/glow |
| Circuit lines Boing Teal | ✅ Glow uses var(--glow-cyan) |
| Not distorted/recolored | ✅ Asset used as provided |

---

## 7. Brand Pillars & Messaging (Reference)

From the PDFs, the six pillars and taglines are documented for copy and future sections; no code changes required for wording only.

- **Security** — "Safety and correctness first — always over speed."
- **Scalability** — High throughput without compromising other pillars.
- **Decentralization** — "Anyone can participate. No one can shut the door."
- **Authenticity** — "One chain, one identity — authentic and independent."
- **Transparency** — "100% open — in design, governance, and operations. Trust through verification."
- **Quality Assurance** — "Legitimate purpose only — enforced by the network."

---

## 8. Assets You May Need to Provide

These are called out in the design system or current implementation; add them when available.

| Asset | Location | Purpose |
|-------|----------|---------|
| **boing-aquatic-space-bg.webp** | `website/public/assets/` | Full Aquatic-Space background for landing pages (coral, jellyfish, starfield). Used when body has class `page-landing`. |
| **Boing Bot mascot** | Already in use as `boing_robot_only.png`, `boing_environment.png` in `public/` | Ensure these match the design system (teal body, yellow eyes, circuit lines). Replace if you have updated official assets. |
| **Hex grid** | `website/public/assets/hex-grid.svg` | ✅ Created; single hex outline stroke rgba(0,229,204,0.06). No asset needed. |
| **Optional: Stone tablet / crystal / “BOING!” comic assets** | N/A | PDFs mention stone tablets for pillars, crystal shapes, and “BOING!” comic-style logo. Only needed if you add those specific sections or components. |
| **Favicon / logo** | `website/public/favicon.svg`, `logo.svg` | Already present; replace with official versions if design system provides them. |

---

## 9. Optional Enhancements (Not Required by PDFs)

- **Responsive breakpoints**: Design system mentions testing at 375px, 768px, 1440px; existing layout is responsive; add breakpoint audits if needed.
- **Reduced motion**: `prefers-reduced-motion` is respected for keyframes and utilities.
- **WCAG**: Primary text on primary background meets AAA; other contrasts follow tokens.

---

*Last updated after PDF review. Implementation follows Boing_Network_Official_Visual_Design_System.pdf Section 10 and Cursor_AI_Agent_Prompt as primary references.*
