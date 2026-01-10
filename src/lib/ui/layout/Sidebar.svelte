<script lang="ts">
	import FolderTree from '../components/FolderTree.svelte';
	import { folderAccessStore } from '$lib/stores/folderAccessStore';
	import { currentFolderStore } from '$lib/stores/currentFolderStore';
	import type { FolderAccess } from '$lib/stores/folderAccessStore';
	import { invoke } from '@tauri-apps/api/core';
	
	interface ExpandableFavorite extends FolderAccess {
		expanded: boolean;
		children: any[] | null;
	}
	
	let activeTab: 'folders' | 'favorites' = 'folders';
	let expandedFavorites: ExpandableFavorite[] = [];
	let isUpdating = false;
	
	// Subscribe to folder access store
	folderAccessStore.subscribe(value => {
		if (isUpdating) return;
		
		const rawFavorites = value
			.filter(f => f.count >= 2)
			.sort((a, b) => b.count - a.count);
		
		// Preserve expansion state
		const oldExpanded = new Map(
			expandedFavorites.map(f => [f.path, { expanded: f.expanded, children: f.children }])
		);
		
		expandedFavorites = rawFavorites.map(raw => {
			const old = oldExpanded.get(raw.path);
			return {
				...raw,
				expanded: old?.expanded || false,
				children: old?.children || null
			};
		});
	});
	
	function switchTab(tab: 'folders' | 'favorites') {
		activeTab = tab;
	}
	
	async function toggleFavorite(index: number) {
		isUpdating = true;
		
		const favorite = expandedFavorites[index];
		
		// Track access
		folderAccessStore.trackAccess(favorite.path, favorite.name);
		
		// Load content panel immediately (don't wait)
		currentFolderStore.setFolder(favorite.path, favorite.name);
		invoke('list_directory', { path: favorite.path })
			.then((files: any) => {
				currentFolderStore.setFiles(files);
			})
			.catch((error) => {
				console.error('Failed to load folder:', error);
				currentFolderStore.setFiles([]);
			});
		
		// Toggle expansion IMMEDIATELY (don't wait for children)
		favorite.expanded = !favorite.expanded;
		
		// Force UI update NOW
		expandedFavorites = [...expandedFavorites];
		
		// Load children in background if expanding
		if (favorite.expanded && favorite.children === null) {
			invoke('list_directory', { path: favorite.path })
				.then((allItems: any) => {
					const folders = allItems
						.filter((item: any) => item.is_dir)
						.map((item: any) => ({
							name: item.name,
							path: item.path,
							is_dir: item.is_dir,
							expanded: false,
							children: null
						}));
					
					favorite.children = folders;
					expandedFavorites = [...expandedFavorites];
				})
				.catch((error) => {
					console.error('Failed to load children:', error);
					favorite.children = [];
					expandedFavorites = [...expandedFavorites];
				});
		}
		
		// Re-enable store updates
		setTimeout(() => {
			isUpdating = false;
		}, 50);
	}
	
	async function toggleSubfolder(parentIndex: number, childIndex: number) {
		isUpdating = true;
		
		const parent = expandedFavorites[parentIndex];
		const child = parent.children![childIndex];
		
		// Track access
		folderAccessStore.trackAccess(child.path, child.name);
		
		// Load content panel (async)
		currentFolderStore.setFolder(child.path, child.name);
		invoke('list_directory', { path: child.path })
			.then((files: any) => {
				currentFolderStore.setFiles(files);
			})
			.catch((error) => {
				console.error('Failed to load subfolder:', error);
				currentFolderStore.setFiles([]);
			});
		
		// Toggle IMMEDIATELY
		child.expanded = !child.expanded;
		
		// Update UI NOW
		expandedFavorites = [...expandedFavorites];
		
		// Load children in background
		if (child.expanded && child.children === null) {
			invoke('list_directory', { path: child.path })
				.then((allItems: any) => {
					const folders = allItems
						.filter((item: any) => item.is_dir)
						.map((item: any) => ({
							name: item.name,
							path: item.path,
							is_dir: item.is_dir,
							expanded: false,
							children: null
						}));
					
					child.children = folders;
					expandedFavorites = [...expandedFavorites];
				})
				.catch((error) => {
					console.error('Failed to load sub-children:', error);
					child.children = [];
					expandedFavorites = [...expandedFavorites];
				});
		}
		
		setTimeout(() => {
			isUpdating = false;
		}, 50);
	}
</script>

