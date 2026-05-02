<script lang="ts">
	interface Props {
		class?: string;
		size?: number;
		children: import("svelte").Snippet;
	}

	let { class: className = "", size = 24, children }: Props = $props();
</script>

<svg
	xmlns="http://www.w3.org/2000/svg"
	width={size}
	height={size}
	viewBox="0 0 24 24"
	fill="none"
	stroke="currentColor"
	stroke-width="2"
	stroke-linecap="round"
	stroke-linejoin="round"
	class={className}
>
	{@render children()}
</svg>
