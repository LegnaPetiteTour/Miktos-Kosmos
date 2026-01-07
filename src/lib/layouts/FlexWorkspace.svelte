<script lang="ts">
	import FlexPanel from '$lib/components/panels/FlexPanel.svelte';
	import { fileStore } from '$lib/stores/photoStore';
	import { currentFolderStore } from '$lib/stores/currentFolderStore';
	import ContentPanel from '$lib/layouts/panels/ContentPanel.svelte';
	import FileBrowser from '$lib/layouts/panels/FileBrowser.svelte';
	import PreviewPanel from '$lib/layouts/panels/PreviewPanel.svelte';
	import MetadataPanel from '$lib/layouts/panels/MetadataPanel.svelte';
	import ToolsPanel from '$lib/layouts/panels/ToolsPanel.svelte';
	import { onMount } from 'svelte';
	
	let scanResult: any = null;
	let currentFolder: any = null;
	
	fileStore.subscribe(value => {
		scanResult = value;
	});
	
	currentFolderStore.subscribe(value => {
		currentFolder = value;
	});
	
	$: hasFiles = scanResult?.files?.length > 0;
	
	// Column widths in percentages (more stable than flex)
	let leftWidth = 35; // 35% of workspace - Files
	let centerWidth = 30; // 30% of workspace - Preview + Metadata (was right column)
	let rightWidth = 35; // 35% of workspace - Tools (was center column)
	
	let isResizing = false;
	let resizingBetween: 'left-center' | 'center-right' | null = null;
	let startX = 0;
	let startLeftWidth = 0;
	let startCenterWidth = 0;
	let startRightWidth = 0;
	let workspaceWidth = 0;
	let workspaceElement: HTMLDivElement;
	
	function handleColumnResizeStart(e: MouseEvent, between: 'left-center' | 'center-right') {
		isResizing = true;
		resizingBetween = between;
		startX = e.clientX;
		startLeftWidth = leftWidth;
		startCenterWidth = centerWidth;
		startRightWidth = rightWidth;
		
		// Get workspace width for pixel-to-percentage conversion
		if (workspaceElement) {
			workspaceWidth = workspaceElement.offsetWidth;
		}
		
		e.preventDefault();
		e.stopPropagation();
	}
	
	function handleMouseMove(e: MouseEvent) {
		if (!isResizing || !resizingBetween || !workspaceWidth) return;
		
		const deltaX = e.clientX - startX;
		const deltaPercent = (deltaX / workspaceWidth) * 100;
		
		if (resizingBetween === 'left-center') {
			// Adjust left and center, keep right unchanged
			const newLeftWidth = Math.max(15, Math.min(70, startLeftWidth + deltaPercent));
			const newCenterWidth = Math.max(15, Math.min(70, startCenterWidth - deltaPercent));
			
			// Only update if both are within valid ranges
			if (newLeftWidth >= 15 && newCenterWidth >= 15) {
				leftWidth = newLeftWidth;
				centerWidth = newCenterWidth;
				// rightWidth stays the same
			}
			
		} else if (resizingBetween === 'center-right') {
			// Adjust center and right, keep left unchanged
			const newCenterWidth = Math.max(15, Math.min(70, startCenterWidth + deltaPercent));
			const newRightWidth = Math.max(15, Math.min(70, startRightWidth - deltaPercent));
			
			// Only update if both are within valid ranges
			if (newCenterWidth >= 15 && newRightWidth >= 15) {
				centerWidth = newCenterWidth;
				rightWidth = newRightWidth;
				// leftWidth stays the same
			}
		}
	}
	
	function handleMouseUp() {
		isResizing = false;
		resizingBetween = null;
	}
	
	onMount(() => {
		window.addEventListener('mousemove', handleMouseMove);
		window.addEventListener('mouseup', handleMouseUp);
		
		return () => {
			window.removeEventListener('mousemove', handleMouseMove);
			window.removeEventListener('mouseup', handleMouseUp);
		};
	});
</script>

<style>
	.flex-workspace {
		display: flex;
		gap: 6px;
		padding: 6px;
		height: 100%;
		overflow: hidden;
		background-color: var(--bg);
		position: relative;
	}
	
	.panel-column {
		display: flex;
		flex-direction: column;
		gap: 6px;
		min-width: 250px;
		position: relative;
		flex-shrink: 0;
	}
	
	.column-resize-handle {
		position: absolute;
		top: 0;
		bottom: 0;
		width: 10px;
		right: -5px;
		cursor: ew-resize;
		z-index: 50;
		background-color: transparent;
		transition: all var(--transition-fast);
	}
	
	.column-resize-handle:hover,
	.column-resize-handle.active {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
		border-left: 1px solid rgba(255, 255, 255, 0.1);
		border-right: 1px solid rgba(255, 255, 255, 0.1);
		box-shadow: 0 0 12px rgba(0, 0, 0, 0.2);
	}
	
	.column-resize-handle::before {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 3px;
		height: 40px;
		background-color: var(--text-muted);
		border-radius: 2px;
		opacity: 0;
		transition: opacity var(--transition-fast);
	}
	
	.column-resize-handle:hover::before,
	.column-resize-handle.active::before {
		opacity: 0.4;
	}
</style>

<div class="flex-workspace" bind:this={workspaceElement}>
	<!-- Left Column: Content (Browse folders) -->
	<div class="panel-column" style="width: {leftWidth}%;">
		<FlexPanel title="Content" minWidth={300} defaultFlex={1}>
			<ContentPanel 
				currentPath={currentFolder?.path || ''}
				files={currentFolder?.files || []}
				loading={currentFolder?.loading || false}
			/>
		</FlexPanel>
		
		<div 
			class="column-resize-handle"
			class:active={isResizing && resizingBetween === 'left-center'}
			on:mousedown={(e) => handleColumnResizeStart(e, 'left-center')}
			role="separator"
			aria-orientation="vertical"
			title="Drag to resize Files ↔ Preview"
		></div>
	</div>
	
	<!-- Center Column: Preview + Metadata (SWAPPED FROM RIGHT) -->
	<div class="panel-column" style="width: {centerWidth}%;">
		<FlexPanel title="Preview" minWidth={250} minHeight={200} defaultFlex={1.5}>
			<PreviewPanel />
		</FlexPanel>
		
		<FlexPanel title="Metadata" minWidth={250} minHeight={150} defaultFlex={1}>
			<MetadataPanel />
		</FlexPanel>
		
		{#if hasFiles}
			<div 
				class="column-resize-handle"
				class:active={isResizing && resizingBetween === 'center-right'}
				on:mousedown={(e) => handleColumnResizeStart(e, 'center-right')}
				role="separator"
				aria-orientation="vertical"
				title="Drag to resize Preview ↔ Tools"
			></div>
		{/if}
	</div>
	
	<!-- Right Column: Tools (SWAPPED FROM CENTER, conditional) -->
	{#if hasFiles}
		<div class="panel-column" style="width: {rightWidth}%;">
			<FlexPanel title="Tools" minWidth={300} defaultFlex={1}>
				<ToolsPanel />
			</FlexPanel>
		</div>
	{/if}
</div>
