/**
 * Frontend UI tests for Phase 4 pages
 * Tests Svelte 5 components using Vitest + @testing-library/svelte
 *
 * Run with: npx vitest run tests/frontend/
 */

import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/svelte";
import { tick } from "svelte";

// Mock Tauri APIs before importing any Svelte components that use them
vi.mock("@tauri-apps/api/core", () => ({
	invoke: vi.fn(),
}));

vi.mock("@tauri-apps/plugin-dialog", () => ({
	open: vi.fn(),
}));

describe("Import Page UI Components", () => {
	describe("Drop Zone", () => {
		it("clicking drop zone triggers file dialog via browseFiles", async () => {
			// Import dynamically after mock is set up
			const { open } = await import("@tauri-apps/plugin-dialog");

			// Reset mock state
			vi.clearAllMocks();
			(open as any).mockResolvedValue("/path/to/file.csv");

			// Import the component
			const { default: page } = await import("../../src/routes/import/+page.svelte");
			render(page);

			// The drop zone is a div with role="button"
			const dropZone = screen.getByRole("button");
			expect(dropZone).toBeTruthy();

			// Click the drop zone
			await fireEvent.click(dropZone);

			// Verify open was called with CSV filter
			expect(open).toHaveBeenCalledWith({
				multiple: false,
				filters: [{ name: "CSV Files", extensions: ["csv"] }],
			});
		});

		it("displays error message when non-CSV file is dropped", async () => {
			const { default: page } = await import("../../src/routes/import/+page.svelte");

			render(page);

			// Find drop zone (role="button" with tabindex="0")
			const dropZone = screen.getByRole("button");

			// Create fake non-CSV file
			const fakeFile = new File(["content"], "test.txt", { type: "text/plain" });
			const dataTransfer = {
				files: [fakeFile],
				getData: vi.fn(),
			};

			await fireEvent(
				dropZone,
				new DragEvent("drop", {
					dataTransfer: dataTransfer as any,
				})
			);

			// Check error message appears - the error shows "Please select a CSV file"
			const errorEl = await waitFor(() => screen.getByText(/please select a csv file/i));
			expect(errorEl).toBeTruthy();
		});
	});

	describe("Date Input", () => {
		it("accepts text input for in-game date", async () => {
			const { default: page } = await import("../../src/routes/import/+page.svelte");

			render(page);

			// Get the date input by placeholder
			const dateInput = screen.getByPlaceholderText("15.6.2029") as HTMLInputElement;
			expect(dateInput).toBeTruthy();

			// Type a date
			await fireEvent.input(dateInput, { target: { value: "15.6.2029" } });

			expect(dateInput.value).toBe("15.6.2029");
		});
	});

	describe("Import Button", () => {
		it("is disabled when no file or date is provided", async () => {
			const { default: page } = await import("../../src/routes/import/+page.svelte");

			render(page);

			// Find the import button by its text
			const importButton = screen.getByRole("button", { name: /import csv/i }) as HTMLButtonElement;
			expect(importButton).toBeTruthy();
			expect(importButton.disabled).toBe(true);
		});

		it("is enabled when both file and date are provided", async () => {
			const { invoke } = await import("@tauri-apps/api/core");
			const { open } = await import("@tauri-apps/plugin-dialog");

			// Reset mocks
			vi.clearAllMocks();
			(open as any).mockResolvedValue("/path/to/file.csv");

			const { default: page } = await import("../../src/routes/import/+page.svelte");
			render(page);

			// First click browse to select a file
			const browseButton = screen.getByRole("button", { name: /browse files/i });
			await fireEvent.click(browseButton);
			await tick();

			// Enter a date
			const dateInput = screen.getByPlaceholderText("15.6.2029") as HTMLInputElement;
			await fireEvent.input(dateInput, { target: { value: "15.6.2029" } });

			await tick();

			// Now import button should be enabled
			const importButton = screen.getByRole("button", { name: /import csv/i }) as HTMLButtonElement;
			expect(importButton.disabled).toBe(false);
		});
	});

	describe("Error Message Display", () => {
		it("shows error when non-CSV file dropped", async () => {
			const { default: page } = await import("../../src/routes/import/+page.svelte");

			render(page);

			const dropZone = screen.getByRole("button");
			const fakeFile = new File(["content"], "document.pdf", { type: "application/pdf" });

			await fireEvent(
				dropZone,
				new DragEvent("drop", {
					dataTransfer: { files: [fakeFile], getData: vi.fn() } as any,
				})
			);

			await waitFor(() => {
				expect(screen.getByText(/please select a csv file/i)).toBeTruthy();
			});
		});
	});
});

describe("Layout Navigation", () => {
	it("renders with navigation links", async () => {
		const { default: layout } = await import("../../src/routes/+layout.svelte");

		render(layout);

		// Check nav links exist
		const landingLink = screen.getByRole("link", { name: /landing/i });
		const csvImportLink = screen.getByRole("link", { name: /csv import/i });

		expect(landingLink).toBeTruthy();
		expect(csvImportLink).toBeTruthy();
		expect(landingLink).toHaveAttribute("href", "/");
		expect(csvImportLink).toHaveAttribute("href", "/import");
	});

	it("displays ValueScout title in header", async () => {
		const { default: layout } = await import("../../src/routes/+layout.svelte");

		render(layout);

		const title = screen.getByText("ValueScout");
		expect(title).toBeTruthy();
	});
});

describe("Landing Page", () => {
	it("shows ValueScout title", async () => {
		const { default: landing } = await import("../../src/routes/+page.svelte");

		render(landing);

		const title = screen.getByText("ValueScout");
		expect(title).toBeTruthy();
	});

	it("shows CTA button linking to import page", async () => {
		const { default: landing } = await import("../../src/routes/+page.svelte");

		render(landing);

		const cta = screen.getByRole("link", { name: /import player data/i });
		expect(cta).toBeTruthy();
		expect(cta).toHaveAttribute("href", "/import");
	});

	it("shows description text about scouting companion", async () => {
		const { default: landing } = await import("../../src/routes/+page.svelte");

		render(landing);

		const description = screen.getByText(/your intelligent scouting companion/i);
		expect(description).toBeTruthy();
	});
});
