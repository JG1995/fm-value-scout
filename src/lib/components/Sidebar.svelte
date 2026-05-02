<script lang="ts">
	import { page } from "$app/stores";

	// Secondary items (top of sidebar)
	const secondaryItems = [
		{
			href: "/squad",
			label: "My Squad",
			icon: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
				<path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/>
				<circle cx="9" cy="7" r="4"/>
				<path d="M22 21v-2a4 4 0 0 0-3-3.87"/>
				<path d="M16 3.13a4 4 0 0 1 0 7.75"/>
			</svg>`,
		},
		{
			href: "/scouting",
			label: "Moneyball Scouting",
			icon: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
				<circle cx="11" cy="11" r="8"/>
				<path d="m21 21-4.3-4.3"/>
				<path d="M11 8v6"/>
				<path d="M8 11h6"/>
			</svg>`,
		},
		{
			href: "/archetypes",
			label: "Custom Archetypes",
			icon: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
				<line x1="4" x2="4" y1="21" y2="14"/>
				<line x1="4" x2="4" y1="10" y2="3"/>
				<line x1="12" x2="12" y1="21" y2="12"/>
				<line x1="12" x2="12" y1="8" y2="3"/>
				<line x1="20" x2="20" y1="21" y2="16"/>
				<line x1="20" x2="20" y1="12" y2="3"/>
				<line x1="2" x2="6" y1="14" y2="14"/>
				<line x1="10" x2="14" y1="8" y2="8"/>
				<line x1="18" x2="22" y1="16" y2="16"/>
			</svg>`,
		},
		{
			href: "/metrics",
			label: "Custom Metrics",
			icon: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
				<path d="M3 3v16a2 2 0 0 0 2 2h16"/>
				<path d="m19 9-5 5-4-4-3 3"/>
			</svg>`,
		},
	];

	// Primary item (bottom of sidebar) - inverted colors
	const primaryItem = {
		href: "/",
		label: "Upload",
		icon: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
			<path d="M4 14.899A7 7 0 1 1 15.71 8h1.79a4.5 4.5 0 0 1 2.5 8.242"/>
			<path d="M12 12v9"/>
			<path d="m16 16-4-4-4 4"/>
		</svg>`,
	};

	// Derive active state reactively
	const isPrimaryActive = $derived($page.url.pathname === primaryItem.href);
</script>

<nav class="sidebar" aria-label="Main navigation">
	{#each secondaryItems as item}
		{@const isActive = $page.url.pathname === item.href}
		<a
			href={item.href}
			class="nav-link"
			class:active={isActive}
			aria-label={item.label}
			data-tooltip={item.label}
			aria-current={isActive ? "page" : undefined}
		>
			{@html item.icon}
		</a>
	{/each}

	<!-- Spacer to push primary item to bottom -->
	<div class="spacer"></div>

	<!-- Primary item with inverted colors -->
	<a
		href={primaryItem.href}
		class="nav-link primary"
		class:active={isPrimaryActive}
		aria-label={primaryItem.label}
		data-tooltip={primaryItem.label}
		aria-current={isPrimaryActive ? "page" : undefined}
	>
		{@html primaryItem.icon}
	</a>
</nav>

<style>
	.sidebar {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-4) var(--space-3);
		background: var(--color-primary-container);
		width: fit-content;
		height: 100dvh;
		position: fixed;
		left: 0;
		top: 0;
	}

	.spacer {
		flex: 1;
	}

	.nav-link {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		border-radius: var(--radius-md);
		color: var(--color-gold);
		text-decoration: none;
		transition:
			border-color 0.2s ease,
			box-shadow 0.2s ease,
			background 0.2s ease;
		border: 1px solid transparent;
	}

	.nav-link:hover {
		border-color: var(--color-gold);
		box-shadow:
			0 0 20px var(--color-gold-dim),
			0 0 40px var(--color-gold-glow);
	}

	.nav-link.active {
		background: rgba(233, 195, 73, 0.1);
		border-color: var(--color-gold);
	}

	.nav-link:focus-visible {
		outline: 2px solid var(--color-gold);
		outline-offset: 2px;
	}

	.nav-link :global(svg) {
		width: 24px;
		height: 24px;
	}

	/* Primary item - inverted colors */
	.nav-link.primary {
		background: var(--color-gold);
		color: var(--color-primary-container);
		border-color: var(--color-gold);
		margin-bottom: var(--space-6);
	}

	.nav-link.primary:hover {
		opacity: 0.9;
		box-shadow:
			0 0 20px var(--color-gold-dim),
			0 0 40px var(--color-gold-glow);
	}

	.nav-link.primary.active {
		background: var(--color-gold);
		opacity: 0.85;
	}

	.nav-link.primary:focus-visible {
		outline: 2px solid var(--color-gold);
		outline-offset: 2px;
	}

	/* Tooltip label on hover */
	.nav-link::after {
		content: attr(data-tooltip);
		position: absolute;
		left: calc(100% + 8px);
		background: var(--color-surface-container-high);
		color: var(--color-on-surface);
		font-size: var(--font-size-body-md);
		font-weight: 500;
		padding: var(--space-2) var(--space-3);
		border-radius: var(--radius-sm);
		white-space: nowrap;
		opacity: 0;
		pointer-events: none;
		transition: opacity 0.15s ease;
		z-index: 100;
	}

	.nav-link:hover::after {
		opacity: 1;
	}
</style>
