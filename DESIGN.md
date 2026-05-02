---
name: Gilded Vanguard
colors:
  surface: "#131313"
  surface-dim: "#131313"
  surface-bright: "#393939"
  surface-container-lowest: "#0e0e0e"
  surface-container-low: "#1b1b1b"
  surface-container: "#1f1f1f"
  surface-container-high: "#2a2a2a"
  surface-container-highest: "#353535"
  on-surface: "#e2e2e2"
  on-surface-variant: "#c3c8c0"
  inverse-surface: "#e2e2e2"
  inverse-on-surface: "#303030"
  outline: "#8d928b"
  outline-variant: "#434842"
  surface-tint: "#b5cdb4"
  primary: "#b5cdb4"
  on-primary: "#213523"
  primary-container: "#0d2111"
  on-primary-container: "#748b74"
  inverse-primary: "#4e644f"
  secondary: "#e9c349"
  on-secondary: "#3c2f00"
  secondary-container: "#af8d11"
  on-secondary-container: "#342800"
  tertiary: "#b5cdb5"
  on-tertiary: "#213524"
  tertiary-container: "#0d2112"
  on-tertiary-container: "#748b76"
  error: "#ffb4ab"
  on-error: "#690005"
  error-container: "#93000a"
  on-error-container: "#ffdad6"
  primary-fixed: "#d0e9cf"
  primary-fixed-dim: "#b5cdb4"
  on-primary-fixed: "#0c2010"
  on-primary-fixed-variant: "#374c39"
  secondary-fixed: "#ffe088"
  secondary-fixed-dim: "#e9c349"
  on-secondary-fixed: "#241a00"
  on-secondary-fixed-variant: "#574500"
  tertiary-fixed: "#d0e9d1"
  tertiary-fixed-dim: "#b5cdb5"
  on-tertiary-fixed: "#0c2011"
  on-tertiary-fixed-variant: "#374c3a"
  background: "#131313"
  on-background: "#e2e2e2"
  surface-variant: "#353535"
typography:
  display-xl:
    fontFamily: Space Grotesk
    fontSize: 48px
    fontWeight: "700"
    lineHeight: "1.1"
    letterSpacing: -0.02em
  headline-lg:
    fontFamily: Space Grotesk
    fontSize: 32px
    fontWeight: "600"
    lineHeight: "1.2"
    letterSpacing: -0.01em
  headline-md:
    fontFamily: Space Grotesk
    fontSize: 24px
    fontWeight: "600"
    lineHeight: "1.3"
  body-lg:
    fontFamily: Space Grotesk
    fontSize: 18px
    fontWeight: "400"
    lineHeight: "1.6"
  body-md:
    fontFamily: Space Grotesk
    fontSize: 16px
    fontWeight: "400"
    lineHeight: "1.6"
  label-caps:
    fontFamily: Space Grotesk
    fontSize: 12px
    fontWeight: "700"
    lineHeight: "1"
    letterSpacing: 0.1em
  code-data:
    fontFamily: Space Grotesk
    fontSize: 14px
    fontWeight: "500"
    lineHeight: "1.4"
    letterSpacing: 0.02em
rounded:
  sm: 0.25rem
  DEFAULT: 0.5rem
  md: 0.75rem
  lg: 1rem
  xl: 1.5rem
  full: 9999px
spacing:
  unit: 4px
  gutter: 24px
  margin: 40px
  container-padding: 20px
  stack-sm: 8px
  stack-md: 16px
  stack-lg: 32px
---

## Brand & Style

This design system establishes a high-energy tactical command center aesthetic, blending the stealth of a forest-ops environment with the prestige of a premium military-grade interface. The brand personality is authoritative, precise, and elite, targeting users who require high-density information visualization without sacrificing visual sophistication.

The design style is a fusion of **Glassmorphism** and **High-Contrast Minimalism**. It utilizes deep atmospheric depth, layered translucent surfaces, and sharp, technical accents to evoke a sense of advanced situational awareness. Every element is designed to feel like a high-end hardware interface—functional, rugged, and exceptionally valuable.

## Colors

The palette is anchored by a "Pitch Black" (#000000) foundation to maximize contrast and eliminate visual noise. The primary brand color, a deep Forest Green, provides a rich, tonal depth for secondary surfaces and containers, creating a sense of night-vision hardware.

The Metallic Gold serves as the functional highlight, used exclusively for critical data points, active states, and call-to-action elements. This gold should be treated as a "light source" within the UI, occasionally paired with subtle outer glows to simulate a high-energy display. Neutral tones are kept to a minimum, using desaturated greens and golds to maintain a cohesive atmospheric temperature.

## Typography

This design system utilizes **Space Grotesk** across all levels to reinforce a technical, futuristic, and geometric feel. The typography relies on heavy weight distribution and high-contrast sizing to establish a clear hierarchy within dense data views.

Headlines should be tight and aggressive. Body text is prioritized for legibility with generous line heights. Labels and technical data points should utilize uppercase styling and increased letter spacing to mimic the readouts of tactical displays and avionics.

## Layout & Spacing

The layout follows a **Fixed Grid** model within a flexible viewport, utilizing a 12-column system. The rhythm is strictly based on a 4px baseline grid to ensure mathematical precision in component alignment.

Large-scale modules are separated by 24px gutters to allow the pitch-black background to act as a "void" between glass panels. Internal component padding is generous (20px) to balance the high-density information with visual breathing room, ensuring the "command center" feel remains organized rather than cluttered.

## Elevation & Depth

Depth is achieved through **Glassmorphism** and tonal layering rather than traditional shadows.

1. **Base Layer:** Pure #000000 background.
2. **Glass Panels:** Semi-transparent Forest Green (#0D2111 at 60-80% opacity) with a 20px backdrop blur and a thin, 1px inner border in a lighter green or gold.
3. **Active State:** Elements requiring focus utilize a subtle Metallic Gold outer glow (15-20% opacity) to appear as if they are emitting light.
4. **Interaction:** Hover states increase the opacity of the glass panel and brighten the border stroke, creating a tactile "activation" effect.

## Shapes

The shape language is defined by large, sophisticated radii paired with razor-sharp internal elements. All primary containers and glass panels use a consistent **20px corner radius**, softening the aggressive nature of the high-contrast colors and providing a premium, hardware-inspired feel.

Smaller interactive components like buttons and input fields follow a standard 8px radius, while status chips and notification pips use pill-shaped geometry to stand out against the structured grid.

## Components

### Buttons

Primary buttons are solid Metallic Gold with black text, featuring a subtle "glint" gradient. Secondary buttons use a ghost style with a 1px Gold border and 20px blurred glass background.

### Cards & Panels

All cards must utilize the 20px rounded glassmorphic style. Headers within cards should be separated by a 1px divider in a desaturated Forest Green.

### Input Fields

Inputs are dark, recessed containers with 8px rounding. The focus state triggers a Metallic Gold border and a matching 12px blur glow, making the field appear "powered on."

### Status Indicators

Use circular "LED" pips. A pulsating Gold pip indicates high-priority activity. A static Forest Green pip indicates standby or "all-clear" status.

### Tactical Data Visualization

Charts and graphs should use thin Metallic Gold lines for data paths. Fill areas should use a low-opacity Gold gradient that fades into the black background. All grid lines in charts should be kept at 10% opacity Forest Green.
