<script lang="ts">
	import { layoutStore } from '$lib/layouts/store';
	import type { LayoutId, LayoutConfig } from '$lib/layouts/types';
	import { LAYOUTS } from '$lib/layouts/types';
	
	let currentLayout: LayoutConfig;
	
	layoutStore.subscribe(value => {
		currentLayout = value;
	});
	
	function selectLayout(layoutId: LayoutId) {
		layoutStore.setLayout(layoutId);
	}
	
	const layoutOptions: LayoutId[] = ['browser', 'transform', 'review', 'analyze'];
</script>

<style>
	.layout-switcher {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-2);
		padding: var(--space-3) var(--space-5);
		background-color: var(--panel);
		border-bottom: 1px solid var(--panel-border);
		height: 48px;
	}
	
	.layout-section {
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}
	
	.layout-buttons {
		display: flex;
		gap: var(--space-1);
	}
	
	.layout-button {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-4);
		border: none;
		background: none;
		color: var(--text-muted);
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		cursor: pointer;
		border-radius: 6px;
		transition: all var(--transition-fast);
	}
	
	.layout-button:hover:not(.active):not(:disabled) {
		background-color: var(--bg-subtle);
		color: var(--text);
	}
	
	.layout-button.active {
		background-color: var(--nav-active-bg);
		color: var(--text);
	}
	
	.layout-button:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}
	
	.layout-icon {
		font-size: 16px;
	}

</style>

<div class="layout-switcher">
	<!-- Workspace Mode Buttons -->
	<div class="layout-section">
		<div class="layout-buttons">
			{#each layoutOptions as layoutId}
				{@const layout = LAYOUTS[layoutId]}
				<button
					type="button"
					class="layout-button"
					class:active={currentLayout.id === layoutId}
					on:click={() => selectLayout(layoutId)}
				>
					{#if layout.icon}
						<span class="layout-icon">{layout.icon}</span>
					{/if}
					<span>{layout.name}</span>
				</button>
			{/each}
		</div>
	</div>
</div>
