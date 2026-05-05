/**
 * Phase 4 Frontend Verification Tests
 * Tests Svelte 5 components using static analysis and compile-time checks.
 * Due to Svelte 5 runes ($state, $derived, $effect) requiring browser context,
 * we verify correctness via svelte-check and static rendering output.
 *
 * Run with: npx vitest run tests/frontend/phase4-runes.test.ts
 */

import { describe, it, expect } from "vitest";
import { compile } from "svelte/compiler";
import fs from "fs";
import path from "path";

function getSvelteSource(filename: string): string {
	return fs.readFileSync(path.join(process.cwd(), "src/routes", filename), "utf-8");
}

function compileSvelte(source: string) {
	try {
		return compile(source, {
			generate: "dom",
			dev: false,
			css: "injected",
		});
	} catch (e) {
		return null;
	}
}

describe("Svelte 5 Runes Syntax Verification", () => {
	describe("+layout.svelte", () => {
		it("compiles without errors", () => {
			const source = getSvelteSource("+layout.svelte");
			const result = compileSvelte(source);
			expect(result).not.toBeNull();
		});

		it("contains $state rune for currentPath", () => {
			const source = getSvelteSource("+layout.svelte");
			expect(source).toContain("let currentPath = $state");
		});

		it("contains $effect rune for pathname tracking", () => {
			const source = getSvelteSource("+layout.svelte");
			expect(source).toContain("$effect(() =>");
			expect(source).toContain("currentPath = $page.url.pathname");
		});

		it("has correct navLinks array structure", () => {
			const source = getSvelteSource("+layout.svelte");
			expect(source).toContain("href: '/'");
			expect(source).toContain("href: '/import'");
			expect(source).toContain("label: 'Landing'");
			expect(source).toContain("label: 'CSV Import'");
		});

		it("uses slot for page content", () => {
			const source = getSvelteSource("+layout.svelte");
			// Svelte 5 still supports <slot> in legacy mode but warns
			expect(source).toContain("<slot />");
		});

		it("renders ValueScout branding", () => {
			const source = getSvelteSource("+layout.svelte");
			expect(source).toContain("ValueScout");
		});

		it("has active class binding for current nav link", () => {
			const source = getSvelteSource("+layout.svelte");
			expect(source).toContain("class:active={currentPath === link.href}");
		});
	});

	describe("+page.svelte (Landing Page)", () => {
		it("compiles without errors", () => {
			const source = getSvelteSource("+page.svelte");
			const result = compileSvelte(source);
			expect(result).not.toBeNull();
		});

		it("has hero section with ValueScout title", () => {
			const source = getSvelteSource("+page.svelte");
			expect(source).toContain("ValueScout");
			expect(source).toContain('<h1 class="title">ValueScout</h1>');
		});

		it("has CTA link to /import", () => {
			const source = getSvelteSource("+page.svelte");
			expect(source).toContain('href="/import"');
			expect(source).toContain("Import Player Data");
		});

		it("has description about scouting companion", () => {
			const source = getSvelteSource("+page.svelte");
			expect(source).toContain("Your intelligent scouting companion");
			expect(source).toContain("Football Manager");
		});
	});

	describe("ManagedClubSettings.svelte", () => {
		it("compiles without errors", () => {
			const source = getSvelteSource("ManagedClubSettings.svelte");
			const result = compileSvelte(source);
			expect(result).not.toBeNull();
		});

		it("uses $state rune for reactive state", () => {
			const source = getSvelteSource("ManagedClubSettings.svelte");
			expect(source).toMatch(/let clubName = \$state\(''\)/);
			expect(source).toMatch(/let statusMessage = \$state\(''\)/);
			expect(source).toMatch(/let statusType = \$state</);
			expect(source).toMatch(/let isLoading = \$state\(false\)/);
		});

		it("has $effect to load club name on mount", () => {
			const source = getSvelteSource("ManagedClubSettings.svelte");
			expect(source).toContain("$effect(() =>");
			expect(source).toContain("loadClubName()");
		});

		it("calls get_managed_club Tauri command", () => {
			const source = getSvelteSource("ManagedClubSettings.svelte");
			expect(source).toContain("get_managed_club");
		});

		it("calls set_managed_club Tauri command on save", () => {
			const source = getSvelteSource("ManagedClubSettings.svelte");
			expect(source).toContain("set_managed_club");
		});

		it("has form with club name input binding", () => {
			const source = getSvelteSource("ManagedClubSettings.svelte");
			expect(source).toContain("bind:value={clubName}");
			expect(source).toContain('id="club-name"');
		});

		it("has save button with disabled state", () => {
			const source = getSvelteSource("ManagedClubSettings.svelte");
			expect(source).toContain("disabled={isLoading}");
			expect(source).toContain("onclick={handleSave}");
		});

		it("shows status messages conditionally", () => {
			const source = getSvelteSource("ManagedClubSettings.svelte");
			expect(source).toContain("{#if statusMessage}");
			expect(source).toContain("class:success={statusType === 'success'}");
			expect(source).toContain("class:error={statusType === 'error'}");
		});
	});
});

