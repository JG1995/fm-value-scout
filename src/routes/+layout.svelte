<script lang="ts">
	import { page } from "$app/stores";

	let currentPath = $state("/");

	$effect(() => {
		currentPath = $page.url.pathname;
	});

	const navLinks = [
		{ href: "/", label: "Landing" },
		{ href: "/import", label: "CSV Import" },
	];
</script>

<div class="nav-shell">
	<header class="nav-header">
		<div class="nav-title">ValueScout</div>
		<nav class="nav-links">
			{#each navLinks as link}
				<a href={link.href} class="nav-link" class:active={currentPath === link.href}>
					{link.label}
				</a>
			{/each}
		</nav>
	</header>
</div>

<main class="page-content">
	<slot />
</main>

<style>
	:global(*) {
		margin: 0;
		padding: 0;
		box-sizing: border-box;
	}

	:global(body) {
		background-color: #0f1417;
		color: #dfe3e7;
		font-family: "Satoshi", sans-serif;
		min-height: 100vh;
	}

	.nav-shell {
		border-bottom: 1px solid #353a3d;
	}

	.nav-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		max-width: 1440px;
		margin: 0 auto;
		padding: 0 48px;
		height: 64px;
	}

	.nav-title {
		font-family: "Playfair Display", serif;
		font-size: 28px;
		font-weight: 600;
		color: #ffd475;
		letter-spacing: -0.01em;
	}

	.nav-links {
		display: flex;
		gap: 24px;
	}

	.nav-link {
		font-family: "Satoshi", sans-serif;
		font-size: 16px;
		font-weight: 500;
		color: #d1c5b1;
		text-decoration: none;
		padding: 8px 16px;
		border: 1px solid transparent;
		transition:
			border-color 0.15s ease,
			color 0.15s ease;
	}

	.nav-link:hover {
		color: #dfe3e7;
		border-color: #353a3d;
	}

	.nav-link.active {
		color: #ffd475;
		border-color: #ffd475;
	}

	.page-content {
		min-height: calc(100vh - 65px);
	}
</style>
