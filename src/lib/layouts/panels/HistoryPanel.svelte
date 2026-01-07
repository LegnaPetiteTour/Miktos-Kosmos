<script lang="ts">
	import { historyStore } from '$lib/stores/historyStore';
	import { operationsStore } from '$lib/stores/operationsStore';
	import type { HistoryEntry } from '$lib/stores/historyStore';
	import type { OperationResult } from '$lib/types';
	
	let history: HistoryEntry[] = [];
	let operations: OperationResult[] = [];
	
	historyStore.subscribe(value => {
		history = value;
	});
	
	operationsStore.subscribe(value => {
		operations = value;
	});
	
	// Accordion state
	let expandedSections: Record<string, boolean> = {
		history: true,
		operations: true,
		system: false
	};
	
	function toggleSection(section: string) {
		expandedSections[section] = !expandedSections[section];
	}
	
	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 MB';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		if (bytes < 1024 * 1024 * 1024) return (bytes / 1024 / 1024).toFixed(1) + ' MB';
		return (bytes / 1024 / 1024 / 1024).toFixed(2) + ' GB';
	}
	
	function formatDate(isoString: string): string {
		const date = new Date(isoString);
		return date.toLocaleString('en-US', {
			year: 'numeric',
			month: '2-digit',
			day: '2-digit',
			hour: '2-digit',
			minute: '2-digit'
		});
	}
	
	function formatDateRange(start?: string, end?: string): string {
		if (!start || !end) return 'No dates';
		const startYear = new Date(start).getFullYear();
		const endYear = new Date(end).getFullYear();
		return startYear === endYear ? `${startYear}` : `${startYear} → ${endYear}`;
	}
	
	function getStatusIcon(status: string): string {
		switch(status) {
			case 'success': return '✓';
			case 'warning': return '⚠';
			case 'error': return '✕';
			default: return '•';
		}
	}
	
	function getStatusColor(status: string): string {
		switch(status) {
			case 'success': return 'var(--success)';
			case 'warning': return 'var(--warning)';
			case 'error': return 'var(--danger)';
			default: return 'var(--text-muted)';
		}
	}
</script>

<style>
	.history-container {
		display: flex;
		flex-direction: column;
		gap: 0;
		height: 100%;
		overflow-y: auto;
		padding: 0;
		background-color: var(--bg-subtle);
	}
	
	.history-section {
		border-bottom: 1px solid var(--panel-border);
	}
	
	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-3) var(--space-4);
		background-color: var(--bg-subtle);
		cursor: pointer;
		user-select: none;
		transition: background-color var(--transition-fast);
	}
	
	.section-header:hover {
		background-color: var(--panel);
	}
	
	.section-header.expanded {
		background-color: var(--panel);
	}
	
	.section-title {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: var(--text);
	}
	
	.section-icon {
		font-size: 10px;
		color: var(--text-muted);
		transition: transform var(--transition-fast);
	}
	
	.section-icon.expanded {
		transform: rotate(90deg);
	}
	
	.section-content {
		padding: var(--space-4);
		background-color: var(--panel);
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}
	
	.history-item {
		padding: var(--space-3);
		background-color: var(--bg-subtle);
		border-radius: var(--radius-md);
		border-left: 3px solid;
		transition: all var(--transition-fast);
	}
	
	.history-item:hover {
		background: rgba(255, 255, 255, 0.03);
		transform: translateX(2px);
	}
	
	.history-header {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		margin-bottom: var(--space-2);
	}
	
	.history-icon {
		font-size: 14px;
	}
	
	.history-date {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: var(--text);
		flex: 1;
	}
	
	.history-info {
		display: flex;
		flex-wrap: wrap;
		gap: var(--space-2);
		font-size: var(--text-xs);
		color: var(--text-muted);
	}
	
	.history-info-item {
		display: flex;
		align-items: center;
		gap: var(--space-1);
	}
	
	.history-path {
		font-size: var(--text-xs);
		color: var(--text-muted);
		font-family: monospace;
		margin-top: var(--space-1);
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}
	
	.empty-state {
		padding: var(--space-5);
		text-align: center;
		color: var(--text-muted);
		font-size: var(--text-sm);
	}
	
	.operation-item {
		padding: var(--space-3);
		background-color: var(--bg-subtle);
		border-radius: var(--radius-md);
		border-left: 3px solid var(--success);
		transition: all var(--transition-fast);
	}
	
	.operation-item.failed {
		border-left-color: var(--danger);
	}
	
	.operation-item:hover {
		background: rgba(255, 255, 255, 0.03);
		transform: translateX(2px);
	}
	
	.operation-header {
		display: flex;
		justify-content: space-between;
		align-items: start;
		margin-bottom: var(--space-2);
	}
	
	.operation-title {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: var(--text);
		display: flex;
		align-items: center;
		gap: var(--space-2);
	}
	
	.operation-status {
		font-size: 14px;
	}
	
	.operation-time {
		font-size: var(--text-xs);
		color: var(--text-muted);
	}
	
	.operation-stats {
		display: flex;
		gap: var(--space-3);
		font-size: var(--text-xs);
	}
	
	.stat-success {
		color: var(--success);
	}
	
	.stat-failed {
		color: var(--danger);
	}
	
	.stat-duration {
		color: var(--text-muted);
	}
	
	.system-info {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}
	
	.stat-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: var(--space-2);
		border-radius: var(--radius-sm);
		transition: background-color var(--transition-fast);
	}
	
	.stat-row:hover {
		background: rgba(255, 255, 255, 0.03);
	}
	
	.stat-label {
		font-size: var(--text-xs);
		color: var(--text-muted);
	}
	
	.stat-value {
		font-size: var(--text-sm);
		color: var(--text);
		font-weight: var(--weight-medium);
	}
