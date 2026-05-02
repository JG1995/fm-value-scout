<script lang="ts">
	import NavLink from "./NavLink.svelte";
	import { IconSquad, IconScouting, IconArchetypes, IconMetrics, IconUpload } from "$lib/icons";

	// Secondary items (top of sidebar)
	const secondaryItems = [
		{ href: "/squad", label: "My Squad", icon: IconSquad },
		{ href: "/scouting", label: "Moneyball Scouting", icon: IconScouting },
		{ href: "/archetypes", label: "Custom Archetypes", icon: IconArchetypes },
		{ href: "/metrics", label: "Custom Metrics", icon: IconMetrics },
	] as const;
</script>

<nav class="sidebar" aria-label="Main navigation">
	{#each secondaryItems as item}
		<NavLink href={item.href} label={item.label}>
			<item.icon />
		</NavLink>
	{/each}

	<!-- Spacer to push primary item to bottom -->
	<div class="spacer"></div>

	<!-- Primary item with inverted colors -->
	<NavLink href="/" label="Upload" variant="primary">
		<IconUpload />
	</NavLink>
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
</style>