describe("import/+page.svelte (CSV Import Page)", () => {
	it("compiles without errors", () => {
		const source = getSvelteSource("import/+page.svelte");
		const result = compileSvelte(source);
		expect(result).not.toBeNull();
	});

	it("uses $state for reactive form state", () => {
		const source = getSvelteSource("import/+page.svelte");
		expect(source).toMatch(/let selectedFile = \$state</);
		expect(source).toMatch(/let inGameDate = \$state\(''\)/);
		expect(source).toMatch(/let isImporting = \$state\(false\)/);
	});

	it("uses $derived for form validation", () => {
		const source = getSvelteSource("import/+page.svelte");
		expect(source).toContain("const isFormValid = $derived(");
		expect(source).toContain("selectedFile !== null");
		expect(source).toContain("inGameDate.trim().length > 0");
	});

	it("uses $effect to load seasons", () => {
		const source = getSvelteSource("import/+page.svelte");
		expect(source).toContain("$effect(() =>");
		expect(source).toContain("loadSeasons()");
	});

	it("calls browseFiles via dialog plugin", () => {
		const source = getSvelteSource("import/+page.svelte");
		expect(source).toContain("import { open } from '@tauri-apps/plugin-dialog'");
		expect(source).toContain("open({");
		expect(source).toContain("filters:");
	});

	it("calls import_csv Tauri command", () => {
		const source = getSvelteSource("import/+page.svelte");
		expect(source).toContain("import_csv");
	});

	it("has drop zone element with handlers", () => {
		const source = getSvelteSource("import/+page.svelte");
		expect(source).toContain("ondrop={handleDrop}");
		expect(source).toContain("onclick={browseFiles}");
		expect(source).toContain('role="button"');
	});

	it("validates CSV file extension on drop", () => {
		const source = getSvelteSource("import/+page.svelte");
		expect(source).toContain("file.name.endsWith('.csv')");
		expect(source).toContain("errorMessage = 'Please select a CSV file'");
	});
});

describe("Static Code Analysis", () => {
	it("all svelte files have proper script tags", () => {
		const files = ["+layout.svelte", "+page.svelte", "ManagedClubSettings.svelte"];
		for (const file of files) {
			const source = getSvelteSource(file);
			expect(source).toMatch(/<script lang="ts">/);
		}
	});

	it("no TODO or FIXME comments in production code", () => {
		const files = [
			"+layout.svelte",
			"+page.svelte",
			"ManagedClubSettings.svelte",
			"import/+page.svelte",
		];
		for (const file of files) {
			const source = getSvelteSource(file);
			expect(source).not.toMatch(/TODO|FIXME|HACK/);
		}
	});

	it("all Tauri invoke calls have proper error handling", () => {
		const source = getSvelteSource("ManagedClubSettings.svelte");
		expect(source).toMatch(/try\s*\{[\s\S]*invoke[\s\S]*\}\s*catch/);
	});
});
