<script lang="ts">
	let isDragging = $state(false);
	let fileInput: HTMLInputElement;

	function handleDragEnter(e: DragEvent) {
		e.preventDefault();
		isDragging = true;
	}

	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		isDragging = true;
	}

	function handleDragLeave(e: DragEvent) {
		e.preventDefault();
		isDragging = false;
	}

	function handleDrop(e: DragEvent) {
		e.preventDefault();
		isDragging = false;
		// Stubbed: no-op for now per plan
	}

	function handleFileChange(e: Event) {
		const target = e.target as HTMLInputElement;
		if (target.files && target.files.length > 0) {
			// Stubbed: no-op for now per plan
		}
	}

	function handleCardClick() {
		fileInput?.click();
	}
</script>

<div class="page">
	<div
		class="dropzone-card"
		class:dragging={isDragging}
		role="button"
		tabindex="0"
		onclick={handleCardClick}
		onkeydown={(e) => e.key === "Enter" && handleCardClick()}
		ondragenter={handleDragEnter}
		ondragover={handleDragOver}
		ondragleave={handleDragLeave}
		ondrop={handleDrop}
	>
		<div class="dropzone-inner">
			<svg
				class="upload-icon"
				width="48"
				height="48"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="1.5"
				stroke-linecap="round"
				stroke-linejoin="round"
				aria-hidden="true"
			>
				<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
				<polyline points="17 8 12 3 7 8" />
				<line x1="12" y1="3" x2="12" y2="15" />
			</svg>

			<h1 class="title">Import Squad Export</h1>

			<p class="subtitle">Drop your FM CSV here, or click to browse</p>

			<span class="file-hint">CSV files only</span>
		</div>

		<input
			type="file"
			accept=".csv"
			bind:this={fileInput}
			onchange={handleFileChange}
			class="hidden-input"
			aria-hidden="true"
		/>
	</div>
</div>

<style>
	.page {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 100vh;
		padding: var(--space-4);
	}

	.dropzone-card {
		background: var(--color-glass-bg);
		backdrop-filter: blur(var(--blur-panel));
		-webkit-backdrop-filter: blur(var(--blur-panel));
		border: 1px solid rgba(181, 205, 180, 0.2);
		border-radius: var(--radius-panel);
		padding: var(--space-6) 2.5rem;
		cursor: pointer;
		transition:
			border-color 0.2s ease,
			box-shadow 0.2s ease,
			background 0.2s ease;
	}

	.dropzone-card:hover {
		border-color: var(--color-gold);
		box-shadow:
			0 0 20px var(--color-gold-dim),
			0 0 40px var(--color-gold-glow);
	}

	.dropzone-card:focus {
		outline: 2px solid var(--color-gold);
		outline-offset: 2px;
	}

	.dropzone-card.dragging {
		border-color: var(--color-gold);
		background: var(--color-glass-bg-hover);
		box-shadow:
			0 0 20px var(--color-gold-glow),
			0 0 40px var(--color-gold-dim);
	}

	.dropzone-inner {
		border: 2px dashed var(--color-outline-variant);
		border-radius: var(--radius-md);
		padding: var(--space-stack-lg) 4rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-stack-sm);
		transition: border-color 0.2s ease;
	}

	.dropzone-card:hover .dropzone-inner {
		border-color: rgba(233, 195, 73, 0.6);
	}

	.dropzone-card.dragging .dropzone-inner {
		border-color: var(--color-gold);
	}

	.upload-icon {
		color: var(--color-gold);
		margin-bottom: var(--space-2);
		transition: opacity 0.2s ease;
	}

	.dropzone-card:hover .upload-icon,
	.dropzone-card.dragging .upload-icon {
		opacity: 0.9;
	}

	.title {
		font-size: var(--font-size-headline-md);
		font-weight: var(--font-weight-headline-md);
		color: var(--color-gold);
		margin: 0;
		text-align: center;
	}

	.subtitle {
		font-size: var(--font-size-body-md);
		font-weight: var(--font-weight-body-md);
		color: var(--color-on-surface-variant);
		margin: 0;
		text-align: center;
	}

	.file-hint {
		font-size: var(--font-size-label-caps);
		font-weight: var(--font-weight-label-caps);
		text-transform: uppercase;
		letter-spacing: var(--font-letter-spacing-label-caps);
		color: var(--color-on-surface-variant);
		margin-top: var(--space-1);
	}

	.hidden-input {
		display: none;
	}
</style>