</style>

<div class="history-container">
	<!-- HISTORY SECTION -->
	<div class="history-section">
		<div 
			class="section-header"
			class:expanded={expandedSections.history}
			on:click={() => toggleSection('history')}
		>
			<h3 class="section-title">History</h3>
			<span class="section-icon" class:expanded={expandedSections.history}>▶</span>
		</div>
		
		{#if expandedSections.history}
			<div class="section-content">
				{#if history.length === 0}
					<div class="empty-state">
						No workspace history yet. Scan a folder to begin.
					</div>
				{:else}
					{#each history as entry}
						<div 
							class="history-item" 
							style="border-left-color: {getStatusColor(entry.status)}"
						>
							<div class="history-header">
								<span class="history-icon" style="color: {getStatusColor(entry.status)}">
									{getStatusIcon(entry.status)}
								</span>
								<span class="history-date">{formatDate(entry.timestamp)}</span>
							</div>
							
							<div class="history-info">
								<span class="history-info-item">📁 {entry.total_files} files</span>
								<span class="history-info-item">💾 {formatBytes(entry.total_size)}</span>
								<span class="history-info-item">📅 {formatDateRange(entry.date_range_start, entry.date_range_end)}</span>
								{#if entry.errors > 0 || entry.warnings > 0}
									<span class="history-info-item" style="color: var(--danger)">
										⚠ {entry.errors} errors, {entry.warnings} warnings
									</span>
								{:else}
									<span class="history-info-item" style="color: var(--success)">
										✓ No errors
									</span>
								{/if}
							</div>
							
							<div class="history-path" title={entry.folder_path}>
								{entry.folder_path}
							</div>
						</div>
					{/each}
				{/if}
			</div>
		{/if}
	</div>
	
	<!-- OPERATIONS HISTORY SECTION -->
	<div class="history-section">
		<div 
			class="section-header"
			class:expanded={expandedSections.operations}
			on:click={() => toggleSection('operations')}
		>
			<h3 class="section-title">Operations</h3>
			<span class="section-icon" class:expanded={expandedSections.operations}>▶</span>
		</div>
		
		{#if expandedSections.operations}
			<div class="section-content">
				{#if operations.length === 0}
					<div class="empty-state">
						No operations yet. Use tools to process files.
					</div>
				{:else}
					{#each operations as operation, index}
						<div class="operation-item" class:failed={!operation.success}>
							<div class="operation-header">
								<div>
									<div class="operation-title">
										<span class="operation-status">{operation.success ? '✓' : '⚠'}</span>
										<span>Operation #{operations.length - index}</span>
									</div>
									<div class="operation-time">{formatDate(operation.timestamp)}</div>
								</div>
							</div>
							
							<div class="operation-stats">
								<span class="stat-success">✓ {operation.successful_count}</span>
								<span class="stat-failed">✕ {operation.failed_count}</span>
								<span class="stat-duration">{(operation.duration_ms / 1000).toFixed(2)}s</span>
							</div>
						</div>
					{/each}
				{/if}
			</div>
		{/if}
	</div>
	
	<!-- SYSTEM INFO SECTION -->
	<div class="history-section">
		<div 
			class="section-header"
			class:expanded={expandedSections.system}
			on:click={() => toggleSection('system')}
		>
			<h3 class="section-title">System</h3>
			<span class="section-icon" class:expanded={expandedSections.system}>▶</span>
		</div>
		
		{#if expandedSections.system}
			<div class="section-content">
				<div class="system-info">
					<div class="stat-row">
						<span class="stat-label">Workspaces</span>
						<span class="stat-value">{history.length} total</span>
					</div>
					
					<div class="stat-row">
						<span class="stat-label">Operations</span>
						<span class="stat-value">{operations.length} total</span>
					</div>
					
					<div class="stat-row">
						<span class="stat-label">Mode</span>
						<span class="stat-value">Local-only</span>
					</div>
					
					<div class="stat-row">
						<span class="stat-label">Safety</span>
						<span class="stat-value">Safe mode</span>
					</div>
				</div>
			</div>
		{/if}
	</div>
</div>
