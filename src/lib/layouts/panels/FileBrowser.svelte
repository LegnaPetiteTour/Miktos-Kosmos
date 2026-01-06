<script lang="ts">
	import { fileStore } from '$lib/stores/photoStore';
	
	let scanResult: any = null;
	
	fileStore.subscribe(value => {
		scanResult = value;
	});
	
	$: files = scanResult?.files || [];
</script>

<style>
	.file-browser {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		height: 100%;
	}
	
	.file-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
		gap: var(--space-3);
		overflow-y: auto;
	}
	
	.file-item {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: var(--space-3);
		border: 1px solid var(--border);
		border-radius: var(--radius-md);
		background-color: var(--bg-subtle);
		cursor: pointer;
		transition: all var(--transition-fast);
	}
	
	.file-item:hover {
		background-color: var(--panel);
		border-color: var(--accent);
		transform: translateY(-2px);
		box-shadow: var(--shadow-md);
	}
	
	.file-icon {
		font-size: 48px;
		margin-bottom: var(--space-2);
		opacity: 0.6;
	}
	
	.file-name {
		font-size: var(--text-xs);
		color: var(--text);
		text-align: center;
		word-break: break-word;
		max-width: 100%;
		overflow: hidden;
		text-overflow: ellipsis;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
	}
	
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: var(--text-muted);
	}
	
	.empty-icon {
		font-size: 64px;
		margin-bottom: var(--space-3);
		opacity: 0.3;
	}
</style>

<div class="file-browser">
	{#if files.length === 0}
		<div class="empty-state">
			<div class="empty-icon">📁</div>
			<p>No files loaded</p>
			<p style="font-size: var(--text-sm); margin-top: var(--space-2);">
				Scan a folder to begin
			</p>
		</div>
	{:else}
		<div class="file-grid">
			{#each files as file}
				<div class="file-item" title={file.file_name}>
					<div class="file-icon">
						{#if file.file_type === 'Image'}
							🖼️
						{:else if file.file_type === 'Video'}
							🎬
						{:else if file.file_type === 'Document'}
							📄
						{:else if file.file_type === 'Audio'}
							🎵
						{:else}
							📎
						{/if}
					</div>
					<div class="file-name">{file.file_name}</div>
				</div>
			{/each}
		</div>
	{/if}
</div>
