<script lang="ts">
	import LayoutSwitcher from '$lib/layouts/LayoutSwitcher.svelte';
	import Workspace from '$lib/layouts/Workspace.svelte';
</script>

<style>
	.organize-page {
		display: flex;
		flex-direction: column;
		height: 100%;
		background-color: var(--bg);
	}
	
	.organize-content {
		flex: 1;
		overflow: hidden;
	}
</style>

<div class="organize-page">
	<LayoutSwitcher />
	<div class="organize-content">
		<Workspace />
	</div>
</div>
