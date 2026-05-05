---
name: Modern Analytical Journal (Dark)
colors:
  surface: "#0f1417"
  surface-dim: "#0f1417"
  surface-bright: "#353a3d"
  surface-container-lowest: "#0a0f12"
  surface-container-low: "#171c1f"
  surface-container: "#1b2023"
  surface-container-high: "#262b2e"
  surface-container-highest: "#313539"
  on-surface: "#dfe3e7"
  on-surface-variant: "#d1c5b1"
  inverse-surface: "#dfe3e7"
  inverse-on-surface: "#2c3134"
  outline: "#9a8f7e"
  outline-variant: "#4e4637"
  surface-tint: "#ecc161"
  primary: "#ffd475"
  on-primary: "#3f2e00"
  primary-container: "#e2b859"
  on-primary-container: "#624800"
  inverse-primary: "#785a00"
  secondary: "#c1c6d8"
  on-secondary: "#2a303e"
  secondary-container: "#434958"
  on-secondary-container: "#b3b8ca"
  tertiary: "#d1daf0"
  on-tertiary: "#283141"
  tertiary-container: "#b5bed3"
  on-tertiary-container: "#444d5f"
  error: "#ffb4ab"
  on-error: "#690005"
  error-container: "#93000a"
  on-error-container: "#ffdad6"
  primary-fixed: "#ffdf9d"
  primary-fixed-dim: "#ecc161"
  on-primary-fixed: "#251a00"
  on-primary-fixed-variant: "#5b4300"
  secondary-fixed: "#dde2f5"
  secondary-fixed-dim: "#c1c6d8"
  on-secondary-fixed: "#151b29"
  on-secondary-fixed-variant: "#414755"
  tertiary-fixed: "#dae2f9"
  tertiary-fixed-dim: "#bec7dc"
  on-tertiary-fixed: "#131c2b"
  on-tertiary-fixed-variant: "#3e4759"
  background: "#0f1417"
  on-background: "#dfe3e7"
  surface-variant: "#313539"
typography:
  display-xl:
    fontFamily: Playfair Display
    fontSize: 64px
    fontWeight: "700"
    lineHeight: "1.1"
    letterSpacing: -0.02em
  headline-lg:
    fontFamily: Playfair Display
    fontSize: 40px
    fontWeight: "600"
    lineHeight: "1.2"
    letterSpacing: -0.01em
  headline-md:
    fontFamily: Playfair Display
    fontSize: 28px
    fontWeight: "600"
    lineHeight: "1.3"
  body-lg:
    fontFamily: Satoshi
    fontSize: 18px
    fontWeight: "400"
    lineHeight: "1.6"
  body-md:
    fontFamily: Satoshi
    fontSize: 16px
    fontWeight: "400"
    lineHeight: "1.5"
  data-mono:
    fontFamily: JetBrains Mono
    fontSize: 13px
    fontWeight: "500"
    lineHeight: "1.4"
    letterSpacing: 0.02em
  label-caps:
    fontFamily: Satoshi
    fontSize: 12px
    fontWeight: "700"
    lineHeight: "1"
    letterSpacing: 0.1em
spacing:
  unit: 4px
  gutter: 24px
  margin: 48px
  section-gap: 64px
  inline-xs: 4px
  inline-sm: 8px
  inline-md: 16px
---

## Brand & Style

This design system is built for the high-intellect observer, prioritizing information density and structural clarity over decorative trends. The personality is authoritative, archival, and precise. By merging the layout rigor of 20th-century broadsheets with a contemporary digital interface, it evokes a sense of "digital permanence."

The style is defined as **Architectural Editorial**. It rejects organic forms and soft depth in favor of a strict 0px radius and a reliance on 1px solid rules to define hierarchy. This creates a highly legible, distraction-free environment for deep analysis and long-form consumption. The emotional response is one of serious inquiry, professional reliability, and quiet luxury.

## Colors

The palette is anchored in **Deep Ink Obsidian**, providing a high-contrast foundation that reduces eye strain during prolonged analytical work. The use of **Midnight Blue-Slate** for surface elements creates a subtle tonal layering that differentiates content blocks without relying on shadows.

**Burnished Gold** serves as the sole accent, used sparingly for calls to action, active states, and critical highlights to maintain an atmosphere of sophistication. All structural elements—dividers, borders, and grid lines—utilize the **Deep Slate rule**, ensuring the "newspaper" scaffolding remains visible but unobtrusive.

## Typography

The typographic hierarchy is the primary driver of the design system's editorial feel.

- **Playfair Display** (Serif) is reserved for headlines and section titles, providing a classic, literary tone.
- **Satoshi** (Sans-serif) is the workhorse for body copy, selected for its modern geometric clarity and exceptional legibility at small sizes.
- **JetBrains Mono** (Monospace) is utilized for all metadata, tabular data, and technical annotations, signaling a high-density "analytical" environment.

Maintain tight line heights for headlines to mimic traditional news print, but allow for generous line-height in body copy (1.5–1.6) to facilitate long-form reading in dark mode.

## Layout & Spacing

This design system employs a **Fixed-Fluid Hybrid Grid**. Content is housed within a 12-column layout with a maximum container width of 1440px.

Structural depth is achieved through the use of "the rule." Vertical and horizontal 1px lines (Deep Slate) should be used to separate columns and sections, mirroring the compartmentalized nature of a financial newspaper. Spacing follows a strict 4px baseline, but editorial sections should utilize the "section-gap" (64px) to provide moments of pause between dense data clusters.

Margins are generous (48px) to frame the content as a curated object within the viewport.

## Elevation & Depth

In this design system, **shadows are strictly prohibited**. Depth is communicated through two primary methods:

1.  **Tonal Layering:** The primary background (#0B0F19) represents the base floor. Surface containers (#151B28) are used to group related modules or data sets, creating a "stacked" appearance.
2.  **Explicit Framing:** 1px solid rules (#2E3748) are the primary tool for spatial definition. Every card, header, and sidebar is bounded by these rules.

To indicate interaction or "lifting," the background color of a surface should shift slightly lighter or the border color should transition to the Burnished Gold accent, rather than introducing a shadow.

## Shapes

The shape language is uncompromisingly **sharp**. Every UI element—including buttons, input fields, dropdowns, and cards—must feature 0px corners. This reinforces the architectural and journalistic integrity of the interface.

Avoid the use of circles or pill shapes even for avatars; use squares or rectangles with 1px borders to maintain the system's structural cohesion.

## Components

### Buttons

Buttons are strictly rectangular (0px radius). Primary buttons use the Burnished Gold background with black text. Secondary buttons are transparent with a 1px Deep Slate rule and Off-White text. On hover, secondary buttons should fill with a subtle 10% white opacity.

### Cards & Containers

Cards are defined by 1px Deep Slate rules. For analytical density, cards should not have internal padding larger than 24px. Use horizontal rules to separate card headers from the body.

### Data Tables

Tables are the core of the analytical experience. Use JetBrains Mono for all cell content. Header cells should be styled with `label-caps` and a subtle Slate-Midnight background. Every row should be separated by a 1px horizontal rule; vertical rules should only be used to separate fixed columns.

### Input Fields

Inputs are minimal: a 1px bottom border that transforms into a full 1px Burnished Gold box upon focus. Labels should use the `label-caps` style positioned above the input field.

### Chips & Tags

Tags are small rectangular boxes with a 1px border. They do not use solid background fills unless they are in an "active" or "selected" state, in which case they adopt the Burnished Gold accent.
