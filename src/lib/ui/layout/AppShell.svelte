<script lang="ts">
	import type { NavItem, StatusBadge } from '$lib/types';
	import Sidebar from './Sidebar.svelte';
	import TopBar from './TopBar.svelte';
	
	export let navItems: NavItem[];
	export let selectedNavId: string;
	export let onNavSelect: (id: string) => void;
	export let workspaceLabel: string | undefined = undefined;
	export let statusBadges: StatusBadge[] = [];
</script>

<style>
	.app-shell {
		display: flex;
		height: 100vh;
		width: 100vw;
		overflow: hidden;
		background-color: var(--bg);
	}
	
	.app-main {
		display: flex;
		flex-direction: column;
		flex: 1;
		min-width: 0; /* Allow flex shrinking */
	}
	
	.app-content {
		flex: 1;
		overflow-y: auto;
		background-color: var(--bg-subtle);
	}
</style>

<div class="app-shell">
	<Sidebar 
		items={navItems} 
		selectedId={selectedNavId} 
		{onNavSelect} 
	/>
	<div class="app-main">
		<TopBar 
			{workspaceLabel} 
			{statusBadges} 
		/>
		<main class="app-content">
			<slot />
		</main>
	</div>
</div>
