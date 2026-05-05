<script lang="ts">
</script>

<main class="landing">
	<div class="hero">
		<h1 class="title">ValueScout</h1>
		<p class="description">
			Your intelligent scouting companion for Football Manager. Evaluate players, track transfer
			targets, and uncover hidden gems with data-driven insights.
		</p>
		<a href="/import" class="cta">Import Player Data</a>
	</div>
</main>

<style>
	.landing {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: calc(100vh - 65px);
		padding: 48px;
	}

	.hero {
		max-width: 640px;
		text-align: center;
	}

	.title {
		font-family: "Playfair Display", serif;
		font-size: 64px;
		font-weight: 700;
		line-height: 1.1;
		letter-spacing: -0.02em;
		color: #ffd475;
		margin-bottom: 24px;
	}

	.description {
		font-family: "Satoshi", sans-serif;
		font-size: 18px;
		font-weight: 400;
		line-height: 1.6;
		color: #dfe3e7;
		margin-bottom: 40px;
	}

	.cta {
		display: inline-block;
		font-family: "Satoshi", sans-serif;
		font-size: 16px;
		font-weight: 700;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		text-decoration: none;
		color: #0f1417;
		background-color: #ffd475;
		padding: 16px 32px;
		border: 1px solid #ffd475;
		transition:
			background-color 0.15s ease,
			color 0.15s ease;
	}

	.cta:hover {
		background-color: #ecc161;
		border-color: #ecc161;
	}
</style>
