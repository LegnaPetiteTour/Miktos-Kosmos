<script lang="ts">
	import { createEventDispatcher, onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import { invoke } from '@tauri-apps/api/core';
	import { convertFileSrc } from '@tauri-apps/api/core';
	import { currentFolderStore } from '$lib/stores/currentFolderStore';
	
	export let currentPath: string = '';
	export let files: any[] = [];
	export let loading: boolean = false;
	
	const dispatch = createEventDispatcher();
	
	let selectedIndex: number = -1;
	let tableElement: HTMLDivElement;
	
	// View controls
	type ViewMode = 'grid' | 'list' | 'details' | 'thumbnails';
	let viewMode: ViewMode = 'details';
	let itemSize: number = 50; // Size in pixels (for thumbnails/grid)
	
	// Extract folder name from path
	$: folderName = currentPath.split('/').filter(Boolean).pop() || 'Computer';
	
	// Sort files: folders first, then by name
	$: sortedFiles = [...files].sort((a, b) => {
		// Folders first
		if (a.is_dir && !b.is_dir) return -1;
		if (!a.is_dir && b.is_dir) return 1;
		// Then alphabetically
		return a.name.localeCompare(b.name);
	});
	
	// Reset selection when folder changes
	$: if (currentPath) {
		selectedIndex = -1;
	}
	
	// Preview file when selection changes
	$: if (selectedIndex >= 0 && selectedIndex < sortedFiles.length) {
		const file = sortedFiles[selectedIndex];
		if (!file.is_dir) {
			// Dispatch preview event for files
			dispatch('fileSelect', file);
		}
	}
	
	// Check if file is an image
	function isImageFile(file: any): boolean {
		if (file.is_dir) return false;
		const ext = file.name?.split('.').pop()?.toLowerCase() || '';
		return ['jpg', 'jpeg', 'png', 'gif', 'webp', 'heic', 'bmp', 'svg'].includes(ext);
	}
	
	// Get thumbnail URL for image files
	function getThumbnailUrl(file: any): string | null {
		if (!isImageFile(file)) return null;
		return convertFileSrc(file.path);
	}
	
	function handleKeyDown(event: KeyboardEvent) {
		if (sortedFiles.length === 0) return;
		
		switch(event.key) {
			case 'ArrowDown':
				event.preventDefault();
				if (selectedIndex < sortedFiles.length - 1) {
					selectedIndex++;
					scrollToSelected();
				} else if (selectedIndex === -1 && sortedFiles.length > 0) {
					selectedIndex = 0;
					scrollToSelected();
				}
				break;
				
			case 'ArrowUp':
				event.preventDefault();
				if (selectedIndex > 0) {
					selectedIndex--;
					scrollToSelected();
				}
				break;
				
			case 'Enter':
				event.preventDefault();
				if (selectedIndex >= 0 && selectedIndex < sortedFiles.length) {
					openFile(sortedFiles[selectedIndex]);
				}
				break;
				
			case 'Home':
				event.preventDefault();
				if (sortedFiles.length > 0) {
					selectedIndex = 0;
					scrollToSelected();
				}
				break;
				
			case 'End':
				event.preventDefault();
				if (sortedFiles.length > 0) {
					selectedIndex = sortedFiles.length - 1;
					scrollToSelected();
				}
				break;
				
			case 'Backspace':
				event.preventDefault();
				goUpOneLevel();
				break;
		}
	}
	
	function scrollToSelected() {
		if (!tableElement) return;
		
		const rows = tableElement.querySelectorAll('tbody tr');
		if (selectedIndex >= 0 && selectedIndex < rows.length) {
			const row = rows[selectedIndex] as HTMLElement;
			row.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
		}
	}
	
	async function openFile(file: any) {
		if (file.is_dir) {
			// Navigate into folder
			currentFolderStore.setFolder(file.path, file.name);
			try {
				const files = await invoke('list_directory', { path: file.path }) as any[];
				currentFolderStore.setFiles(files);
			} catch (error) {
				console.error('Failed to load folder contents:', error);
				currentFolderStore.setFiles([]);
			}
		} else {
			// Already previewed via reactive statement, Enter just confirms
			dispatch('fileSelect', file);
		}
	}
	
	async function goUpOneLevel() {
		if (!currentPath) return;
		
		const pathParts = currentPath.split('/').filter(Boolean);
		if (pathParts.length === 0) return;
		
		pathParts.pop(); // Remove last segment
		const parentPath = '/' + pathParts.join('/');
		const parentName = pathParts[pathParts.length - 1] || 'Computer';
		
		currentFolderStore.setFolder(parentPath, parentName);
		try {
			const files = await invoke('list_directory', { path: parentPath }) as any[];
			currentFolderStore.setFiles(files);
		} catch (error) {
			console.error('Failed to load parent folder:', error);
		}
	}
	
	function selectFile(file: any, index: number) {
		selectedIndex = index;
		// Preview happens automatically via reactive statement
		// Enter opens folders
		if (file.is_dir) {
			openFile(file);
		}
	}
	
	function formatSize(bytes: number): string {
		if (!bytes || bytes === 0) return '0 B';
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(0) + ' KB';
		if (bytes < 1024 * 1024 * 1024) return (bytes / 1024 / 1024).toFixed(1) + ' MB';
		return (bytes / 1024 / 1024 / 1024).toFixed(2) + ' GB';
	}
	
	function formatDate(timestamp: number | string | undefined): string {
		if (!timestamp) return '--';
		
		try {
			let date: Date;
			if (typeof timestamp === 'number') {
				// Unix timestamp in seconds or milliseconds
				date = timestamp > 10000000000 
					? new Date(timestamp) 
					: new Date(timestamp * 1000);
			} else {
				date = new Date(timestamp);
			}
			
			if (isNaN(date.getTime())) return '--';
			
			return date.toLocaleDateString('en-US', {
				year: 'numeric',
				month: 'short',
				day: 'numeric'
			}) + ' at ' + date.toLocaleTimeString('en-US', {
				hour: 'numeric',
				minute: '2-digit',
				hour12: true
			});
		} catch {
			return '--';
		}
	}
	
	function getFileType(file: any): string {
		if (file.is_dir) return 'Folder';
		
		const fileName = file.name || '';
		const ext = fileName.split('.').pop()?.toLowerCase();
		
		if (!ext || ext === fileName.toLowerCase()) return 'File';
		
		const typeMap: Record<string, string> = {
			'jpg': 'JPEG image', 'jpeg': 'JPEG image', 'png': 'PNG image',
			'gif': 'GIF image', 'webp': 'WebP image', 'heic': 'HEIC image',
			'mp4': 'MP4 video', 'mov': 'MOV video', 'avi': 'AVI video',
			'mp3': 'MP3 audio', 'wav': 'WAV audio', 'pdf': 'PDF document',
			'doc': 'Word document', 'docx': 'Word document', 'txt': 'Text file',
			'zip': 'ZIP archive', 'rar': 'RAR archive',
		};
		
		return typeMap[ext] || ext.toUpperCase() + ' file';
	}
	
	function getFileIcon(file: any): string {
		if (file.is_dir) return '📁';
		
		const fileName = file.name || '';
		const ext = fileName.split('.').pop()?.toLowerCase();
		
		const iconMap: Record<string, string> = {
			'jpg': '🖼️', 'jpeg': '🖼️', 'png': '🖼️', 'gif': '🖼️',
			'mp4': '🎬', 'mov': '🎬', 'avi': '🎬',
			'pdf': '📄', 'doc': '📝', 'docx': '📝', 'txt': '📝',
			'zip': '📦', 'rar': '📦',
			'mp3': '🎵', 'wav': '🎵',
		};
		
		return iconMap[ext || ''] || '📄';
	}
	
	// Only run in browser (not during SSR)
	onMount(() => {
		if (browser) {
			window.addEventListener('keydown', handleKeyDown);
		}
	});
	
	onDestroy(() => {
		if (browser) {
			window.removeEventListener('keydown', handleKeyDown);
		}
	});
</script>

<style>
	.content-panel {
		display: flex;
		flex-direction: column;
		height: 100%;
		background-color: var(--panel);
		overflow: hidden;
	}
	
	.content-header {
		padding: var(--space-3) var(--space-4);
		border-bottom: 1px solid var(--panel-border);
		background-color: var(--bg-subtle);
		flex-shrink: 0;
	}
	
	.content-title {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: var(--text);
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}
	
	.content-title-label {
		color: var(--text-muted);
	}
	
	.file-count {
		color: var(--text-muted);
		font-size: var(--text-xs);
		font-weight: normal;
		margin-left: var(--space-2);
	}
	
	.content-body {
		flex: 1;
		overflow: hidden;
		display: flex;
		flex-direction: column;
	}
	
	.content-table-wrapper {
		flex: 1;
		overflow-y: auto;
		overflow-x: auto;
	}
	
	.content-table {
		width: 100%;
		border-collapse: collapse;
	}
	
	.content-table thead {
		position: sticky;
		top: 0;
		background-color: var(--bg-subtle);
		z-index: 10;
	}
	
	.content-table th {
		padding: var(--space-2) var(--space-3);
		text-align: left;
		font-size: var(--text-xs);
		font-weight: var(--weight-semibold);
		color: var(--text-muted);
		border-bottom: 1px solid var(--panel-border);
		white-space: nowrap;
	}
	
	.content-table tbody tr {
		border-bottom: 1px solid rgba(255, 255, 255, 0.03);
		cursor: pointer;
		transition: background-color var(--transition-fast);
	}
	
	.content-table tbody tr:hover {
		background-color: var(--bg-subtle);
	}
	
	.content-table tbody tr.selected {
		background-color: var(--nav-active-bg);
		border-left: 2px solid var(--accent);
	}
	
	.content-table td {
		padding: var(--space-2) var(--space-3);
		font-size: var(--text-xs);
		color: var(--text);
	}
	
	.file-name {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}
	
	.file-thumbnail {
		width: 32px;
		height: 32px;
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		overflow: hidden;
		border-radius: 4px;
		background-color: var(--bg-subtle);
	}
	
	.file-thumbnail img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}
	
	.file-icon {
		font-size: 20px;
		flex-shrink: 0;
	}
	
	.empty-state, .loading-state {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-5);
		text-align: center;
		color: var(--text-muted);
		font-size: var(--text-sm);
	}
	
	.keyboard-hint {
		padding: var(--space-2) var(--space-3);
		background-color: var(--bg-subtle);
		border-top: 1px solid var(--panel-border);
		font-size: var(--text-xs);
		color: var(--text-muted);
		text-align: center;
	}
	
	.view-controls {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-2) var(--space-3);
		background: var(--bg);
		border-top: 1px solid var(--border);
		gap: var(--space-3);
	}

	.view-modes {
		display: flex;
		gap: var(--space-1);
	}

	.view-mode-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		padding: 0;
		background: transparent;
		border: 1px solid transparent;
		border-radius: 4px;
		color: var(--text-muted);
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.view-mode-btn:hover {
		background: var(--bg-subtle);
		color: var(--text);
	}

	.view-mode-btn.active {
		background: var(--accent);
		color: white;
		border-color: var(--accent);
	}

	.size-control {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}

	.size-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		padding: 0;
		background: transparent;
		border: 1px solid var(--border);
		border-radius: 4px;
		color: var(--text-muted);
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.size-btn:hover {
		background: var(--bg-subtle);
		color: var(--text);
		border-color: var(--text-muted);
	}

	.size-slider {
		width: 100px;
		height: 4px;
		-webkit-appearance: none;
		appearance: none;
		background: var(--border);
		border-radius: 2px;
		outline: none;
	}

	.size-slider::-webkit-slider-thumb {
		-webkit-appearance: none;
		appearance: none;
		width: 14px;
		height: 14px;
		background: var(--accent);
		border-radius: 50%;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.size-slider::-webkit-slider-thumb:hover {
		transform: scale(1.2);
	}

	.size-slider::-moz-range-thumb {
		width: 14px;
		height: 14px;
		background: var(--accent);
		border: none;
		border-radius: 50%;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.size-slider::-moz-range-thumb:hover {
		transform: scale(1.2);
	}

	/* List View Styles */
	.list-view {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-2);
	}

	.list-item {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-2) var(--space-3);
		border-radius: 6px;
		cursor: pointer;
		transition: background 0.15s ease;
	}

	.list-item:hover {
		background: var(--bg-subtle);
	}

	.list-item.selected {
		background: var(--accent);
		color: white;
	}

	.list-thumbnail, .list-icon {
		width: 40px;
		height: 40px;
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 6px;
		overflow: hidden;
		background: var(--bg-subtle);
	}

	.list-thumbnail img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.list-icon {
		font-size: 24px;
	}

	.list-content {
		flex: 1;
		min-width: 0;
	}

	.list-name {
		font-weight: 500;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.list-meta {
		font-size: var(--text-xs);
		color: var(--text-muted);
		margin-top: 2px;
	}

	.list-item.selected .list-meta {
		color: rgba(255, 255, 255, 0.8);
	}

	/* Grid View Styles */
	.grid-view {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-3);
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
		gap: var(--space-3);
		align-content: start;
	}

	.grid-item {
		display: flex;
		flex-direction: column;
		cursor: pointer;
		border-radius: 8px;
		overflow: hidden;
		transition: transform 0.15s ease, box-shadow 0.15s ease;
	}

	.grid-item:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
	}

	.grid-item.selected {
		box-shadow: 0 0 0 3px var(--accent);
	}

	.grid-preview {
		width: 100%;
		background: var(--bg-subtle);
		display: flex;
		align-items: center;
		justify-content: center;
		overflow: hidden;
	}

	.grid-preview img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.grid-icon {
		color: var(--text-muted);
	}

	.grid-name {
		padding: var(--space-2);
		font-size: var(--text-xs);
		text-align: center;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		background: var(--panel);
	}

	.grid-item.selected .grid-name {
		background: var(--accent);
		color: white;
	}

	/* Thumbnails View Styles */
	.thumbnails-view {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-3);
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(50px, 1fr));
		gap: var(--space-2);
		align-content: start;
	}

	.thumbnail-item {
		display: flex;
		flex-direction: column;
		cursor: pointer;
		border-radius: 6px;
		overflow: hidden;
		transition: transform 0.15s ease;
	}

	.thumbnail-item:hover {
		transform: scale(1.05);
	}

	.thumbnail-item.selected {
		box-shadow: 0 0 0 2px var(--accent);
	}

	.thumbnail-preview {
		width: 100%;
		background: var(--bg-subtle);
		display: flex;
		align-items: center;
		justify-content: center;
		overflow: hidden;
	}

	.thumbnail-preview img {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.thumbnail-icon {
		color: var(--text-muted);
	}

	.thumbnail-name {
		padding: var(--space-1);
		font-size: 10px;
		text-align: center;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		background: var(--panel);
	}

	.thumbnail-item.selected .thumbnail-name {
		background: var(--accent);
		color: white;
	}
</style>

<div class="content-panel">
	<div class="content-body">
		{#if loading}
			<div class="loading-state">
				Loading...
			</div>
		{:else if !currentPath}
			<div class="empty-state">
				Select a folder to view its contents
			</div>
		{:else if files.length === 0}
			<div class="empty-state">
				This folder is empty
			</div>
		{:else}
			{#if viewMode === 'details'}
				<div class="content-table-wrapper" bind:this={tableElement}>
					<table class="content-table">
						<thead>
							<tr>
								<th style="width: 40%;">Name</th>
								<th style="width: 25%;">Date Modified</th>
								<th style="width: 15%;">Size</th>
								<th style="width: 20%;">Type</th>
							</tr>
						</thead>
						<tbody>
							{#each sortedFiles as file, index}
								<tr 
									class:selected={index === selectedIndex}
									on:click={() => selectFile(file, index)}
								>
									<td>
										<div class="file-name">
											{#if isImageFile(file)}
												<div class="file-thumbnail">
													<img src={getThumbnailUrl(file)} alt={file.name} loading="lazy" />
												</div>
											{:else}
												<span class="file-icon">{getFileIcon(file)}</span>
											{/if}
											<span>{file.name}</span>
										</div>
									</td>
									<td>{formatDate(file.modified || file.created)}</td>
									<td>{file.is_dir ? '--' : formatSize(file.size)}</td>
									<td>{getFileType(file)}</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			{:else if viewMode === 'list'}
				<div class="list-view" bind:this={tableElement}>
					{#each sortedFiles as file, index}
						<div 
							class="list-item"
							class:selected={index === selectedIndex}
							on:click={() => selectFile(file, index)}
						>
							{#if isImageFile(file)}
								<div class="list-thumbnail">
									<img src={getThumbnailUrl(file)} alt={file.name} loading="lazy" />
								</div>
							{:else}
								<span class="list-icon">{getFileIcon(file)}</span>
							{/if}
							<div class="list-content">
								<div class="list-name">{file.name}</div>
								<div class="list-meta">
									{formatDate(file.modified || file.created)} • {file.is_dir ? 'Folder' : formatSize(file.size)}
								</div>
							</div>
						</div>
					{/each}
				</div>
			{:else if viewMode === 'grid'}
				<div class="grid-view" bind:this={tableElement}>
					{#each sortedFiles as file, index}
						<div 
							class="grid-item"
							class:selected={index === selectedIndex}
							on:click={() => selectFile(file, index)}
							style="width: {itemSize * 2}px;"
						>
							<div class="grid-preview" style="height: {itemSize * 2}px;">
								{#if isImageFile(file)}
									<img src={getThumbnailUrl(file)} alt={file.name} loading="lazy" />
								{:else}
									<span class="grid-icon" style="font-size: {itemSize}px;">{getFileIcon(file)}</span>
								{/if}
							</div>
							<div class="grid-name">{file.name}</div>
						</div>
					{/each}
				</div>
			{:else if viewMode === 'thumbnails'}
				<div class="thumbnails-view" bind:this={tableElement}>
					{#each sortedFiles as file, index}
						<div 
							class="thumbnail-item"
							class:selected={index === selectedIndex}
							on:click={() => selectFile(file, index)}
							style="width: {itemSize}px;"
						>
							<div class="thumbnail-preview" style="height: {itemSize}px;">
								{#if isImageFile(file)}
									<img src={getThumbnailUrl(file)} alt={file.name} loading="lazy" />
								{:else}
									<span class="thumbnail-icon" style="font-size: {itemSize * 0.5}px;">{getFileIcon(file)}</span>
								{/if}
							</div>
							<div class="thumbnail-name">{file.name}</div>
						</div>
					{/each}
				</div>
			{/if}
			<div class="view-controls">
				<div class="view-modes">
					<button 
						class="view-mode-btn" 
						class:active={viewMode === 'grid'}
						on:click={() => viewMode = 'grid'}
						title="Grid view"
					>
						<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
							<rect x="1" y="1" width="6" height="6" rx="1"/>
							<rect x="9" y="1" width="6" height="6" rx="1"/>
							<rect x="1" y="9" width="6" height="6" rx="1"/>
							<rect x="9" y="9" width="6" height="6" rx="1"/>
						</svg>
					</button>
					<button 
						class="view-mode-btn" 
						class:active={viewMode === 'thumbnails'}
						on:click={() => viewMode = 'thumbnails'}
						title="Thumbnails view"
					>
						<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
							<rect x="1" y="1" width="4" height="4" rx="0.5"/>
							<rect x="6" y="1" width="4" height="4" rx="0.5"/>
							<rect x="11" y="1" width="4" height="4" rx="0.5"/>
							<rect x="1" y="6" width="4" height="4" rx="0.5"/>
							<rect x="6" y="6" width="4" height="4" rx="0.5"/>
							<rect x="11" y="6" width="4" height="4" rx="0.5"/>
							<rect x="1" y="11" width="4" height="4" rx="0.5"/>
							<rect x="6" y="11" width="4" height="4" rx="0.5"/>
							<rect x="11" y="11" width="4" height="4" rx="0.5"/>
						</svg>
					</button>
					<button 
						class="view-mode-btn" 
						class:active={viewMode === 'details'}
						on:click={() => viewMode = 'details'}
						title="Details view"
					>
						<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
							<rect x="1" y="2" width="14" height="2" rx="1"/>
							<rect x="1" y="7" width="14" height="2" rx="1"/>
							<rect x="1" y="12" width="14" height="2" rx="1"/>
						</svg>
					</button>
					<button 
						class="view-mode-btn" 
						class:active={viewMode === 'list'}
						on:click={() => viewMode = 'list'}
						title="List view"
					>
						<svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
							<rect x="1" y="2" width="2" height="2" rx="0.5"/>
							<rect x="5" y="2" width="10" height="2" rx="1"/>
							<rect x="1" y="7" width="2" height="2" rx="0.5"/>
							<rect x="5" y="7" width="10" height="2" rx="1"/>
							<rect x="1" y="12" width="2" height="2" rx="0.5"/>
							<rect x="5" y="12" width="10" height="2" rx="1"/>
						</svg>
					</button>
				</div>
				
				<div class="size-control">
					<button class="size-btn" on:click={() => itemSize = Math.max(30, itemSize - 10)}>
						<svg width="14" height="14" viewBox="0 0 14 14" fill="currentColor">
							<path d="M2 7h10" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
						</svg>
					</button>
					<input 
						type="range" 
						min="30" 
						max="200" 
						bind:value={itemSize}
						class="size-slider"
					/>
					<button class="size-btn" on:click={() => itemSize = Math.min(200, itemSize + 10)}>
						<svg width="14" height="14" viewBox="0 0 14 14" fill="currentColor">
							<path d="M7 2v10M2 7h10" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>
						</svg>
					</button>
				</div>
			</div>
		{/if}
	</div>
</div>
