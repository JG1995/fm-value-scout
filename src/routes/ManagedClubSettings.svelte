<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";

	let clubName = $state("");
	let statusMessage = $state("");
	let statusType = $state<"success" | "error" | "">("");
	let isLoading = $state(false);

	$effect(() => {
		loadClubName();
	});

	async function loadClubName() {
		try {
			isLoading = true;
			const result = await invoke<string | null>("get_managed_club");
			clubName = result ?? "";
		} catch (err) {
			console.error("Failed to load managed club:", err);
			statusMessage = "Failed to load managed club";
			statusType = "error";
		} finally {
			isLoading = false;
		}
	}

	async function handleSave() {
		if (isLoading) return;

		try {
			isLoading = true;
			statusMessage = "";
			statusType = "";

			await invoke("set_managed_club", { clubName });

			statusMessage = "Managed club saved successfully";
			statusType = "success";
		} catch (err) {
			console.error("Failed to save managed club:", err);
			statusMessage = "Failed to save managed club";
			statusType = "error";
		} finally {
			isLoading = false;
		}
	}
</script>

<div class="settings-container">
	<h2 class="section-heading">Managed Club Settings</h2>

	<div class="form-group">
		<label for="club-name" class="label-caps">Club Name</label>
		<input
			id="club-name"
			type="text"
			bind:value={clubName}
			placeholder="Enter managed club name"
			class="club-input"
			disabled={isLoading}
		/>
	</div>

	<button type="button" class="save-button" onclick={handleSave} disabled={isLoading}>
		{isLoading ? "Saving..." : "Save"}
	</button>

	{#if statusMessage}
		<p
			class="status-message"
			class:success={statusType === "success"}
			class:error={statusType === "error"}
		>
			{statusMessage}
		</p>
	{/if}
</div>

<style>
	.settings-container {
		padding: 48px;
		max-width: 600px;
	}

	.section-heading {
		font-family: "Playfair Display", serif;
		font-size: 28px;
		font-weight: 600;
		line-height: 1.3;
		color: #ffd475;
		margin-bottom: 32px;
	}

	.form-group {
		margin-bottom: 24px;
	}

	.label-caps {
		display: block;
		font-family: "Satoshi", sans-serif;
		font-size: 12px;
		font-weight: 700;
		line-height: 1;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: #d1c5b1;
		margin-bottom: 8px;
	}

	.club-input {
		width: 100%;
		background: transparent;
		border: none;
		border-bottom: 1px solid #353a3d;
		padding: 8px 0;
		font-family: "Satoshi", sans-serif;
		font-size: 16px;
		font-weight: 400;
		line-height: 1.5;
		color: #dfe3e7;
		outline: none;
		transition: border-color 0.15s ease;
	}

	.club-input::placeholder {
		color: #9a8f7e;
	}

	.club-input:focus {
		border-bottom-color: #ffd475;
	}

	.club-input:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.save-button {
		font-family: "Satoshi", sans-serif;
		font-size: 16px;
		font-weight: 500;
		color: #3f2e00;
		background-color: #ffd475;
		border: none;
		padding: 12px 24px;
		cursor: pointer;
		transition: background-color 0.15s ease;
		border-radius: 0px;
	}

	.save-button:hover:not(:disabled) {
		background-color: #ecc161;
	}

	.save-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.status-message {
		margin-top: 16px;
		font-family: "Satoshi", sans-serif;
		font-size: 14px;
		font-weight: 400;
		line-height: 1.5;
	}

	.status-message.success {
		color: #88d9a8;
	}

	.status-message.error {
		color: #ffb4ab;
	}
</style>
