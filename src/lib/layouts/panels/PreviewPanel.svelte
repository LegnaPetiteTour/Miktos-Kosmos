<script lang="ts">
	import { convertFileSrc } from '@tauri-apps/api/core';
	
	export let selectedFile: any = null;
	
	let imageDimensions: { width: number; height: number } | null = null;
	let videoDimensions: { width: number; height: number } | null = null;
	let imageLoadError = false;
	let videoLoadError = false;
	
	function getFileType(file: any): 'image' | 'video' | 'pdf' | 'audio' | 'document' | 'unknown' {
		if (!file || file.is_dir) return 'unknown';
		const ext = file.name?.split('.').pop()?.toLowerCase() || '';
		
		if (['jpg', 'jpeg', 'png', 'gif', 'webp', 'heic', 'bmp', 'svg'].includes(ext)) return 'image';
		if (['mp4', 'mov', 'avi', 'mkv', 'webm', 'm4v', 'mpg', 'mpeg'].includes(ext)) return 'video';
		if (['pdf'].includes(ext)) return 'pdf';
		if (['mp3', 'wav', 'ogg', 'flac', 'm4a', 'aac'].includes(ext)) return 'audio';
		if (['doc', 'docx', 'txt', 'rtf', 'odt'].includes(ext)) return 'document';
		
		return 'unknown';
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
	
	function formatDimensions(dims: { width: number; height: number } | null): string {
		if (!dims) return 'Loading...';
		return `${dims.width} × ${dims.height} px`;
	}
	
	$: fileType = selectedFile ? getFileType(selectedFile) : 'unknown';
	$: fileUrl = selectedFile && !selectedFile.is_dir ? convertFileSrc(selectedFile.path) : null;
	
	// Reset state when file changes
	$: if (selectedFile) {
		imageDimensions = null;
		videoDimensions = null;
		imageLoadError = false;
		videoLoadError = false;
	}
	
	function handleImageLoad(event: Event) {
		const img = event.target as HTMLImageElement;
		imageDimensions = {
			width: img.naturalWidth,
			height: img.naturalHeight
		};
	}
	
	function handleImageError(event: Event) {
		console.error('Image failed to load:', selectedFile?.path);
		imageLoadError = true;
		imageDimensions = null;
	}
	
	function handleVideoLoad(event: Event) {
		const video = event.target as HTMLVideoElement;
		videoDimensions = {
			width: video.videoWidth,
			height: video.videoHeight
		};
	}
	
	function handleVideoError(event: Event) {
		console.error('Video failed to load:', selectedFile?.path);
		videoLoadError = true;
		videoDimensions = null;
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
	
	.preview-media-container {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-4);
		background-color: var(--bg-subtle);
		overflow: hidden;
	}
	
	.preview-image, .preview-video {
		max-width: 100%;
		max-height: 100%;
		object-fit: contain;
		border-radius: var(--radius-md);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
	}
	
	.preview-video {
		background-color: #000;
	}
	
	.preview-pdf-container {
		flex: 1;
		padding: var(--space-4);
		background-color: var(--bg-subtle);
		overflow: hidden;
	}
	
	.preview-pdf {
		width: 100%;
		height: 100%;
		border: none;
		border-radius: var(--radius-md);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
	}
	
	.preview-audio-container {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: var(--space-4);
		background-color: var(--bg-subtle);
	}
	
	.audio-icon {
		font-size: 64px;
		margin-bottom: var(--space-4);
		opacity: 0.5;
	}
	
	.preview-audio {
		width: 100%;
		max-width: 500px;
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
	
	.unsupported-preview {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: var(--space-4);
		color: var(--text-muted);
		text-align: center;
	}
	
	.unsupported-icon {
		font-size: 48px;
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
	{:else if fileType === 'image'}
		<div class="preview-content">
			<div class="preview-media-container">
				{#if imageLoadError}
					<div style="text-align: center; padding: var(--space-4); color: var(--text-muted);">
						<div style="font-size: 48px; margin-bottom: var(--space-2);">⚠️</div>
						<p>Failed to load image</p>
						<p style="font-size: var(--text-xs); margin-top: var(--space-2); word-break: break-all;">{selectedFile.path}</p>
					</div>
				{:else}
					<img 
						src={fileUrl} 
						alt={selectedFile.name} 
						class="preview-image" 
						on:load={handleImageLoad}
						on:error={handleImageError}
					/>
				{/if}
			</div>
			<div class="preview-info">
				<div class="preview-filename">{selectedFile.name}</div>
				<div class="preview-metadata">
					{#if imageDimensions}
						<div class="metadata-row">
							<span class="metadata-label">Dimensions:</span>
							<span class="metadata-value">{formatDimensions(imageDimensions)}</span>
						</div>
					{/if}
					<div class="metadata-row">
						<span class="metadata-label">Size:</span>
						<span class="metadata-value">{formatSize(selectedFile.size)}</span>
					</div>
					<div class="metadata-row">
						<span class="metadata-label">Modified:</span>
						<span class="metadata-value">{formatDate(selectedFile.modified)}</span>
					</div>
					<div class="metadata-row">
						<span class="metadata-label">Path:</span>
						<span class="metadata-value" style="font-size: var(--text-xs); word-break: break-all;">
							{selectedFile.path}
						</span>
					</div>
				</div>
			</div>
		</div>
	{:else if fileType === 'video'}
		<div class="preview-content">
			<div class="preview-media-container">
				{#if videoLoadError}
					<div style="text-align: center; padding: var(--space-4); color: var(--text-muted);">
						<div style="font-size: 48px; margin-bottom: var(--space-2);">⚠️</div>
						<p>Failed to load video</p>
						<p style="font-size: var(--text-xs); margin-top: var(--space-2); word-break: break-all;">{selectedFile.path}</p>
					</div>
				{:else}
					<video
						src={fileUrl}
						class="preview-video"
						controls
						preload="metadata"
						on:loadedmetadata={handleVideoLoad}
						on:error={handleVideoError}
					>
						<track kind="captions" />
						Your browser does not support the video tag.
					</video>
				{/if}
			</div>
			<div class="preview-info">
				<div class="preview-filename">{selectedFile.name}</div>
				<div class="preview-metadata">
					{#if videoDimensions}
						<div class="metadata-row">
							<span class="metadata-label">Dimensions:</span>
							<span class="metadata-value">{formatDimensions(videoDimensions)}</span>
						</div>
					{/if}
					<div class="metadata-row">
						<span class="metadata-label">Size:</span>
						<span class="metadata-value">{formatSize(selectedFile.size)}</span>
					</div>
					<div class="metadata-row">
						<span class="metadata-label">Modified:</span>
						<span class="metadata-value">{formatDate(selectedFile.modified)}</span>
					</div>
					<div class="metadata-row">
						<span class="metadata-label">Path:</span>
						<span class="metadata-value" style="font-size: var(--text-xs); word-break: break-all;">
							{selectedFile.path}
						</span>
					</div>
				</div>
			</div>
		</div>
	{:else if fileType === 'pdf'}
		<div class="preview-content">
			<div class="preview-pdf-container">
				<iframe
					src={fileUrl}
					class="preview-pdf"
					title={selectedFile.name}
				></iframe>
			</div>
			<div class="preview-info">
				<div class="preview-filename">{selectedFile.name}</div>
				<div class="preview-metadata">
					<div class="metadata-row">
						<span class="metadata-label">Type:</span>
						<span class="metadata-value">PDF Document</span>
					</div>
					<div class="metadata-row">
						<span class="metadata-label">Size:</span>
						<span class="metadata-value">{formatSize(selectedFile.size)}</span>
					</div>
					<div class="metadata-row">
						<span class="metadata-label">Modified:</span>
						<span class="metadata-value">{formatDate(selectedFile.modified)}</span>
					</div>
					<div class="metadata-row">
						<span class="metadata-label">Path:</span>
						<span class="metadata-value" style="font-size: var(--text-xs); word-break: break-all;">
							{selectedFile.path}
						</span>
					</div>
				</div>
			</div>
		</div>
	{:else if fileType === 'audio'}
		<div class="preview-content">
			<div class="preview-audio-container">
				<div class="audio-icon">🎵</div>
				<audio
					src={fileUrl}
					class="preview-audio"
					controls
					preload="metadata"
				>
					Your browser does not support the audio tag.
				</audio>
			</div>
			<div class="preview-info">
				<div class="preview-filename">{selectedFile.name}</div>
				<div class="preview-metadata">
					<div class="metadata-row">
						<span class="metadata-label">Type:</span>
						<span class="metadata-value">Audio File</span>
					</div>
					<div class="metadata-row">
						<span class="metadata-label">Size:</span>
						<span class="metadata-value">{formatSize(selectedFile.size)}</span>
					</div>
					<div class="metadata-row">
						<span class="metadata-label">Modified:</span>
						<span class="metadata-value">{formatDate(selectedFile.modified)}</span>
					</div>
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
		<div class="unsupported-preview">
			<div class="unsupported-icon">📄</div>
			<p style="margin-bottom: var(--space-3);">Preview not available for this file type</p>
			<div class="preview-info" style="width: 100%; max-width: 500px;">
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
					<div class="metadata-row">
						<span class="metadata-label">Path:</span>
						<span class="metadata-value" style="font-size: var(--text-xs); word-break: break-all;">
							{selectedFile.path}
						</span>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>
