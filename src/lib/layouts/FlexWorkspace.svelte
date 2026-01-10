<script lang="ts">
	import FlexPanel from '$lib/components/panels/FlexPanel.svelte';
	import { fileStore } from '$lib/stores/photoStore';
	import { currentFolderStore } from '$lib/stores/currentFolderStore';
	import ContentPanel from '$lib/layouts/panels/ContentPanel.svelte';
	import FileBrowser from '$lib/layouts/panels/FileBrowser.svelte';
	import PreviewPanel from '$lib/layouts/panels/PreviewPanel.svelte';
	import { onMount } from 'svelte';
	
	let scanResult: any = null;
	let currentFolder: any = null;
	let selectedFile: any = null;
	
	fileStore.subscribe(value => {
		scanResult = value;
	});
	
	currentFolderStore.subscribe(value => {
		currentFolder = value;
		// Clear selection when folder changes
		selectedFile = null;
	});
	
	function handleFileSelect(event: CustomEvent) {
		selectedFile = event.detail;
	}
	
	$: hasFiles = scanResult?.files?.length > 0;
	
	// Two column widths (50/50 default, resizable)
	let leftWidth = 50;  // Content panel
	let rightWidth = 50; // Preview panel
	
	let isResizing = false;
	let startX = 0;
	let startLeftWidth = 0;
	let workspaceWidth = 0;
	let workspaceElement: HTMLDivElement;
	
	function handleColumnResizeStart(e: MouseEvent) {
		isResizing = true;
		startX = e.clientX;
		startLeftWidth = leftWidth;
		
		if (workspaceElement) {
			workspaceWidth = workspaceElement.offsetWidth;
		}
		
		e.preventDefault();
		e.stopPropagation();
	}
	
	function handleMouseMove(e: MouseEvent) {
		if (!isResizing || !workspaceWidth) return;
		
		const deltaX = e.clientX - startX;
		const deltaPercent = (deltaX / workspaceWidth) * 100;
		
		const newLeftWidth = Math.max(25, Math.min(75, startLeftWidth + deltaPercent));
		const newRightWidth = 100 - newLeftWidth;
		
		leftWidth = newLeftWidth;
		rightWidth = newRightWidth;
	}
	
	function handleMouseUp() {
		isResizing = false;
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
	<!-- Left Column: Content -->
	<div class="panel-column" style="width: {leftWidth}%;">
		<FlexPanel 
			title="Content" 
			subtitle={currentFolder?.path ? `${currentFolder.path.split('/').filter(Boolean).pop() || 'Computer'} (${currentFolder.files?.length || 0} items)` : ''}
			minWidth={300} 
			defaultFlex={1}
		>
			<ContentPanel 
				currentPath={currentFolder?.path || ''}
				files={currentFolder?.files || []}
				loading={currentFolder?.loading || false}
				on:fileSelect={handleFileSelect}
			/>
		</FlexPanel>
		
		<div 
			class="column-resize-handle"
			class:active={isResizing}
			on:mousedown={handleColumnResizeStart}
			role="button"
			aria-label="Resize columns"
			tabindex="0"
		></div>
	</div>
	
	<!-- Right Column: Preview -->
	<div class="panel-column" style="width: {rightWidth}%;">
		<FlexPanel title="Preview" minWidth={250} minHeight={200} defaultFlex={1}>
			<PreviewPanel selectedFile={selectedFile} />
		</FlexPanel>
	</div>
</div>
