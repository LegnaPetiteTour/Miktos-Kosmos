<script lang="ts">
	import { layoutStore } from '$lib/layouts/store';
	import type { LayoutConfig, PanelConfig } from '$lib/layouts/types';
	import Panel from '$lib/layouts/Panel.svelte';
	import FileBrowser from './panels/FileBrowser.svelte';
	import PreviewPanel from './panels/PreviewPanel.svelte';
	import MetadataPanel from './panels/MetadataPanel.svelte';
	import ToolsPanel from './panels/ToolsPanel.svelte';
	import HistoryPanel from './panels/HistoryPanel.svelte';
	
	let currentLayout: LayoutConfig;
	
	layoutStore.subscribe(value => {
		currentLayout = value;
	});
	
	function getPanelComponent(panelId: string) {
		switch (panelId) {
			case 'files':
				return FileBrowser;
			case 'preview':
				return PreviewPanel;
			case 'metadata':
				return MetadataPanel;
			case 'tools':
				return ToolsPanel;
			case 'history':
				return HistoryPanel;
			default:
				return null;
		}
	}
	
	function getPanelTitle(panelId: string): string {
		const titles: Record<string, string> = {
			folders: 'Folders',
			files: 'Files',
			preview: 'Preview',
			metadata: 'Metadata',
			tools: 'Tools',
			history: 'History'
		};
		return titles[panelId] || panelId;
	}
</script>

<style>
	.workspace {
		display: flex;
		height: 100%;
		gap: var(--space-2);
		padding: var(--space-2);
		background-color: var(--bg);
	}
	
	.workspace-section {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}
	
	.workspace-left {
		flex-shrink: 0;
	}
	
	.workspace-center {
		flex: 1;
		min-width: 0;
	}
	
	.workspace-right {
		flex-shrink: 0;
	}
	
	.workspace-bottom {
		flex-shrink: 0;
	}
</style>

<div class="workspace">
	<!-- Left Section -->
	{#if currentLayout.panels.left && currentLayout.panels.left.length > 0}
		<div 
			class="workspace-section workspace-left" 
			style="width: {currentLayout.panels.left[0].width}%"
		>
			{#each currentLayout.panels.left as panelConfig}
				<Panel config={panelConfig} title={getPanelTitle(panelConfig.id)}>
					{#if panelConfig.id === 'files'}
						<FileBrowser />
					{:else}
						<div style="color: var(--text-muted);">
							{panelConfig.id} panel (coming soon)
						</div>
					{/if}
				</Panel>
			{/each}
		</div>
	{/if}
	
	<!-- Center Section -->
	{#if currentLayout.panels.center && currentLayout.panels.center.length > 0}
		<div class="workspace-section workspace-center">
			{#each currentLayout.panels.center as panelConfig}
				<Panel config={panelConfig} title={getPanelTitle(panelConfig.id)}>
					{#if panelConfig.id === 'files'}
						<FileBrowser />
					{:else if panelConfig.id === 'tools'}
						<ToolsPanel />
					{:else}
						<div style="color: var(--text-muted);">
							{panelConfig.id} panel (coming soon)
						</div>
					{/if}
				</Panel>
			{/each}
		</div>
	{/if}
	
	<!-- Right Section -->
	{#if currentLayout.panels.right && currentLayout.panels.right.length > 0}
		<div 
			class="workspace-section workspace-right"
			style="width: {currentLayout.panels.right[0].width}%"
		>
			{#each currentLayout.panels.right as panelConfig}
				<Panel config={panelConfig} title={getPanelTitle(panelConfig.id)}>
					{#if panelConfig.id === 'preview'}
						<PreviewPanel />
					{:else if panelConfig.id === 'metadata'}
						<MetadataPanel />
					{:else if panelConfig.id === 'history'}
						<HistoryPanel />
					{:else}
						<div style="color: var(--text-muted);">
							{panelConfig.id} panel (coming soon)
						</div>
					{/if}
				</Panel>
			{/each}
		</div>
	{/if}
</div>
