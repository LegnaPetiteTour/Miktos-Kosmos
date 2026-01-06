<script lang="ts">
	import type { NavItem } from '$lib/types';
	
	export let items: NavItem[];
	export let selectedId: string;
	export let onNavSelect: (id: string) => void;
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
		border-radius: var(--radius-md);
		transition: all var(--transition-fast);
		position: relative;
		text-align: left;
	}
	
	.nav-item:hover {
		background-color: var(--bg-subtle);
		color: var(--text);
	}
	
	.nav-item.active {
		background-color: var(--accent);
		color: var(--accent-contrast);
		font-weight: var(--weight-semibold);
		box-shadow: var(--shadow-sm);
	}
	
	.nav-item.active::before {
		content: '';
		position: absolute;
		left: 0;
		top: 50%;
		transform: translateY(-50%);
		width: 3px;
		height: 60%;
		background-color: var(--accent-contrast);
		border-radius: 0 2px 2px 0;
	}
	
	.nav-icon {
		font-size: 18px;
		flex-shrink: 0;
		line-height: 1;
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
		padding: var(--space-4);
		border-top: 1px solid var(--panel-border);
		font-size: var(--text-xs);
		color: var(--text-subtle);
	}
</style>

<aside class="sidebar">
	<div class="sidebar-brand">
		<div class="sidebar-title">Miktos Kosmos</div>
	</div>
	
	<nav class="sidebar-nav">
		{#each items as item (item.id)}
			<button
				type="button"
				class="nav-item"
				class:active={item.id === selectedId}
				on:click={() => onNavSelect(item.id)}
			>
				{#if item.icon}
					<span class="nav-icon">{item.icon}</span>
				{/if}
				<span class="nav-label">{item.label}</span>
				{#if item.badgeCount && item.badgeCount > 0}
					<span class="nav-badge">{item.badgeCount}</span>
				{/if}
			</button>
		{/each}
	</nav>
	
	<div class="sidebar-footer">
		<!-- Optional footer content -->
	</div>
</aside>
