<script lang="ts">
	import { fileStore } from '$lib/stores/photoStore';
	import { icons } from '$lib/ui/icons';
	import { operationsStore } from '$lib/stores/operationsStore';
	import type { OperationResult } from '$lib/types';
	
	import LayoutSwitcher from '$lib/layouts/LayoutSwitcher.svelte';
	import FlexWorkspace from '$lib/layouts/FlexWorkspace.svelte';
	import Section from '$lib/ui/layout/Section.svelte';
	import StatCard from '$lib/ui/components/StatCard.svelte';
	
	let scanResult: any = null;
	let operations: OperationResult[] = [];
	
	fileStore.subscribe(value => {
		scanResult = value;
	});
	
	operationsStore.subscribe(value => {
		operations = value;
	});
	
	// Derived values
	$: totalFiles = scanResult?.files?.length || 0;
	$: totalSize = scanResult?.stats?.total_size || 0;
	$: fileTypes = scanResult?.stats?.file_types;
	$: hasData = totalFiles > 0;
	
	// File type summary
	$: fileTypesSummary = (() => {
		if (!fileTypes) return 'No files';
		const parts = [];
		if (fileTypes.images > 0) parts.push(`${fileTypes.images} images`);
		if (fileTypes.videos > 0) parts.push(`${fileTypes.videos} videos`);
		if (fileTypes.documents > 0) parts.push(`${fileTypes.documents} docs`);
		return parts.length > 0 ? parts.join(', ') : 'No files';
	})();
	
	// Date range
	$: dateRange = (() => {
		if (!scanResult?.files?.length) return 'No data';
		
		const dates = scanResult.files
			.map((f: any) => f.date_taken || f.modified_at || f.created_at)
			.filter((d: any) => d)
			.map((d: any) => new Date(d).getTime());
		
		if (!dates.length) return 'No dates';
		
		const min = new Date(Math.min(...dates)).getFullYear();
		const max = new Date(Math.max(...dates)).getFullYear();
		
		return min === max ? `${min}` : `${min} → ${max}`;
	})();
	
	// Format file size
	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 MB';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		if (bytes < 1024 * 1024 * 1024) return (bytes / 1024 / 1024).toFixed(1) + ' MB';
		return (bytes / 1024 / 1024 / 1024).toFixed(2) + ' GB';
	}
	
	function formatDate(isoString: string): string {
		const date = new Date(isoString);
		return date.toLocaleString();
	}
	
	// Resizable splitter for workspace/overview with LIMITS
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
		
		// STRICT LIMITS: Keep overview content always visible (min 250px = ~20% at 1080p)
		const minWorkspace = 45; // Min 45% for workspace
		const maxWorkspace = 75; // Max 75% for workspace (ensures 25% for overview)
		
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
	
	.info-section {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-5);
		background-color: var(--bg-subtle);
		display: flex;
		flex-direction: column;
		gap: var(--space-5);
		min-height: 200px;
	}
	
	.stats-grid {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: var(--space-4);
	}
</style>

<div class="home-page">
	<!-- Top: Layout Switcher + Action Buttons -->
	<LayoutSwitcher />
	
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
	
	<!-- Bottom: Overview + Activity (ALWAYS VISIBLE min 25%) -->
	<div class="info-section">
		<!-- Overview Stats -->
		<div class="stats-grid">
			<StatCard
				label="Total Files"
				value={totalFiles}
				meta={fileTypesSummary}
				icon={icons.files}
			/>
			<StatCard
				label="Total Size"
				value={formatBytes(totalSize)}
				meta={hasData ? `Across ${totalFiles} files` : 'No data yet'}
				icon={icons.storage}
			/>
			<StatCard
				label="Date Range"
				value={dateRange}
				meta={hasData ? 'From file metadata' : 'Scan to discover'}
				icon={icons.calendar}
			/>
			<StatCard
				label="Status"
				value={hasData ? 'Ready' : 'No data'}
				meta={hasData ? `${totalFiles} files loaded` : 'No workspace loaded'}
				icon={icons.diamond}
			/>
		</div>
		
		<!-- Recent Activity -->
		<Section title="Recent Activity">
			{#if operations.length === 0}
				<div style="padding: var(--space-5); text-align: center; color: var(--text-muted);">
					{#if hasData}
						<p>No operations yet. Use "Organize" button to organize files.</p>
					{:else}
						<p>No recent activity</p>
					{/if}
				</div>
			{:else}
				<div style="display: flex; flex-direction: column; gap: var(--space-3);">
					{#each operations.slice(0, 3) as operation, index}
						<div style="padding: var(--space-3); background-color: var(--panel); border-radius: var(--radius-md); border-left: 3px solid {operation.success ? 'var(--success)' : 'var(--danger)'};">
							<div style="display: flex; justify-content: space-between; align-items: start;">
								<div>
									<div style="font-size: var(--text-sm); font-weight: var(--weight-semibold); color: var(--text);">
										{operation.success ? '✓' : '⚠'} Operation #{operations.length - index}
									</div>
									<div style="font-size: var(--text-xs); color: var(--text-muted);">
										{formatDate(operation.timestamp)}
									</div>
								</div>
								<div style="display: flex; gap: var(--space-2); font-size: var(--text-xs);">
									<span style="color: var(--success);">✓ {operation.successful_count}</span>
									<span style="color: var(--danger);">✕ {operation.failed_count}</span>
									<span style="color: var(--text-muted);">{(operation.duration_ms / 1000).toFixed(2)}s</span>
								</div>
							</div>
						</div>
					{/each}
				</div>
			{/if}
		</Section>
	</div>
</div>
