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
		padding: 1rem;
	}

	.dropzone-card {
		background: rgba(13, 33, 17, 0.7);
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
		border: 1px solid rgba(181, 205, 180, 0.2);
		border-radius: 20px;
		padding: 2rem 2.5rem;
		cursor: pointer;
		transition:
			border-color 0.2s ease,
			box-shadow 0.2s ease,
			background 0.2s ease;
	}

	.dropzone-card:hover {
		border-color: #e9c349;
		box-shadow:
			0 0 20px rgba(233, 195, 73, 0.15),
			0 0 40px rgba(233, 195, 73, 0.08);
	}

	.dropzone-card:focus {
		outline: 2px solid #e9c349;
		outline-offset: 2px;
	}

	.dropzone-card.dragging {
		border-color: #e9c349;
		background: rgba(13, 33, 17, 0.85);
		box-shadow:
			0 0 20px rgba(233, 195, 73, 0.2),
			0 0 40px rgba(233, 195, 73, 0.1);
	}

	.dropzone-inner {
		border: 2px dashed #434842;
		border-radius: 8px;
		padding: 3rem 4rem;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		transition: border-color 0.2s ease;
	}

	.dropzone-card:hover .dropzone-inner {
		border-color: rgba(233, 195, 73, 0.6);
	}

	.dropzone-card.dragging .dropzone-inner {
		border-color: #e9c349;
	}

	.upload-icon {
		color: #e9c349;
		margin-bottom: 0.5rem;
		transition: opacity 0.2s ease;
	}

	.dropzone-card:hover .upload-icon,
	.dropzone-card.dragging .upload-icon {
		opacity: 0.9;
	}

	.title {
		font-size: 24px;
		font-weight: 600;
		color: #e9c349;
		margin: 0;
		text-align: center;
	}

	.subtitle {
		font-size: 16px;
		font-weight: 400;
		color: #c3c8c0;
		margin: 0;
		text-align: center;
	}

	.file-hint {
		font-size: 12px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.1em;
		color: #c3c8c0;
		margin-top: 0.25rem;
	}

	.hidden-input {
		display: none;
	}
</style>
