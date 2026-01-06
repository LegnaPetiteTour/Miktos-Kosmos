<script lang="ts">
	import type { PanelConfig } from '$lib/layouts/types';
	
	export let config: PanelConfig;
	export let title: string;
</script>

<style>
	.panel {
		display: flex;
		flex-direction: column;
		background-color: var(--panel);
		border: 1px solid var(--panel-border);
		border-radius: var(--radius-md);
		overflow: hidden;
		height: 100%;
	}
	
	.panel-header {
		padding: var(--space-3) var(--space-4);
		border-bottom: 1px solid var(--panel-border);
		background-color: var(--bg-subtle);
	}
	
	.panel-title {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: var(--text);
	}
	
	.panel-content {
		flex: 1;
		overflow: auto;
		padding: var(--space-4);
	}
</style>

{#if config.visible}
	<div class="panel">
		<div class="panel-header">
			<h3 class="panel-title">{title}</h3>
		</div>
		<div class="panel-content">
			<slot />
		</div>
	</div>
{/if}
