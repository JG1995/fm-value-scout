<script lang="ts">
	import { page } from "$app/stores";

	interface Props {
		href: string;
		label: string;
		variant?: "default" | "primary";
		children: import("svelte").Snippet;
	}

	let { href, label, variant = "default", children }: Props = $props();

	let isActive = $derived($page.url.pathname === href);
</script>

<a
	{href}
	class="nav-link"
	class:primary={variant === "primary"}
	class:active={isActive}
	aria-label={label}
	data-tooltip={label}
	aria-current={isActive ? "page" : undefined}
>
	{@render children()}
</a>

<style>
	.nav-link {
		display: flex;
		align-items: center;
		justify-content: center;
		width: var(--nav-link-size, 48px);
		height: var(--nav-link-size, 48px);
		border-radius: var(--radius-md);
		color: var(--color-gold);
		text-decoration: none;
		transition:
			border-color 0.2s ease,
			box-shadow 0.2s ease,
			background 0.2s ease,
			opacity 0.2s ease;
		border: 1px solid transparent;
		position: relative;
	}

	.nav-link :global(svg) {
		width: 24px;
		height: 24px;
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

	/* Primary variant - inverted colors */
	.nav-link.primary {
		background: var(--color-gold);
		color: var(--color-primary-container);
		border-color: var(--color-gold);
		margin-bottom: var(--space-6);
	}

	.nav-link.primary:hover {
		opacity: 0.9;
	}

	.nav-link.primary.active {
		opacity: 0.85;
	}

	/* Tooltip on hover */
	.nav-link::after {
		content: attr(data-tooltip);
		position: absolute;
		left: calc(100% + 24px);
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