<style>
	.sidebar {
		width: var(--sidebar-width);
		height: 100vh;
		background-color: var(--panel);
		border-right: 1px solid var(--panel-border);
		display: flex;
		flex-direction: column;
		flex-shrink: 0;
	}
	
	.sidebar-brand {
		padding: var(--space-4) var(--space-3);
		border-bottom: 1px solid var(--panel-border);
		flex-shrink: 0;
		display: flex;
		align-items: center;
		height: 56px;
		box-sizing: border-box;
	}
	
	.sidebar-title {
		font-size: var(--text-lg);
		font-weight: var(--weight-bold);
		color: var(--text);
		letter-spacing: -0.01em;
	}
	
	.tabs-header {
		display: flex;
		border-bottom: 1px solid var(--panel-border);
		flex-shrink: 0;
		background-color: var(--bg-subtle);
	}
	
	.tab-button {
		flex: 1;
		padding: var(--space-3);
		border: none;
		background: none;
		color: var(--text-muted);
		font-size: var(--text-sm);
		font-weight: var(--semibold);
		cursor: pointer;
		transition: all var(--transition-fast);
		border-bottom: 2px solid transparent;
	}
	
	.tab-button:hover:not(.active) {
		color: var(--text);
		background-color: var(--panel);
	}
	
	.tab-button.active {
		color: var(--text);
		background-color: var(--panel);
		border-bottom-color: var(--accent);
	}
	
	.tab-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		overflow: hidden;
		background-color: var(--bg-subtle);
	}
	
	.content-scroll {
		flex: 1;
		overflow-y: auto;
		overflow-x: hidden;
		padding: var(--space-3);
	}
	
	.favorites-list {
		display: flex;
		flex-direction: column;
		gap: 0;
	}
	
	.favorite-item {
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
	
	.favorite-item:hover {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
	}
	
	.folder-arrow {
		font-size: 10px;
		color: var(--text-muted);
		flex-shrink: 0;
		width: 12px;
		transition: transform 0.15s ease-out;
		display: inline-block;
		transform-origin: center;
		will-change: transform;
	}
	
	.folder-arrow.expanded {
		transform: rotate(90deg);
	}
	
	.favorite-icon {
		font-size: 14px;
		flex-shrink: 0;
		width: 16px;
		text-align: center;
	}
	
	.favorite-name {
		flex: 1;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}
	
	.favorite-children {
		padding-left: var(--space-4);
	}
	
	.empty-state {
		padding: var(--space-5);
		text-align: center;
		color: var(--text-muted);
		font-size: var(--text-sm);
		line-height: 1.6;
	}
</style>

<aside class="sidebar">
	<div class="sidebar-brand">
		<div class="sidebar-title">Miktos Kosmos</div>
	</div>
	
	<!-- Tabs Header -->
	<div class="tabs-header">
		<button
			type="button"
			class="tab-button"
			class:active={activeTab === 'folders'}
			on:click={() => switchTab('folders')}
		>
			Folders
		</button>
		<button
			type="button"
			class="tab-button"
			class:active={activeTab === 'favorites'}
			on:click={() => switchTab('favorites')}
		>
			Favorites
		</button>
	</div>
	
	<!-- Tab Content -->
	<div class="tab-content">
		<div class="content-scroll">
			{#if activeTab === 'folders'}
				<FolderTree />
			{:else}
				{#if expandedFavorites.length === 0}
					<div class="empty-state">
						No favorites yet.<br/>
						Access folders 2+ times to add them here automatically.
					</div>
				{:else}
					<div class="favorites-list">
						{#each expandedFavorites as favorite, i (favorite.path)}
							<div>
								<!-- Parent Favorite -->
								<div 
									class="favorite-item" 
									on:click={() => toggleFavorite(i)}
								on:keydown={(e) => e.key === 'Enter' && toggleFavorite(i)}
								role="button"
								tabindex="0"
							>
								<span class="folder-arrow" class:expanded={favorite.expanded}>▶</span>
									<div class="favorite-children">
										{#each favorite.children as child, j (child.path)}
											<div>
												<div 
													class="favorite-item" 
													on:click={() => toggleSubfolder(i, j)}
											on:keydown={(e) => e.key === 'Enter' && toggleSubfolder(i, j)}
											role="button"
											tabindex="0"
										>
											<span class="folder-arrow" class:expanded={child.expanded}>▶</span>
												<!-- Sub-subfolders -->
												{#if child.expanded && child.children && child.children.length > 0}
													<div class="favorite-children">
														{#each child.children as grandchild (grandchild.path)}
															<div 
																class="favorite-item"
																role="button"
																tabindex="0"
															>
																<span class="folder-arrow"></span>
																<span class="favorite-icon">📁</span>
																<span class="favorite-name">{grandchild.name}</span>
															</div>
														{/each}
													</div>
												{/if}
											</div>
										{/each}
									</div>
								{/if}
							</div>
						{/each}
					</div>
				{/if}
			{/if}
		</div>
	</div>
</aside>