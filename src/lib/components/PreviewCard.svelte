<script lang="ts">
	import { convertFileSrc } from '@tauri-apps/api/core';
	
	export let file: any;
	export let onClick: () => void;
	
	let imageLoadError = false;
	
	// Determine file type
	$: ext = file.path?.split('.').pop()?.toLowerCase() || '';
	$: isImage = ['jpg', 'jpeg', 'png', 'gif', 'webp', 'heic', 'bmp', 'svg'].includes(ext);
	$: isVideo = ['mp4', 'mov', 'avi', 'mkv', 'webm'].includes(ext);
	
	// Convert file path to asset URL for Tauri
	$: imageUrl = isImage && file.path ? convertFileSrc(file.path) : null;
	
	// Debug logging
	$: if (imageUrl) {
		console.log('Image URL:', imageUrl);
		console.log('File path:', file.path);
		imageLoadError = false; // Reset error state when URL changes
	}
	
	function handleImageError(event: Event) {
		console.error('Image failed to load:', imageUrl);
		console.error('File:', file);
		imageLoadError = true;
	}
	
	// Determine file type icon for non-images
	$: fileIcon = (() => {
		if (isVideo) return '🎥';
		if (['pdf'].includes(ext)) return '📄';
		if (['txt', 'md', 'json'].includes(ext)) return '📝';
		if (['doc', 'docx'].includes(ext)) return '📃';
		if (['xls', 'xlsx'].includes(ext)) return '📊';
		if (['ppt', 'pptx'].includes(ext)) return '📽️';
		return '📎';
	})();
	
	// Format file size
	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 B';
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		if (bytes < 1024 * 1024 * 1024) return (bytes / 1024 / 1024).toFixed(1) + ' MB';
		return (bytes / 1024 / 1024 / 1024).toFixed(2) + ' GB';
	}
	
	// Format date
	function formatDate(dateStr: string | undefined): string {
		if (!dateStr) return 'No date';
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
	}
	
	$: fileName = file.path?.split('/').pop() || 'Unknown';
	$: fileExt = file.path?.split('.').pop()?.toUpperCase() || '';
	$: metadata = file.date_taken || file.modified_at || file.created_at;
</script>

<style>
	.preview-card {
		width: 240px;
		flex-shrink: 0;
		background: var(--bg);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: var(--radius-lg);
		padding: var(--space-4);
		cursor: pointer;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		position: relative;
		overflow: hidden;
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08),
		            inset 0 1px 1px rgba(255, 255, 255, 0.1);
	}
	
	.preview-card::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(
			135deg,
			rgba(255, 255, 255, 0.1) 0%,
			rgba(255, 255, 255, 0.05) 100%
		);
		opacity: 0;
		transition: opacity 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		border-radius: var(--radius-lg);
	}
	
	.preview-card:hover {
		border-color: rgba(255, 255, 255, 0.2);
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12),
		            inset 0 1px 1px rgba(255, 255, 255, 0.15);
		transform: translateY(-4px) scale(1.02);
	}
	
	.preview-card:hover::before {
		opacity: 1;
	}
	
	.preview-thumbnail {
		width: 100%;
		height: 120px;
		background: var(--bg-subtle);
		border-radius: var(--radius-md);
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 48px;
		overflow: hidden;
		position: relative;
	}
	
	.preview-thumbnail img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}
	
	.preview-thumbnail-icon {
		font-size: 48px;
	}
	
	.preview-info {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}
	
	.preview-name {
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		color: var(--text);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
	
	.preview-meta {
		font-size: var(--text-xs);
		color: var(--text-subtle);
		display: flex;
		flex-direction: column;
		gap: 2px;
	}
	
	.preview-type {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		font-size: var(--text-xs);
		font-weight: var(--weight-semibold);
		color: var(--text-muted);
		background: var(--bg-subtle);
		padding: 2px 6px;
		border-radius: 4px;
		width: fit-content;
	}
</style>

<button type="button" class="preview-card" on:click={onClick}>
	<div class="preview-thumbnail">
		{#if imageUrl && !imageLoadError}
			<img src={imageUrl} alt={fileName} loading="lazy" on:error={handleImageError} />
		{:else}
			<div class="preview-thumbnail-icon">{fileIcon}</div>
		{/if}
	</div>
	
	<div class="preview-info">
		<div class="preview-name" title={fileName}>{fileName}</div>
		<div class="preview-type">{fileExt}</div>
		<div class="preview-meta">
			<div>{formatDate(metadata)}</div>
			<div>{formatBytes(file.size || 0)}</div>
		</div>
	</div>
</button>
