<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { open } from "@tauri-apps/plugin-dialog";

	interface ImportResult {
		season_id: number;
		season_name: string;
		players_imported: number;
		rows_rejected: Array<{ row_number: number; reasons: string[] }>;
	}

	interface SeasonInfo {
		id: number;
		name: string;
		in_game_date: string;
		import_date: string;
		filename: string;
		player_count: number;
	}

	let selectedFile = $state<string | null>(null);
	let inGameDate = $state("");
	let isImporting = $state(false);
	let importProgress = $state(0);
	let importResult = $state<ImportResult | null>(null);
	let errorMessage = $state<string | null>(null);
	let seasons = $state<SeasonInfo[]>([]);

	const isFormValid = $derived(selectedFile !== null && inGameDate.trim().length > 0);

	async function loadSeasons() {
		try {
			seasons = await invoke<SeasonInfo[]>("get_seasons");
		} catch {
			// Silently fail - seasons are optional for import page
		}
	}

	$effect(() => {
		loadSeasons();
	});

	function handleDrop(event: DragEvent) {
		event.preventDefault();
		const files = event.dataTransfer?.files;
		if (files && files.length > 0) {
			const file = files[0];
			if (file.name.endsWith(".csv")) {
				// Trigger dialog to get full path
				browseFiles();
			} else {
				errorMessage = "Please select a CSV file";
			}
		}
	}

	async function browseFiles() {
		try {
			const selected = await open({
				multiple: false,
				filters: [{ name: "CSV Files", extensions: ["csv"] }],
			});
			if (selected) {
				selectedFile = selected as string;
				errorMessage = null;
			}
		} catch {
			errorMessage = "Failed to open file dialog";
		}
	}

	async function handleImport() {
		if (!isFormValid || isImporting) return;

		isImporting = true;
		importProgress = 0;
		errorMessage = null;
		importResult = null;

		// Simulate progress during import
		const progressInterval = setInterval(() => {
			if (importProgress < 90) {
				importProgress += 10;
			}
		}, 200);

		try {
			const result = await invoke<ImportResult>("import_csv", {
				filePath: selectedFile,
				inGameDate: inGameDate.trim(),
			});
			importProgress = 100;
			importResult = result;
			selectedFile = null;
			inGameDate = "";
		} catch (err) {
			errorMessage = err instanceof Error ? err.message : String(err);
		} finally {
			clearInterval(progressInterval);
			isImporting = false;
		}
	}
</script>

