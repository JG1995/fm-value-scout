import { defineConfig } from "vitest/config";
import { sveltekit } from "@sveltejs/kit/vite";

export default defineConfig({
	plugins: [sveltekit()],
	test: {
		include: ["tests/**/*.test.ts"],
		environment: "jsdom",
		globals: true,
		coverage: {
			provider: "v8",
			include: ["src/routes/**/*.svelte"],
		},
		server: {
			deps: {
				inline: ["@sveltejs/kit"],
			},
		},
	},
});
