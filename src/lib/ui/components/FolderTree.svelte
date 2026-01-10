<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { folderAccessStore } from '$lib/stores/folderAccessStore';
	import { currentFolderStore } from '$lib/stores/currentFolderStore';
	import type { FolderAccess } from '$lib/stores/folderAccessStore';
	
	interface FolderItem {
		name: string;
		path: string;
		is_dir: boolean;
		children?: FolderItem[];
		expanded?: boolean;
	}
	
	let rootFolders: FolderItem[] = [];
	let expandedPaths = new Set<string>();
	
	// Get system folders (Desktop, Documents, etc.)
	onMount(async () => {
		try {
			// Get home directory
			const homeDir = await invoke('get_home_dir');
			
			rootFolders = [
				{ name: '💻 Computer', path: '/', is_dir: true, expanded: false },
				{ name: '🏠 Home', path: homeDir as string, is_dir: true, expanded: true },
				{ name: '🖥️ Desktop', path: `${homeDir}/Desktop`, is_dir: true, expanded: false },
				{ name: '📄 Documents', path: `${homeDir}/Documents`, is_dir: true, expanded: false },
				{ name: '🖼️ Pictures', path: `${homeDir}/Pictures`, is_dir: true, expanded: false },
				{ name: '📹 Videos', path: `${homeDir}/Movies`, is_dir: true, expanded: false },
				{ name: '🎵 Music', path: `${homeDir}/Music`, is_dir: true, expanded: false },
				{ name: '💾 Downloads', path: `${homeDir}/Downloads`, is_dir: true, expanded: false }
			];
		} catch (error) {
			console.error('Failed to get system folders:', error);
		}
	});
	
	async function toggleFolder(folder: FolderItem) {
		if (!folder.is_dir) return;
		
		// Always track and load contents when clicking a folder
		folderAccessStore.trackAccess(folder.path, folder.name);
		
		// Set as current folder and load its contents IMMEDIATELY
		currentFolderStore.setFolder(folder.path, folder.name);
		console.log('Loading folder:', folder.path);
		try {
			const files = await invoke('list_directory', { path: folder.path }) as any[];
			console.log('Received files:', files.length, files);
			currentFolderStore.setFiles(files);
		} catch (error) {
			console.error('Failed to load folder contents:', error);
			currentFolderStore.setFiles([]);
		}
		
		// Toggle expansion
		folder.expanded = !folder.expanded;
		
		if (folder.expanded && !folder.children) {
			// Load children
			try {
				const children = await invoke('list_directory', { path: folder.path }) as any[];
				folder.children = children
					.filter((item: any) => item.is_dir) // Only show folders
					.map((item: any) => ({
						name: item.name,
						path: item.path,
						is_dir: item.is_dir,
						expanded: false
					}));
			} catch (error) {
				console.error('Failed to list directory:', error);
				folder.children = [];
			}
		}
		
		// Force reactivity
		rootFolders = [...rootFolders];
	}
	
	function selectFolder(folder: FolderItem) {
		// Track folder access
		folderAccessStore.trackAccess(folder.path, folder.name);
		
		// Emit event or call parent function
		console.log('Selected folder:', folder.path);
		// TODO: Integrate with scan functionality
	}
</script>

<style>
	.folder-tree {
		display: flex;
		flex-direction: column;
		gap: 0;
		height: 100%;
	}
	
	.folder-item {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-3);
		cursor: pointer;
		user-select: none;
		transition: background-color var(--transition-fast);
		font-size: var(--text-xs);
		color: var(--text);
		border-radius: var(--radius-sm);
		margin: 1px 0;
	}
	
	.folder-item:hover {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
	}
	
	.folder-icon {
		font-size: 14px;
		flex-shrink: 0;
		width: 16px;
		text-align: center;
	}
	
	.folder-arrow {
		font-size: 10px;
		color: var(--text-muted);
		flex-shrink: 0;
		width: 12px;
		transition: transform var(--transition-fast);
	}
	
	.folder-arrow.expanded {
		transform: rotate(90deg);
	}
	
	.folder-name {
		flex: 1;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
	
	.folder-children {
		padding-left: var(--space-4);
	}
</style>

<div class="folder-tree">
	{#each rootFolders as folder}
		<div>
			<div class="folder-item" on:click={() => toggleFolder(folder)} on:keydown={(e) => e.key === 'Enter' && toggleFolder(folder)} role="button" tabindex="0">
				{#if folder.is_dir}
					<span class="folder-arrow" class:expanded={folder.expanded}>▶</span>
				{:else}
					<span class="folder-arrow"></span>
				{/if}
				<span class="folder-icon">
					{#if folder.name.includes('💻') || folder.name.includes('🏠') || folder.name.includes('🖥️') || folder.name.includes('📄') || folder.name.includes('🖼️') || folder.name.includes('📹') || folder.name.includes('🎵') || folder.name.includes('💾')}
						<!-- Emoji already in name -->
					{:else if folder.is_dir}
						📁
					{:else}
						📄
					{/if}
				</span>
				<span class="folder-name">{folder.name}</span>
			</div>
			
			{#if folder.expanded && folder.children}
				<div class="folder-children">
					{#each folder.children as child}
						<div class="folder-item" on:click={() => toggleFolder(child)} on:keydown={(e) => e.key === 'Enter' && toggleFolder(child)} role="button" tabindex="0">
							{#if child.is_dir}
								<span class="folder-arrow" class:expanded={child.expanded}>▶</span>
							{:else}
								<span class="folder-arrow"></span>
							{/if}
							<span class="folder-icon">
								{child.is_dir ? '📁' : '📄'}
							</span>
							<span class="folder-name">{child.name}</span>
						</div>
						
						{#if child.expanded && child.children}
							<div class="folder-children">
								{#each child.children as grandchild}
								<div class="folder-item" on:click={() => selectFolder(grandchild)} on:keydown={(e) => e.key === 'Enter' && selectFolder(grandchild)} role="button" tabindex="0">
										<span class="folder-arrow"></span>
										<span class="folder-icon">
											{grandchild.is_dir ? '📁' : '📄'}
										</span>
										<span class="folder-name">{grandchild.name}</span>
									</div>
								{/each}
							</div>
						{/if}
					{/each}
				</div>
			{/if}
		</div>
	{/each}
</div>