<main class="import-page">
	<div class="import-container">
		<header class="import-header">
			<h1 class="page-title">CSV Import</h1>
			<p class="page-description">Import player data from Football Manager scouting reports</p>
		</header>

		<div class="import-content">
			<!-- Drop Zone -->
			<div
				class="drop-zone"
				class:has-file={selectedFile}
				role="button"
				tabindex="0"
				onclick={browseFiles}
				onkeydown={(e) => e.key === "Enter" && browseFiles()}
				ondrop={handleDrop}
			>
				{#if selectedFile}
					<div class="file-selected">
						<svg
							class="file-icon"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="1.5"
						>
							<path
								d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
							/>
						</svg>
						<span class="file-name">{selectedFile}</span>
						<span class="file-hint">Click or drop another file to change</span>
					</div>
				{:else}
					<div class="drop-prompt">
						<svg
							class="upload-icon"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="1.5"
						>
							<path
								d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
							/>
						</svg>
						<span class="drop-text">Drop CSV file here</span>
						<span class="drop-subtext">or click to browse</span>
					</div>
				{/if}
			</div>

			<!-- File Picker Button -->
			<button class="browse-button" onclick={browseFiles} disabled={isImporting}>
				<svg
					class="folder-icon"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="1.5"
				>
					<path d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
				</svg>
				Browse Files
			</button>

			<!-- Date Input -->
			<div class="input-group">
				<label class="input-label" for="in-game-date">In-Game Date</label>
				<input
					id="in-game-date"
					type="text"
					class="date-input"
					placeholder="15.6.2029"
					bind:value={inGameDate}
					disabled={isImporting}
				/>
				<span class="input-hint">Format: day.month.year (e.g., 15.6.2029)</span>
			</div>

			<!-- Import Button -->
			<button class="import-button" onclick={handleImport} disabled={!isFormValid || isImporting}>
				{#if isImporting}
					Importing...
				{:else}
					Import CSV
				{/if}
			</button>

			<!-- Progress Bar -->
			{#if isImporting}
				<div class="progress-container">
					<div class="progress-label">
						<span>Importing data</span>
						<span class="progress-percent">{importProgress}%</span>
					</div>
					<div class="progress-track">
						<div class="progress-fill" style="width: {importProgress}%"></div>
					</div>
				</div>
			{/if}

			<!-- Error Message -->
			{#if errorMessage}
				<div class="error-card">
					<svg
						class="error-icon"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="1.5"
					>
						<path d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
					</svg>
					<span>{errorMessage}</span>
				</div>
			{/if}

			<!-- Summary Card -->
			{#if importResult}
				<div class="summary-card">
					<div class="summary-header">
						<svg
							class="success-icon"
							viewBox="0 0 24 24"
							fill="none"
							stroke="currentColor"
							stroke-width="1.5"
						>
							<path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
						</svg>
						<h2 class="summary-title">Import Complete</h2>
					</div>
					<div class="summary-stats">
						<div class="stat-row">
							<span class="stat-label">Players Imported</span>
							<span class="stat-value">{importResult.players_imported}</span>
						</div>
						<div class="stat-row">
							<span class="stat-label">Season</span>
							<span class="stat-value season-name">{importResult.season_name}</span>
						</div>
						<div class="stat-row">
							<span class="stat-label">Season ID</span>
							<span class="stat-value mono">{importResult.season_id}</span>
						</div>
						{#if importResult.rows_rejected.length > 0}
							<div class="stat-row rejected">
								<span class="stat-label">Rows Rejected</span>
								<span class="stat-value">{importResult.rows_rejected.length}</span>
							</div>
						{/if}
					</div>
					{#if importResult.rows_rejected.length > 0}
						<div class="rejected-list">
							<span class="rejected-header">Rejected Rows</span>
							{#each importResult.rows_rejected.slice(0, 5) as rejected}
								<div class="rejected-row">
									<span class="rejected-number">Row {rejected.row_number}</span>
									<span class="rejected-reasons">{rejected.reasons.join(", ")}</span>
								</div>
							{/each}
							{#if importResult.rows_rejected.length > 5}
								<span class="rejected-more">
									+{importResult.rows_rejected.length - 5} more rows rejected
								</span>
							{/if}
						</div>
					{/if}
				</div>
			{/if}
		</div>
	</div>
</main>

<style>
	.import-page {
		min-height: calc(100vh - 65px);
		padding: 48px;
		background-color: #0f1417;
	}

	.import-container {
		max-width: 640px;
		margin: 0 auto;
	}

	.import-header {
		margin-bottom: 48px;
		border-bottom: 1px solid #353a3d;
		padding-bottom: 32px;
	}

	.page-title {
		font-family: "Playfair Display", serif;
		font-size: 40px;
		font-weight: 600;
		line-height: 1.2;
		letter-spacing: -0.01em;
		color: #ffd475;
		margin-bottom: 12px;
	}

	.page-description {
		font-family: "Satoshi", sans-serif;
		font-size: 16px;
		font-weight: 400;
		line-height: 1.5;
		color: #d1c5b1;
	}

	.import-content {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	/* Drop Zone */
	.drop-zone {
		border: 1px dashed #353a3d;
		background-color: #1b2023;
		padding: 48px 24px;
		cursor: pointer;
		transition:
			border-color 0.15s ease,
			background-color 0.15s ease;
	}

	.drop-zone:hover {
		border-color: #4e4637;
		background-color: #262b2e;
	}

	.drop-zone.dragging {
		border-color: #ffd475;
		border-style: solid;
		background-color: #262b2e;
	}

	.drop-zone.has-file {
		border-style: solid;
		border-color: #4e4637;
	}

	.drop-prompt,
	.file-selected {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 12px;
	}

	.upload-icon,
	.file-icon {
		width: 48px;
		height: 48px;
		color: #d1c5b1;
	}

	.drop-text {
		font-family: "Satoshi", sans-serif;
		font-size: 18px;
		font-weight: 500;
		color: #dfe3e7;
	}

	.drop-subtext {
		font-family: "Satoshi", sans-serif;
		font-size: 14px;
		color: #d1c5b1;
	}

	.file-name {
		font-family: "JetBrains Mono", monospace;
		font-size: 13px;
		font-weight: 500;
		letter-spacing: 0.02em;
		color: #ffd475;
	}

	.file-hint {
		font-family: "Satoshi", sans-serif;
		font-size: 12px;
		color: #d1c5b1;
	}

	/* Browse Button */
	.browse-button {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		font-family: "Satoshi", sans-serif;
		font-size: 14px;
		font-weight: 700;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: #dfe3e7;
		background-color: transparent;
		border: 1px solid #353a3d;
		padding: 12px 24px;
		cursor: pointer;
		transition:
			background-color 0.15s ease,
			border-color 0.15s ease;
	}

	.browse-button:hover:not(:disabled) {
		background-color: rgba(255, 255, 255, 0.1);
		border-color: #4e4637;
	}

	.browse-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.folder-icon {
		width: 18px;
		height: 18px;
	}

	/* Input Group */
	.input-group {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.input-label {
		font-family: "Satoshi", sans-serif;
		font-size: 12px;
		font-weight: 700;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: #d1c5b1;
	}

	.date-input {
		font-family: "JetBrains Mono", monospace;
		font-size: 16px;
		font-weight: 500;
		color: #dfe3e7;
		background-color: #1b2023;
		border: 1px solid #353a3d;
		border-bottom-width: 2px;
		padding: 12px 16px;
		outline: none;
		transition: border-color 0.15s ease;
	}

	.date-input::placeholder {
		color: #4e4637;
	}

	.date-input:focus {
		border-color: #ffd475;
	}

	.date-input:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.input-hint {
		font-family: "Satoshi", sans-serif;
		font-size: 12px;
		color: #d1c5b1;
	}

	/* Import Button */
	.import-button {
		font-family: "Satoshi", sans-serif;
		font-size: 14px;
		font-weight: 700;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: #3f2e00;
		background-color: #ffd475;
		border: 1px solid #ffd475;
		padding: 16px 32px;
		cursor: pointer;
		transition: background-color 0.15s ease;
	}

	.import-button:hover:not(:disabled) {
		background-color: #ecc161;
		border-color: #ecc161;
	}

	.import-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Progress Bar */
	.progress-container {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.progress-label {
		display: flex;
		justify-content: space-between;
		font-family: "Satoshi", sans-serif;
		font-size: 12px;
		font-weight: 500;
		color: #d1c5b1;
	}

	.progress-percent {
		font-family: "JetBrains Mono", monospace;
		color: #ffd475;
	}

	.progress-track {
		height: 4px;
		background-color: #262b2e;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background-color: #ffd475;
		transition: width 0.3s ease;
	}

	/* Error Card */
	.error-card {
		display: flex;
		align-items: center;
		gap: 12px;
		border: 1px solid #93000a;
		background-color: rgba(147, 0, 10, 0.1);
		padding: 16px;
	}

	.error-icon {
		width: 20px;
		height: 20px;
		color: #ffb4ab;
		flex-shrink: 0;
	}

	.error-card span {
		font-family: "Satoshi", sans-serif;
		font-size: 14px;
		color: #ffb4ab;
	}

	/* Summary Card */
	.summary-card {
		border: 1px solid #353a3d;
		background-color: #1b2023;
		padding: 24px;
	}

	.summary-header {
		display: flex;
		align-items: center;
		gap: 12px;
		padding-bottom: 16px;
		border-bottom: 1px solid #353a3d;
		margin-bottom: 16px;
	}

	.success-icon {
		width: 24px;
		height: 24px;
		color: #ffd475;
	}

	.summary-title {
		font-family: "Playfair Display", serif;
		font-size: 28px;
		font-weight: 600;
		color: #dfe3e7;
	}

	.summary-stats {
		display: flex;
		flex-direction: column;
		gap: 12px;
	}

	.stat-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 8px 0;
		border-bottom: 1px solid #262b2e;
	}

	.stat-row:last-child {
		border-bottom: none;
	}

	.stat-row.rejected {
		color: #ffb4ab;
	}

	.stat-label {
		font-family: "Satoshi", sans-serif;
		font-size: 14px;
		color: #d1c5b1;
	}

	.stat-value {
		font-family: "Satoshi", sans-serif;
		font-size: 16px;
		font-weight: 500;
		color: #dfe3e7;
	}

	.stat-value.mono {
		font-family: "JetBrains Mono", monospace;
		font-size: 13px;
	}

	.stat-value.season-name {
		color: #ffd475;
	}

	/* Rejected List */
	.rejected-list {
		margin-top: 16px;
		padding-top: 16px;
		border-top: 1px solid #353a3d;
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.rejected-header {
		font-family: "Satoshi", sans-serif;
		font-size: 12px;
		font-weight: 700;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: #ffb4ab;
	}

	.rejected-row {
		display: flex;
		justify-content: space-between;
		padding: 8px;
		background-color: #262b2e;
	}

	.rejected-number {
		font-family: "JetBrains Mono", monospace;
		font-size: 12px;
		color: #d1c5b1;
	}

	.rejected-reasons {
		font-family: "Satoshi", sans-serif;
		font-size: 12px;
		color: #ffb4ab;
	}

	.rejected-more {
		font-family: "Satoshi", sans-serif;
		font-size: 12px;
		color: #d1c5b1;
		text-align: center;
		padding-top: 8px;
	}
</style>
