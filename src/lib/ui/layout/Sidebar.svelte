<script lang="ts">
	import type { NavItem } from '$lib/types';
	import ThemeToggle from '../components/ThemeToggle.svelte';
	import FolderTree from '../components/FolderTree.svelte';
	
	export let items: NavItem[];
	export let selectedId: string;
	export let onNavSelect: (id: string) => void;
	
	let foldersPanelExpanded = true;
	
	function toggleFoldersPanel() {
		foldersPanelExpanded = !foldersPanelExpanded;
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
	}
	
	.sidebar-title {
		font-size: var(--text-lg);
		font-weight: var(--weight-bold);
		color: var(--text);
		letter-spacing: -0.01em;
	}
	
	.folders-panel {
		border-bottom: 1px solid var(--panel-border);
		background-color: var(--bg-subtle);
	}
	
	.folders-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-3) var(--space-4);
		cursor: pointer;
		user-select: none;
		transition: background-color var(--transition-fast);
	}
	
	.folders-header:hover {
		background-color: var(--panel);
	}
	
	.folders-title {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: var(--text);
	}
	
	.folders-icon {
		font-size: 10px;
		color: var(--text-muted);
		transition: transform var(--transition-fast);
	}
	
	.folders-icon.expanded {
		transform: rotate(90deg);
	}
	
	.folders-content {
		padding: var(--space-3);
	}
	
	.sidebar-nav {
		flex: 1;
		padding: var(--space-4);
		overflow-y: auto;
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}
	
	.nav-item {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		width: 100%;
		padding: var(--space-3);
		border: none;
		background: none;
		color: var(--text-muted);
		font-size: var(--text-base);
		font-weight: var(--weight-medium);
		cursor: pointer;
		border-radius: 6px;
		transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
		text-align: left;
		position: relative;
		overflow: hidden;
	}
	
	.nav-item::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(
			135deg,
			rgba(255, 255, 255, 0.1) 0%,
			rgba(255, 255, 255, 0.05) 100%
		);
		opacity: 0;
		transition: opacity 0.2s cubic-bezier(0.4, 0, 0.2, 1);
		border-radius: 6px;
	}
	
	.nav-item:hover:not(.active) {
		background-color: var(--nav-hover-bg);
		color: var(--text);
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05),
		            0 1px 3px rgba(0, 0, 0, 0.08),
		            inset 0 1px 1px rgba(255, 255, 255, 0.1);
		transform: translateY(-1px);
	}
	
	.nav-item:hover:not(.active)::before {
		opacity: 1;
	}
	
	.nav-item.active {
		background-color: var(--nav-active-bg);
		color: var(--text);
		font-weight: var(--weight-semibold);
	}
	
	.nav-label {
		flex: 1;
	}
	
	.nav-badge {
		background-color: var(--danger);
		color: white;
		font-size: var(--text-xs);
		font-weight: var(--weight-semibold);
		padding: 3px 7px;
		border-radius: 12px;
		min-width: 20px;
		text-align: center;
	}
	
	.sidebar-footer {
		padding: var(--space-3);
		border-top: 1px solid var(--panel-border);
	}
</style>

<aside class="sidebar">
	<div class="sidebar-brand">
		<div class="sidebar-title">Miktos Kosmos</div>
	</div>
	
	<!-- Folders Panel -->
	<div class="folders-panel">
		<div class="folders-header" on:click={toggleFoldersPanel}>
			<h3 class="folders-title">Folders</h3>
			<span class="folders-icon" class:expanded={foldersPanelExpanded}>▶</span>
		</div>
		
		{#if foldersPanelExpanded}
			<div class="folders-content">
				<FolderTree />
			</div>
		{/if}
	</div>
	
	<!-- Navigation -->
	<nav class="sidebar-nav">
		{#each items as item (item.id)}
			<button
				type="button"
				class="nav-item"
				class:active={item.id === selectedId}
				on:click={() => onNavSelect(item.id)}
			>
				<span class="nav-label">{item.label}</span>
				{#if item.badgeCount && item.badgeCount > 0}
					<span class="nav-badge">{item.badgeCount}</span>
				{/if}
			</button>
		{/each}
	</nav>
	
	<div class="sidebar-footer">
		<ThemeToggle />
	</div>
</aside>
