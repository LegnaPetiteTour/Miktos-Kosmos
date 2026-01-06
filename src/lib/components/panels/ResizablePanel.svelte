<script lang="ts">
	import { onMount } from 'svelte';
	
	export let title: string;
	export let initialWidth: number = 300;
	export let initialHeight: number = 400;
	export let minWidth: number = 200;
	export let minHeight: number = 150;
	export let resizable: boolean = true;
	export let panelId: string;
	export let onDragStart: ((id: string) => void) | undefined = undefined;
	export let onDragEnd: (() => void) | undefined = undefined;
	
	let panelElement: HTMLDivElement;
	let width = initialWidth;
	let height = initialHeight;
	let isResizingX = false;
	let isResizingY = false;
	let isDraggingPanel = false;
	let startX = 0;
	let startY = 0;
	let startWidth = 0;
	let startHeight = 0;
	
	function handleResizeXStart(e: MouseEvent) {
		if (!resizable) return;
		isResizingX = true;
		startX = e.clientX;
		startWidth = width;
		e.preventDefault();
		e.stopPropagation();
	}
	
	function handleResizeYStart(e: MouseEvent) {
		if (!resizable) return;
		isResizingY = true;
		startY = e.clientY;
		startHeight = height;
		e.preventDefault();
		e.stopPropagation();
	}
	
	function handleHeaderDragStart(e: MouseEvent) {
		// Only drag from header, not resize handles
		if ((e.target as HTMLElement).closest('.resize-handle')) return;
		
		isDraggingPanel = true;
		if (onDragStart) {
			onDragStart(panelId);
		}
	}
	
	function handleMouseMove(e: MouseEvent) {
		if (isResizingX) {
			const deltaX = e.clientX - startX;
			width = Math.max(minWidth, startWidth + deltaX);
		}
		if (isResizingY) {
			const deltaY = e.clientY - startY;
			height = Math.max(minHeight, startHeight + deltaY);
		}
	}
	
	function handleMouseUp() {
		if (isDraggingPanel && onDragEnd) {
			onDragEnd();
		}
		isResizingX = false;
		isResizingY = false;
		isDraggingPanel = false;
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
	.resizable-panel {
		position: relative;
		background-color: var(--panel);
		border: 1px solid var(--panel-border);
		border-radius: var(--radius-md);
		display: flex;
		flex-direction: column;
		overflow: hidden;
		transition: box-shadow var(--transition-fast);
	}
	
	.resizable-panel.dragging {
		opacity: 0.8;
		box-shadow: var(--shadow-lg);
		z-index: 1000;
	}
	
	.panel-header {
		padding: var(--space-3) var(--space-4);
		border-bottom: 1px solid var(--panel-border);
		background-color: var(--bg-subtle);
		cursor: grab;
		user-select: none;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}
	
	.panel-header:active {
		cursor: grabbing;
	}
	
	.panel-title {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: var(--text);
	}
	
	.drag-indicator {
		display: flex;
		gap: 2px;
		opacity: 0.3;
	}
	
	.drag-dot {
		width: 3px;
		height: 3px;
		background-color: var(--text-muted);
		border-radius: 50%;
	}
	
	.panel-header:hover .drag-indicator {
		opacity: 0.6;
	}
	
	.panel-content {
		flex: 1;
		overflow: auto;
		padding: var(--space-4);
	}
	
	.resize-handle {
		position: absolute;
		background-color: transparent;
		transition: background-color var(--transition-fast);
		z-index: 10;
	}
	
	.resize-handle:hover {
		background-color: var(--accent);
	}
	
	.resize-handle.resizing {
		background-color: var(--accent);
	}
	
	.resize-x {
		right: 0;
		top: 0;
		bottom: 0;
		width: 4px;
		cursor: ew-resize;
	}
	
	.resize-y {
		bottom: 0;
		left: 0;
		right: 0;
		height: 4px;
		cursor: ns-resize;
	}
	
	.resize-corner {
		bottom: 0;
		right: 0;
		width: 12px;
		height: 12px;
		cursor: nwse-resize;
	}
	
	.resize-corner::after {
		content: '';
		position: absolute;
		bottom: 2px;
		right: 2px;
		width: 8px;
		height: 8px;
		border-right: 2px solid var(--text-muted);
		border-bottom: 2px solid var(--text-muted);
		opacity: 0.3;
	}
	
	.resize-corner:hover::after {
		opacity: 0.6;
	}
</style>

<div 
	class="resizable-panel" 
	class:dragging={isDraggingPanel}
	bind:this={panelElement}
	style="width: {width}px; height: {height}px;"
	draggable="true"
	data-panel-id={panelId}
>
	<div 
		class="panel-header"
		on:mousedown={handleHeaderDragStart}
	>
		<h3 class="panel-title">{title}</h3>
		<div class="drag-indicator">
			<div class="drag-dot"></div>
			<div class="drag-dot"></div>
			<div class="drag-dot"></div>
			<div class="drag-dot"></div>
			<div class="drag-dot"></div>
			<div class="drag-dot"></div>
		</div>
	</div>
	
	<div class="panel-content">
		<slot />
	</div>
	
	{#if resizable}
		<!-- Right edge resize handle -->
		<div 
			class="resize-handle resize-x"
			class:resizing={isResizingX}
			on:mousedown={handleResizeXStart}
			role="separator"
			aria-orientation="vertical"
		></div>
		
		<!-- Bottom edge resize handle -->
		<div 
			class="resize-handle resize-y"
			class:resizing={isResizingY}
			on:mousedown={handleResizeYStart}
			role="separator"
			aria-orientation="horizontal"
		></div>
		
		<!-- Corner resize handle -->
		<div 
			class="resize-handle resize-corner"
			on:mousedown={(e) => {
				handleResizeXStart(e);
				handleResizeYStart(e);
			}}
		></div>
	{/if}
</div>
