<script lang="ts">
	import { onMount } from 'svelte';
	
	export let title: string;
	export let subtitle: string = '';
	export let minWidth: number = 200;
	export let minHeight: number = 150;
	export let defaultFlex: number = 1;
	export let allowHorizontalResize: boolean = false; // NEW: Control horizontal resize
	
	let flexGrow = defaultFlex;
	let isResizing = false;
	let resizeEdge: 'top' | 'bottom' | null = null;
	let startY = 0;
	let startFlex = 0;
	let panelElement: HTMLDivElement;
	
	function handleResizeStart(e: MouseEvent, edge: 'top' | 'bottom') {
		isResizing = true;
		resizeEdge = edge;
		startY = e.clientY;
		startFlex = flexGrow;
		e.preventDefault();
		e.stopPropagation();
	}
	
	function handleMouseMove(e: MouseEvent) {
		if (!isResizing || !resizeEdge) return;
		
		let delta = 0;
		
		if (resizeEdge === 'bottom') {
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
	
	.panel-subtitle {
		font-size: var(--text-sm);
		font-weight: var(--weight-normal);
		color: var(--text-muted);
		margin-left: var(--space-2);
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
		<h3 class="panel-title">
			{title}
			{#if subtitle}
				<span class="panel-subtitle">{subtitle}</span>
			{/if}
		</h3>
	</div>
	
	<div class="panel-content">
		<slot />
	</div>
	
	<!-- ONLY TOP/BOTTOM resize handles (vertical resizing within column) -->
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
