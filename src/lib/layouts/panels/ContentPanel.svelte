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
</style>

<div class="content-panel">
	<div class="content-header">
		<div class="content-title">
			<span class="content-title-label">Content:</span>
			<span>{folderName}</span>
			{#if files.length > 0}
				<span class="file-count">({files.length} items)</span>
			{/if}
		</div>
	</div>
	
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
			<div class="keyboard-hint">
				↑↓ Navigate & Preview • Enter Open Folder • Backspace Go Up • Home/End Jump
			</div>
		{/if}
	</div>
</div>
