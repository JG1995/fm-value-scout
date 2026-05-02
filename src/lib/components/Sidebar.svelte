<script lang="ts">
	import { page } from "$app/stores";

	// Secondary items (top of sidebar)
	const secondaryItems = [
		{
			href: "/database",
			label: "Database",
			icon: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
				<ellipse cx="12" cy="5" rx="9" ry="3"/>
				<path d="M3 5V19A9 3 0 0 0 21 19V5"/>
				<path d="M3 12A9 3 0 0 0 21 12"/>
			</svg>`,
		},
		{
			href: "/archetypes",
			label: "Archetypes",
			icon: `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
				<path d="M12 20a8 8 0 1 0 0-16 8 8 0 0 0 0 16Z"/>
				<path d="M12 14a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z"/>
				<path d="M12 2v2"/>
				<path d="M12 20v2"/>
				<path d="m4.93 4.93 1.41 1.41"/>
				<path d="m17.66 17.66 1.41 1.41"/>
				<path d="M2 12h2"/>
				<path d="M20 12h2"/>
				<path d="m6.34 17.66-1.41 1.41"/>
				<path d="m19.07 4.93-1.41 1.41"/>
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
</style>
