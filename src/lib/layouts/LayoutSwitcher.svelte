<script lang="ts">
	import { goto } from '$app/navigation';
	import { layoutStore } from '$lib/layouts/store';
	import type { LayoutId, LayoutConfig } from '$lib/layouts/types';
	import { LAYOUTS } from '$lib/layouts/types';
	import { fileStore } from '$lib/stores/photoStore';
	import { icons } from '$lib/ui/icons';
	
	let currentLayout: LayoutConfig;
	let scanResult: any = null;
	
	layoutStore.subscribe(value => {
		currentLayout = value;
	});
	
	fileStore.subscribe(value => {
		scanResult = value;
	});
	
	$: hasFiles = scanResult?.files?.length > 0;
	
	function selectLayout(layoutId: LayoutId) {
		layoutStore.setLayout(layoutId);
	}
	
	const layoutOptions: LayoutId[] = ['essentials', 'transform', 'review', 'analyze'];
</script>

<style>
	.layout-switcher {
		display: flex;
		align-items: center;
		justify-content: space-between;
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
	
	.layout-label {
		font-size: var(--text-sm);
		color: var(--text-muted);
		margin-right: var(--space-2);
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
	
	.actions-section {
		display: flex;
		align-items: center;
		gap: var(--space-1);
	}
	
	.action-button {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2) var(--space-3);
		border: none;
		background: none;
		color: var(--text-muted);
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		cursor: pointer;
		border-radius: 6px;
		transition: all var(--transition-fast);
	}
	
	.action-button:hover:not(:disabled) {
		background-color: var(--bg-subtle);
		color: var(--text);
	}
	
	.action-button:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}
	
	.action-button.primary {
		background-color: var(--accent);
		color: var(--accent-contrast);
	}
	
	.action-button.primary:hover:not(:disabled) {
		background-color: var(--accent-hover);
	}
	
	.action-icon {
		font-size: 16px;
	}
	
	.divider {
		width: 1px;
		height: 24px;
		background-color: var(--panel-border);
		margin: 0 var(--space-2);
	}
</style>

<div class="layout-switcher">
	<!-- Left: Layout switcher -->
	<div class="layout-section">
		<span class="layout-label">Layout:</span>
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
	
	<!-- Right: Action buttons -->
	<div class="actions-section">
		<button
			type="button"
			class="action-button"
			on:click={() => goto('/workspace')}
			title="Scan folder"
		>
			<span class="action-icon">{icons.folder}</span>
			<span>Scan Folder</span>
		</button>
		
		<div class="divider"></div>
		
		<button
			type="button"
			class="action-button"
			disabled={!hasFiles}
			on:click={() => goto('/analyze')}
			title={hasFiles ? 'Analyze files' : 'Scan files first'}
		>
			<span class="action-icon">{icons.search}</span>
			<span>Analyze</span>
		</button>
		
		<button
			type="button"
			class="action-button primary"
			disabled={!hasFiles}
			on:click={() => goto('/transform')}
			title={hasFiles ? 'Create folder structure' : 'Scan files first'}
		>
			<span class="action-icon">{icons.transform}</span>
			<span>Organize</span>
		</button>
	</div>
</div>
