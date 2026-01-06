<script lang="ts">
	import { onMount } from 'svelte';
	
	export let title: string;
	export let minWidth: number = 200;
	export let minHeight: number = 150;
	export let defaultFlex: number = 1;
	
	let flexGrow = defaultFlex;
	let isResizing = false;
	let resizeEdge: 'left' | 'right' | 'top' | 'bottom' | null = null;
	let startX = 0;
	let startY = 0;
	let startFlex = 0;
	let panelElement: HTMLDivElement;
	
	function handleResizeStart(e: MouseEvent, edge: 'left' | 'right' | 'top' | 'bottom') {
		isResizing = true;
		resizeEdge = edge;
		startX = e.clientX;
		startY = e.clientY;
		startFlex = flexGrow;
		e.preventDefault();
		e.stopPropagation();
	}
	
	function handleMouseMove(e: MouseEvent) {
		if (!isResizing || !resizeEdge) return;
		
		let delta = 0;
		
		if (resizeEdge === 'right') {
			delta = (e.clientX - startX) / 200; // More responsive
		} else if (resizeEdge === 'left') {
			delta = (startX - e.clientX) / 200; // Reverse direction
		} else if (resizeEdge === 'bottom') {
			delta = (e.clientY - startY) / 200;
		} else if (resizeEdge === 'top') {
			delta = (startY - e.clientY) / 200;
		}
		
		// Smooth, fluid resizing with better constraints
		const newFlex = Math.max(0.3, Math.min(5, startFlex + delta));
		flexGrow = newFlex;
	}
	
	function handleMouseUp() {
		isResizing = false;
		resizeEdge = null;
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
	.flex-panel {
		position: relative;
		background-color: var(--panel);
		border: 1px solid var(--panel-border);
		border-radius: var(--radius-md);
		display: flex;
		flex-direction: column;
		overflow: hidden;
		min-height: 150px;
	}
	
	.panel-header {
		padding: var(--space-3) var(--space-4);
		border-bottom: 1px solid var(--panel-border);
		background-color: var(--bg-subtle);
		user-select: none;
		flex-shrink: 0;
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
	
	.resize-handle {
		position: absolute;
		background-color: transparent;
		transition: all var(--transition-fast);
		z-index: 10;
	}
	
	.resize-handle:hover,
	.resize-handle.active {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
	}
	
	.resize-left,
	.resize-right {
		top: 0;
		bottom: 0;
		width: 6px;
		cursor: ew-resize;
	}
	
	.resize-left {
		left: -3px;
	}
	
	.resize-right {
		right: -3px;
	}
	
	.resize-top,
	.resize-bottom {
		left: 0;
		right: 0;
		height: 6px;
		cursor: ns-resize;
	}
	
	.resize-top {
		top: -3px;
	}
	
	.resize-bottom {
		bottom: -3px;
	}
</style>

<div 
	bind:this={panelElement}
	class="flex-panel" 
	style="flex: {flexGrow}; min-width: {minWidth}px; min-height: {minHeight}px;"
>
	<div class="panel-header">
		<h3 class="panel-title">{title}</h3>
	</div>
	
	<div class="panel-content">
		<slot />
	</div>
	
	<!-- All 4 edge resize handles -->
	<div 
		class="resize-handle resize-left"
		class:active={isResizing && resizeEdge === 'left'}
		on:mousedown={(e) => handleResizeStart(e, 'left')}
		role="separator"
		aria-orientation="vertical"
	></div>
	
	<div 
		class="resize-handle resize-right"
		class:active={isResizing && resizeEdge === 'right'}
		on:mousedown={(e) => handleResizeStart(e, 'right')}
		role="separator"
		aria-orientation="vertical"
	></div>
	
	<div 
		class="resize-handle resize-top"
		class:active={isResizing && resizeEdge === 'top'}
		on:mousedown={(e) => handleResizeStart(e, 'top')}
		role="separator"
		aria-orientation="horizontal"
	></div>
	
	<div 
		class="resize-handle resize-bottom"
		class:active={isResizing && resizeEdge === 'bottom'}
		on:mousedown={(e) => handleResizeStart(e, 'bottom')}
		role="separator"
		aria-orientation="horizontal"
	></div>
</div>
