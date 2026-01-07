<script lang="ts">
	import type { NavItem, StatusBadge } from '$lib/types';
	import Sidebar from './Sidebar.svelte';
	import LayoutSwitcher from '$lib/layouts/LayoutSwitcher.svelte';
	
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
	<Sidebar />
	<div class="app-main">
		<!-- Top Bar - Always visible -->
		<LayoutSwitcher />
		
		<!-- Main Content -->
		<main class="app-content">
			<slot />
		</main>
	</div>
</div>
