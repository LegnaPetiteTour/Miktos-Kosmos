<script lang="ts">
	import { convertFileSrc } from '@tauri-apps/api/core';
	
	export let selectedFile: any = null;
	
	function isImageFile(file: any): boolean {
		if (!file || file.is_dir) return false;
		const ext = file.name?.split('.').pop()?.toLowerCase() || '';
		return ['jpg', 'jpeg', 'png', 'gif', 'webp', 'heic', 'bmp', 'svg'].includes(ext);
	}
	
	function formatSize(bytes: number): string {
		if (!bytes || bytes === 0) return '0 B';
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(0) + ' KB';
		if (bytes < 1024 * 1024 * 1024) return (bytes / 1024 / 1024).toFixed(1) + ' MB';
		return (bytes / 1024 / 1024 / 1024).toFixed(2) + ' GB';
	}
	
	function formatDate(timestamp: number | string | undefined): string {
		if (!timestamp) return 'Unknown';
		
		try {
			let date: Date;
			if (typeof timestamp === 'number') {
				date = timestamp > 10000000000 
					? new Date(timestamp) 
					: new Date(timestamp * 1000);
			} else {
				date = new Date(timestamp);
			}
			
			if (isNaN(date.getTime())) return 'Unknown';
			
			return date.toLocaleDateString('en-US', {
				year: 'numeric',
				month: 'long',
				day: 'numeric',
				hour: 'numeric',
				minute: '2-digit',
				hour12: true
			});
		} catch {
			return 'Unknown';
		}
	}
	
	$: imageUrl = selectedFile && isImageFile(selectedFile) 
		? convertFileSrc(selectedFile.path) 
		: null;
	
	$: if (selectedFile) {
		console.log('PreviewPanel - selectedFile changed:', {
			fileName: selectedFile.name,
			filePath: selectedFile.path,
			isImage: isImageFile(selectedFile),
			imageUrl: imageUrl
		});
	}
	
	let imageLoadError = false;
	
	function handleImageError(event: Event) {
		console.error('PreviewPanel - Image failed to load:', {
			imageUrl: imageUrl,
			filePath: selectedFile?.path
		});
		imageLoadError = true;
	}
	
	$: if (imageUrl) {
		imageLoadError = false;
	}
</script>

<style>
	.preview-panel {
		display: flex;
		flex-direction: column;
		height: 100%;
		background-color: var(--panel);
		overflow: hidden;
	}
	
	.preview-empty {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--text-muted);
		text-align: center;
	}
	
	.preview-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		overflow: hidden;
	}
	
	.preview-image-container {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-4);
		background-color: var(--bg-subtle);
		overflow: hidden;
	}
	
	.preview-image {
		max-width: 100%;
		max-height: 100%;
		object-fit: contain;
		border-radius: var(--radius-md);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
	}
	
	.preview-info {
		padding: var(--space-4);
		border-top: 1px solid var(--panel-border);
		background-color: var(--bg);
	}
	
	.preview-filename {
		font-size: var(--text-base);
		font-weight: var(--weight-semibold);
		color: var(--text);
		margin-bottom: var(--space-3);
		word-break: break-word;
	}
	
	.preview-metadata {
		display: grid;
		gap: var(--space-2);
	}
	
	.metadata-row {
		display: flex;
		gap: var(--space-2);
		font-size: var(--text-sm);
	}
	
	.metadata-label {
		color: var(--text-muted);
		min-width: 100px;
	}
	
	.metadata-value {
		color: var(--text);
		font-weight: var(--weight-medium);
	}
	
	.preview-folder {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		color: var(--text-muted);
		text-align: center;
		padding: var(--space-4);
	}
	
	.folder-icon {
		font-size: 64px;
		margin-bottom: var(--space-3);
		opacity: 0.5;
	}
</style>

<div class="preview-panel">
	{#if !selectedFile}
		<div class="preview-empty">
			<div>
				<div style="font-size: 48px; margin-bottom: var(--space-2); opacity: 0.3;">👁️</div>
				<p>Select a file to preview</p>
			</div>
		</div>
	{:else if selectedFile.is_dir}
		<div class="preview-folder">
			<div class="folder-icon">📁</div>
			<div class="preview-filename">{selectedFile.name}</div>
			<div class="metadata-row">
				<span class="metadata-label">Type:</span>
				<span class="metadata-value">Folder</span>
			</div>
			<div class="metadata-row">
				<span class="metadata-label">Modified:</span>
				<span class="metadata-value">{formatDate(selectedFile.modified || selectedFile.created)}</span>
			</div>
		</div>
	{:else if imageUrl}
		<div class="preview-content">
			<div class="preview-image-container">
				{#if imageLoadError}
					<div style="text-align: center; padding: var(--space-4); color: var(--text-muted);">
						<div style="font-size: 48px; margin-bottom: var(--space-2);">⚠️</div>
						<p>Failed to load image</p>
						<p style="font-size: var(--text-xs); margin-top: var(--space-2); word-break: break-all;">{selectedFile.path}</p>
					</div>
				{:else}
					<img 
						src={imageUrl} 
						alt={selectedFile.name} 
						class="preview-image" 
						on:error={handleImageError}
					/>
				{/if}
			</div>
			<div class="preview-info">
				<div class="preview-filename">{selectedFile.name}</div>
				<div class="preview-metadata">
					<div class="metadata-row">
						<span class="metadata-label">Size:</span>
						<span class="metadata-value">{formatSize(selectedFile.size)}</span>
					</div>
					<div class="metadata-row">
						<span class="metadata-label">Modified:</span>
						<span class="metadata-value">{formatDate(selectedFile.modified)}</span>
					</div>
					{#if selectedFile.created && selectedFile.created !== selectedFile.modified}
						<div class="metadata-row">
							<span class="metadata-label">Created:</span>
							<span class="metadata-value">{formatDate(selectedFile.created)}</span>
						</div>
					{/if}
					<div class="metadata-row">
						<span class="metadata-label">Path:</span>
						<span class="metadata-value" style="font-size: var(--text-xs); word-break: break-all;">
							{selectedFile.path}
						</span>
					</div>
				</div>
			</div>
		</div>
	{:else}
		<div class="preview-info" style="flex: 1;">
			<div class="preview-filename">{selectedFile.name}</div>
			<div class="preview-metadata">
				<div class="metadata-row">
					<span class="metadata-label">Type:</span>
					<span class="metadata-value">{selectedFile.name.split('.').pop()?.toUpperCase() || 'FILE'}</span>
				</div>
				<div class="metadata-row">
					<span class="metadata-label">Size:</span>
					<span class="metadata-value">{formatSize(selectedFile.size)}</span>
				</div>
				<div class="metadata-row">
					<span class="metadata-label">Modified:</span>
					<span class="metadata-value">{formatDate(selectedFile.modified)}</span>
				</div>
				{#if selectedFile.created && selectedFile.created !== selectedFile.modified}
					<div class="metadata-row">
						<span class="metadata-label">Created:</span>
						<span class="metadata-value">{formatDate(selectedFile.created)}</span>
					</div>
				{/if}
				<div class="metadata-row">
					<span class="metadata-label">Path:</span>
					<span class="metadata-value" style="font-size: var(--text-xs); word-break: break-all;">
						{selectedFile.path}
					</span>
				</div>
			</div>
		</div>
	{/if}
</div>
