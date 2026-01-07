<script lang="ts">
	import FlexWorkspace from '$lib/layouts/FlexWorkspace.svelte';
	import HistoryPanel from '$lib/layouts/panels/HistoryPanel.svelte';
	
	// Resizable splitter for workspace/history with LIMITS
	let workspaceHeight = 65;
	let isDragging = false;
	let startY = 0;
	let startHeight = 0;
	
	function handleMouseDown(e: MouseEvent) {
		isDragging = true;
		startY = e.clientY;
		startHeight = workspaceHeight;
		e.preventDefault();
	}
	
	function handleMouseMove(e: MouseEvent) {
		if (!isDragging) return;
		
		const windowHeight = window.innerHeight;
		const newHeight = (e.clientY / windowHeight) * 100;
		
		// STRICT LIMITS: Keep history content always visible
		const minWorkspace = 45; // Min 45% for workspace
		const maxWorkspace = 75; // Max 75% for workspace (ensures 25% for history)
		
		if (newHeight >= minWorkspace && newHeight <= maxWorkspace) {
			workspaceHeight = newHeight;
		}
	}
	
	function handleMouseUp() {
		isDragging = false;
	}
</script>

<svelte:window on:mousemove={handleMouseMove} on:mouseup={handleMouseUp} />

<style>
	.home-page {
		display: flex;
		flex-direction: column;
		height: 100%;
		background-color: var(--bg);
	}
	
	.workspace-section {
		overflow: hidden;
		flex-shrink: 0;
		display: flex;
		flex-direction: column;
	}
	
	.resize-handle {
		height: 8px;
		background-color: transparent;
		cursor: ns-resize;
		position: relative;
		transition: all var(--transition-fast);
		flex-shrink: 0;
		z-index: 100;
	}
	
	.resize-handle:hover,
	.resize-handle.dragging {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
		border-top: 1px solid rgba(255, 255, 255, 0.1);
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
	}
	
	.resize-handle::before {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 40px;
		height: 3px;
		background-color: var(--text-muted);
		border-radius: 2px;
		opacity: 0.2;
		transition: opacity var(--transition-fast);
	}
	
	.resize-handle:hover::before,
	.resize-handle.dragging::before {
		opacity: 0.5;
	}
	
	.history-section {
		flex: 1;
		overflow: hidden;
		min-height: 200px;
	}
</style>

<div class="home-page">
	<!-- Middle: Flex Workspace (Resizable with LIMITS) -->
	<div class="workspace-section" style="height: {workspaceHeight}%">
		<FlexWorkspace />
	</div>
	
	<!-- Resize Handle (Glassmorphism on hover) -->
	<div 
		class="resize-handle" 
		class:dragging={isDragging}
		on:mousedown={handleMouseDown}
		role="separator"
		aria-orientation="horizontal"
	></div>
	
	<!-- Bottom: History Panel (ALWAYS VISIBLE min 25%) -->
	<div class="history-section">
		<HistoryPanel />
	</div>
</div>
