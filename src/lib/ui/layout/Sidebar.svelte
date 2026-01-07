<script lang="ts">
	import ThemeToggle from '../components/ThemeToggle.svelte';
	import FolderTree from '../components/FolderTree.svelte';
	import { folderAccessStore } from '$lib/stores/folderAccessStore';
	import type { FolderAccess } from '$lib/stores/folderAccessStore';
	
	let activeTab: 'folders' | 'favorites' = 'folders';
	let favorites: FolderAccess[] = [];
	
	// Subscribe to folder access store
	folderAccessStore.subscribe(value => {
		// Get folders accessed 2+ times
		favorites = value
			.filter(f => f.count >= 2)
			.sort((a, b) => b.count - a.count);
	});
	
	function switchTab(tab: 'folders' | 'favorites') {
		activeTab = tab;
	}
	
	function selectFavorite(favorite: FolderAccess) {
		// Track this access too
		folderAccessStore.trackAccess(favorite.path, favorite.name);
		console.log('Selected favorite:', favorite.path);
		// TODO: Integrate with scan functionality
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
		padding: var(--space-5);
		border-bottom: 1px solid var(--panel-border);
		flex-shrink: 0;
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
		font-weight: var(--weight-semibold);
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
	
	.favorite-count {
		font-size: var(--text-xs);
		color: var(--text-muted);
		flex-shrink: 0;
	}
	
	.empty-state {
		padding: var(--space-5);
		text-align: center;
		color: var(--text-muted);
		font-size: var(--text-sm);
		line-height: 1.6;
	}
	
	.sidebar-footer {
		padding: var(--space-3);
		border-top: 1px solid var(--panel-border);
		flex-shrink: 0;
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
				{#if favorites.length === 0}
					<div class="empty-state">
						No favorites yet.<br/>
						Access folders 2+ times to add them here automatically.
					</div>
				{:else}
					<div class="favorites-list">
						{#each favorites as favorite}
							<div class="favorite-item" on:click={() => selectFavorite(favorite)}>
								<span class="favorite-icon">📁</span>
								<span class="favorite-name">{favorite.name}</span>
								<span class="favorite-count">×{favorite.count}</span>
							</div>
						{/each}
					</div>
				{/if}
			{/if}
		</div>
	</div>
	
	<div class="sidebar-footer">
		<ThemeToggle />
	</div>
</aside>
