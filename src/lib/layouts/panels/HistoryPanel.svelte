<script lang="ts">
	import { operationsStore } from '$lib/stores/operationsStore';
	import type { OperationResult } from '$lib/types';
	
	let operations: OperationResult[] = [];
	
	operationsStore.subscribe(value => {
		operations = value;
	});
	
	function formatDate(isoString: string): string {
		const date = new Date(isoString);
		return date.toLocaleString();
	}
</script>

<style>
	.history-panel {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}
	
	.history-item {
		padding: var(--space-3);
		background-color: var(--bg-subtle);
		border-radius: var(--radius-md);
		border-left: 3px solid var(--success);
	}
	
	.history-item.failed {
		border-left-color: var(--danger);
	}
	
	.history-title {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: var(--text);
		margin-bottom: var(--space-1);
	}
	
	.history-time {
		font-size: var(--text-xs);
		color: var(--text-muted);
	}
	
	.history-stats {
		display: flex;
		gap: var(--space-3);
		margin-top: var(--space-2);
		font-size: var(--text-xs);
	}
	
	.empty-state {
		text-align: center;
		color: var(--text-muted);
		padding: var(--space-5);
	}
</style>

<div class="history-panel">
	{#if operations.length === 0}
		<div class="empty-state">
			<div style="font-size: 48px; margin-bottom: var(--space-2); opacity: 0.3;">📋</div>
			<p>No operations yet</p>
		</div>
	{:else}
		{#each operations as operation, index}
			<div class="history-item" class:failed={!operation.success}>
				<div class="history-title">
					{operation.success ? '✓' : '⚠'} Operation #{operations.length - index}
				</div>
				<div class="history-time">{formatDate(operation.timestamp)}</div>
				<div class="history-stats">
					<span style="color: var(--success);">✓ {operation.successful_count}</span>
					<span style="color: var(--danger);">✕ {operation.failed_count}</span>
					<span>{(operation.duration_ms / 1000).toFixed(2)}s</span>
				</div>
			</div>
		{/each}
	{/if}
</div>
