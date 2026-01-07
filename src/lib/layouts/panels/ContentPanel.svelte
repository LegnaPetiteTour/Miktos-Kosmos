<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { convertFileSrc } from '@tauri-apps/api/core';
	
	export let currentPath: string = '';
	export let files: any[] = [];
	export let loading: boolean = false;
	
	const dispatch = createEventDispatcher();
	
	type SortColumn = 'name' | 'modified' | 'size' | 'type';
	type SortDirection = 'asc' | 'desc';
	
	let sortColumn: SortColumn = 'name';
	let sortDirection: SortDirection = 'asc';
	
	// Extract folder name from path
	$: folderName = currentPath.split('/').filter(Boolean).pop() || 'Computer';
	
	// Check if file is an image
	function isImageFile(file: any): boolean {
		if (file.is_dir) return false;
		const ext = file.name?.split('.').pop()?.toLowerCase() || '';
		return ['jpg', 'jpeg', 'png', 'gif', 'webp', 'heic', 'bmp', 'svg'].includes(ext);
	}
	
	// Get thumbnail URL for image files
	function getThumbnailUrl(file: any): string | null {
		if (!isImageFile(file)) return null;
		const url = convertFileSrc(file.path);
		console.log('ContentPanel - getThumbnailUrl:', {
			fileName: file.name,
			filePath: file.path,
			convertedUrl: url
		});
		return url;
	}
	
	// Sort files based on current sort column and direction
	$: sortedFiles = [...files].sort((a, b) => {
		let result = 0;
		
		switch (sortColumn) {
			case 'name':
				// Folders first, then by name
				if (a.is_dir && !b.is_dir) result = -1;
				else if (!a.is_dir && b.is_dir) result = 1;
				else result = a.name.localeCompare(b.name);
				break;
			
			case 'modified':
				const aTime = a.modified || a.created || 0;
				const bTime = b.modified || b.created || 0;
				result = aTime - bTime;
				break;
			
			case 'size':
				const aSize = a.is_dir ? 0 : (a.size || 0);
				const bSize = b.is_dir ? 0 : (b.size || 0);
				result = aSize - bSize;
				break;
			
			case 'type':
				const aType = getFileType(a);
				const bType = getFileType(b);
				result = aType.localeCompare(bType);
				break;
		}
		
		return sortDirection === 'asc' ? result : -result;
	});
	
	function handleSort(column: SortColumn) {
		if (sortColumn === column) {
			// Toggle direction
			sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
		} else {
			// New column, default to ascending
			sortColumn = column;
			sortDirection = 'asc';
		}
	}
	
	function selectFile(file: any) {
		dispatch('fileSelect', file);
	}
	
	function formatSize(bytes: number): string {
		if (!bytes || bytes === 0) return '0 B';
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(0) + ' KB';
		if (bytes < 1024 * 1024 * 1024) return (bytes / 1024 / 1024).toFixed(1) + ' MB';
		return (bytes / 1024 / 1024 / 1024).toFixed(2) + ' GB';
	}
	
	let imageErrors = new Set<string>();
	
	function handleThumbnailError(filePath: string) {
		console.error('ContentPanel - Thumbnail failed to load:', filePath);
		imageErrors.add(filePath);
		imageErrors = imageErrors; // Trigger reactivity
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
			// Images
			'jpg': 'JPEG image',
			'jpeg': 'JPEG image',
			'png': 'PNG image',
			'gif': 'GIF image',
			'webp': 'WebP image',
			'heic': 'HEIC image',
			'heif': 'HEIF image',
			'bmp': 'BMP image',
			'svg': 'SVG image',
			// Videos
			'mp4': 'MP4 video',
			'mov': 'MOV video',
			'avi': 'AVI video',
			'mkv': 'MKV video',
			'webm': 'WebM video',
			// Audio
			'mp3': 'MP3 audio',
			'wav': 'WAV audio',
			'flac': 'FLAC audio',
			'm4a': 'M4A audio',
			// Documents
			'pdf': 'PDF document',
			'doc': 'Word document',
			'docx': 'Word document',
			'txt': 'Text file',
			'rtf': 'RTF document',
			// Archives
			'zip': 'ZIP archive',
			'rar': 'RAR archive',
			'7z': '7-Zip archive',
			'tar': 'TAR archive',
			'gz': 'GZIP archive',
			// Code
			'js': 'JavaScript file',
			'ts': 'TypeScript file',
			'py': 'Python file',
			'java': 'Java file',
			'cpp': 'C++ file',
			'html': 'HTML file',
			'css': 'CSS file',
			// Executables
			'exe': 'Application',
			'app': 'Application',
			'dmg': 'Disk Image',
			'bin': 'Binary file'
		};
		
		return typeMap[ext] || ext.toUpperCase() + ' file';
	}
	
	function getFileIcon(file: any): string {
		if (file.is_dir) return '📁';
		
		const fileName = file.name || '';
		const ext = fileName.split('.').pop()?.toLowerCase();
		
		switch(ext) {
			case 'jpg':
			case 'jpeg':
			case 'png':
			case 'gif':
			case 'webp':
			case 'heic':
			case 'heif':
			case 'bmp':
			case 'svg':
				return '🖼️';
			case 'mp4':
			case 'mov':
			case 'avi':
			case 'mkv':
			case 'webm':
				return '🎬';
			case 'pdf':
				return '📄';
			case 'doc':
			case 'docx':
			case 'txt':
			case 'rtf':
				return '📝';
			case 'zip':
			case 'rar':
			case '7z':
			case 'tar':
			case 'gz':
				return '📦';
			case 'mp3':
			case 'wav':
			case 'flac':
			case 'm4a':
				return '🎵';
			case 'exe':
			case 'app':
			case 'dmg':
			case 'bin':
				return '⚙️';
			default:
				return '📄';
		}
	}
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
		cursor: pointer;
		user-select: none;
		transition: background-color var(--transition-fast);
	}
	
	.content-table th:hover {
		background-color: rgba(255, 255, 255, 0.05);
	}
	
	.content-table th.sortable {
		position: relative;
	}
	
	.sort-indicator {
		margin-left: var(--space-1);
		font-size: 10px;
		color: var(--text-accent);
		display: inline-block;
		transition: transform var(--transition-fast);
	}
	
	.sort-indicator.desc {
		transform: rotate(180deg);
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
	
	.file-icon {
		font-size: 14px;
		flex-shrink: 0;
	}
	
	.file-thumbnail {
		width: 32px;
		height: 32px;
		flex-shrink: 0;
		border-radius: 4px;
		object-fit: cover;
		background-color: var(--bg-subtle);
	}
	
	.file-thumbnail-placeholder {
		width: 32px;
		height: 32px;
		flex-shrink: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 20px;
	}
	
	.empty-state {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-5);
		text-align: center;
		color: var(--text-muted);
		font-size: var(--text-sm);
	}
	
	.loading-state {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-5);
		color: var(--text-muted);
		font-size: var(--text-sm);
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
			<div class="content-table-wrapper">
				<table class="content-table">
					<thead>
						<tr>
							<th style="width: 40%;" class="sortable" on:click={() => handleSort('name')}>
								Name
								{#if sortColumn === 'name'}
									<span class="sort-indicator" class:desc={sortDirection === 'desc'}>▲</span>
								{/if}
							</th>
							<th style="width: 25%;" class="sortable" on:click={() => handleSort('modified')}>
								Date Modified
								{#if sortColumn === 'modified'}
									<span class="sort-indicator" class:desc={sortDirection === 'desc'}>▲</span>
								{/if}
							</th>
							<th style="width: 15%;" class="sortable" on:click={() => handleSort('size')}>
								Size
								{#if sortColumn === 'size'}
									<span class="sort-indicator" class:desc={sortDirection === 'desc'}>▲</span>
								{/if}
							</th>
							<th style="width: 20%;" class="sortable" on:click={() => handleSort('type')}>
								Type
								{#if sortColumn === 'type'}
									<span class="sort-indicator" class:desc={sortDirection === 'desc'}>▲</span>
								{/if}
							</th>
						</tr>
					</thead>
					<tbody>
						{#each sortedFiles as file}
							<tr on:click={() => selectFile(file)}>
								<td>
									<div class="file-name">
										{#if getThumbnailUrl(file) && !imageErrors.has(file.path)}
											<img 
												src={getThumbnailUrl(file)} 
												alt={file.name}
												class="file-thumbnail"
												loading="lazy"
												on:error={() => handleThumbnailError(file.path)}
											/>
										{:else}
											<div class="file-thumbnail-placeholder">
												{getFileIcon(file)}
											</div>
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
		{/if}
	</div>
</div>
