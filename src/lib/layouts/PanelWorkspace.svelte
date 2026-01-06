<script lang="ts">
	import ResizablePanel from '$lib/components/panels/ResizablePanel.svelte';
	import { fileStore } from '$lib/stores/photoStore';
	import FileBrowser from '$lib/layouts/panels/FileBrowser.svelte';
	import PreviewPanel from '$lib/layouts/panels/PreviewPanel.svelte';
	import MetadataPanel from '$lib/layouts/panels/MetadataPanel.svelte';
	import ToolsPanel from '$lib/layouts/panels/ToolsPanel.svelte';
	
	interface PanelConfig {
		id: string;
		title: string;
		component: any;
		width: number;
		height: number;
		column: 'left' | 'center' | 'right';
		order: number;
	}
	
	let scanResult: any = null;
	
	fileStore.subscribe(value => {
		scanResult = value;
	});
	
	$: hasFiles = scanResult?.files?.length > 0;
	
	// Panel configurations
	let panels: PanelConfig[] = [
		{ id: 'files', title: 'Files', component: FileBrowser, width: 500, height: 600, column: 'left', order: 0 },
		{ id: 'tools', title: 'Tools', component: ToolsPanel, width: 400, height: 600, column: 'center', order: 0 },
		{ id: 'preview', title: 'Preview', component: PreviewPanel, width: 350, height: 350, column: 'right', order: 0 },
		{ id: 'metadata', title: 'Metadata', component: MetadataPanel, width: 350, height: 250, column: 'right', order: 1 }
	];
	
	let draggedPanelId: string | null = null;
	let dropTargetColumn: 'left' | 'center' | 'right' | null = null;
	
	function handleDragStart(id: string) {
		draggedPanelId = id;
	}
	
	function handleDragEnd() {
		draggedPanelId = null;
		dropTargetColumn = null;
	}
	
	function handleDragOver(e: DragEvent, column: 'left' | 'center' | 'right') {
		e.preventDefault();
		dropTargetColumn = column;
	}
	
	function handleDrop(e: DragEvent, column: 'left' | 'center' | 'right') {
		e.preventDefault();
		
		if (!draggedPanelId) return;
		
		// Move panel to new column
		panels = panels.map(panel => {
			if (panel.id === draggedPanelId) {
				return { ...panel, column };
			}
			return panel;
		});
		
		draggedPanelId = null;
		dropTargetColumn = null;
	}
	
	// Group panels by column
	$: leftPanels = panels.filter(p => p.column === 'left').sort((a, b) => a.order - b.order);
	$: centerPanels = panels.filter(p => p.column === 'center').sort((a, b) => a.order - b.order);
	$: rightPanels = panels.filter(p => p.column === 'right').sort((a, b) => a.order - b.order);
	
	// Filter tools panel if no files
	$: visibleCenterPanels = hasFiles ? centerPanels : [];
</script>

<style>
	.panel-workspace {
		display: flex;
		gap: var(--space-3);
		padding: var(--space-3);
		height: 100%;
		overflow: auto;
		background-color: var(--bg);
	}
	
	.panel-column {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		min-height: 100%;
		transition: background-color var(--transition-fast);
		border-radius: var(--radius-md);
		padding: var(--space-2);
	}
	
	.panel-column.drop-target {
		background-color: var(--accent-subtle);
		border: 2px dashed var(--accent);
	}
	
	.panel-column.left {
		flex-shrink: 0;
	}
	
	.panel-column.center {
		flex: 1;
		min-width: 400px;
	}
	
	.panel-column.right {
		flex-shrink: 0;
	}
	
	.panel-column.empty {
		min-width: 100px;
		border: 2px dashed var(--border);
		display: flex;
		align-items: center;
		justify-content: center;
		color: var(--text-muted);
		font-size: var(--text-sm);
	}
</style>

<div class="panel-workspace">
	<!-- Left Column -->
	<div 
		class="panel-column left"
		class:drop-target={dropTargetColumn === 'left'}
		class:empty={leftPanels.length === 0}
		on:dragover={(e) => handleDragOver(e, 'left')}
		on:drop={(e) => handleDrop(e, 'left')}
		role="region"
	>
		{#if leftPanels.length === 0}
			<span>Drop panels here</span>
		{:else}
			{#each leftPanels as panel (panel.id)}
				<ResizablePanel 
					title={panel.title}
					panelId={panel.id}
					initialWidth={panel.width}
					initialHeight={panel.height}
					minWidth={300}
					onDragStart={handleDragStart}
					onDragEnd={handleDragEnd}
				>
					<svelte:component this={panel.component} />
				</ResizablePanel>
			{/each}
		{/if}
	</div>
	
	<!-- Center Column (only if has panels) -->
	{#if visibleCenterPanels.length > 0}
		<div 
			class="panel-column center"
			class:drop-target={dropTargetColumn === 'center'}
			on:dragover={(e) => handleDragOver(e, 'center')}
			on:drop={(e) => handleDrop(e, 'center')}
			role="region"
		>
			{#each visibleCenterPanels as panel (panel.id)}
				<ResizablePanel 
					title={panel.title}
					panelId={panel.id}
					initialWidth={panel.width}
					initialHeight={panel.height}
					minWidth={300}
					onDragStart={handleDragStart}
					onDragEnd={handleDragEnd}
				>
					<svelte:component this={panel.component} />
				</ResizablePanel>
			{/each}
		</div>
	{/if}
	
	<!-- Right Column -->
	<div 
		class="panel-column right"
		class:drop-target={dropTargetColumn === 'right'}
		class:empty={rightPanels.length === 0}
		on:dragover={(e) => handleDragOver(e, 'right')}
		on:drop={(e) => handleDrop(e, 'right')}
		role="region"
	>
		{#if rightPanels.length === 0}
			<span>Drop panels here</span>
		{:else}
			{#each rightPanels as panel (panel.id)}
				<ResizablePanel 
					title={panel.title}
					panelId={panel.id}
					initialWidth={panel.width}
					initialHeight={panel.height}
					minWidth={250}
					minHeight={panel.id === 'preview' ? 200 : 150}
					onDragStart={handleDragStart}
					onDragEnd={handleDragEnd}
				>
					<svelte:component this={panel.component} />
				</ResizablePanel>
			{/each}
		{/if}
	</div>
</div>
