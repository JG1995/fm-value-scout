/**
 * Design Tokens — TypeScript Mirror
 * Mirrors src/styles/tokens.css for programmatic access (JS/TS/Canvas)
 */

export const colors = {
	// Core surfaces
	background: "#131313",
	surface: "#131313",
	surfaceDim: "#131313",
	surfaceBright: "#393939",
	surfaceContainerLowest: "#0e0e0e",
	surfaceContainerLow: "#1b1b1b",
	surfaceContainer: "#1f1f1f",
	surfaceContainerHigh: "#2a2a2a",
	surfaceContainerHighest: "#353535",
	onSurface: "#e2e2e2",
	onSurfaceVariant: "#c3c8c0",
	inverseSurface: "#e2e2e2",
	inverseOnSurface: "#303030",
	outline: "#8d928b",
	outlineVariant: "#434842",
	surfaceTint: "#b5cdb4",

	// Primary (Forest Green)
	primary: "#b5cdb4",
	onPrimary: "#213523",
	primaryContainer: "#0d2111",
	onPrimaryContainer: "#748b74",
	inversePrimary: "#4e644f",

	// Secondary (Metallic Gold)
	secondary: "#e9c349",
	onSecondary: "#3c2f00",
	secondaryContainer: "#af8d11",
	onSecondaryContainer: "#342800",

	// Tertiary
	tertiary: "#b5cdb5",
	onTertiary: "#213524",
	tertiaryContainer: "#0d2112",
	onTertiaryContainer: "#748b76",

	// Error
	error: "#ffb4ab",
	onError: "#690005",
	errorContainer: "#93000a",
	onErrorContainer: "#ffdad6",

	// Fixed
	primaryFixed: "#d0e9cf",
	primaryFixedDim: "#b5cdb4",
	onPrimaryFixed: "#0c2010",
	onPrimaryFixedVariant: "#374c39",

	secondaryFixed: "#ffe088",
	secondaryFixedDim: "#e9c349",
	onSecondaryFixed: "#241a00",
	onSecondaryFixedVariant: "#574500",

	tertiaryFixed: "#d0e9d1",
	tertiaryFixedDim: "#b5cdb5",
	onTertiaryFixed: "#0c2011",
	onTertiaryFixedVariant: "#374c3a",

	// Semantic aliases
	gold: "#e9c349",
	goldDim: "rgba(233, 195, 73, 0.15)",
	goldGlow: "rgba(233, 195, 73, 0.2)",
	glassBg: "rgba(13, 33, 17, 0.7)",
	glassBgHover: "rgba(13, 33, 17, 0.85)",
} as const;

export const spacing = {
	unit: "4px",
	// Atomic scale
	1: "4px",
	2: "8px",
	3: "12px",
	4: "16px",
	5: "20px",
	6: "24px",
	8: "32px",
	10: "40px",
	12: "48px",
	16: "64px",
	// Semantic
	gutter: "24px",
	margin: "40px",
	containerPadding: "20px",
	stackSm: "8px",
	stackMd: "16px",
	stackLg: "32px",
} as const;

export const radius = {
	sm: "0.25rem",
	md: "0.5rem",
	lg: "1rem",
	xl: "1.5rem",
	full: "9999px",
	// Semantic
	panel: "20px",
	button: "8px",
} as const;

export const typography = {
	displayXl: {
		fontFamily: "Space Grotesk",
		fontSize: "48px",
		fontWeight: 700,
		lineHeight: "1.1",
		letterSpacing: "-0.02em",
	},
	headlineLg: {
		fontFamily: "Space Grotesk",
		fontSize: "32px",
		fontWeight: 600,
		lineHeight: "1.2",
		letterSpacing: "-0.01em",
	},
	headlineMd: {
		fontFamily: "Space Grotesk",
		fontSize: "24px",
		fontWeight: 600,
		lineHeight: "1.3",
		letterSpacing: "0",
	},
	bodyLg: {
		fontFamily: "Space Grotesk",
		fontSize: "18px",
		fontWeight: 400,
		lineHeight: "1.6",
		letterSpacing: "0",
	},
	bodyMd: {
		fontFamily: "Space Grotesk",
		fontSize: "16px",
		fontWeight: 400,
		lineHeight: "1.6",
		letterSpacing: "0",
	},
	labelCaps: {
		fontFamily: "Space Grotesk",
		fontSize: "12px",
		fontWeight: 700,
		lineHeight: "1",
		letterSpacing: "0.1em",
	},
	codeData: {
		fontFamily: "Space Grotesk",
		fontSize: "14px",
		fontWeight: 500,
		lineHeight: "1.4",
		letterSpacing: "0.02em",
	},
} as const;

export const blur = {
	panel: "20px",
	glow: "12px",
} as const;
