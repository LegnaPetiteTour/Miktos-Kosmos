<script lang="ts">
	import type { NavItem, StatusBadge } from '$lib/types';
	import Sidebar from './Sidebar.svelte';
	import ToolsPanel from '$lib/layouts/panels/ToolsPanel.svelte';
	import LayoutSwitcher from '$lib/layouts/LayoutSwitcher.svelte';
	
	export const navItems: NavItem[] = [];
	export const selectedNavId: string = '';
	export const onNavSelect: (id: string) => void = () => {};
	export const workspaceLabel: string | undefined = undefined;
	export const statusBadges: StatusBadge[] = [];
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
	
	.right-sidebar {
		width: var(--tools-panel-width);
		height: 100vh;
		flex-shrink: 0;
		background-color: var(--panel);
		border-left: 1px solid var(--panel-border);
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
	<aside class="right-sidebar">
		<ToolsPanel />
	</aside>
</div>
