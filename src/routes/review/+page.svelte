<script lang="ts">
	import { operationsStore } from '$lib/stores/operationsStore';
	import type { OperationResult } from '$lib/types';
	import Page from '$lib/ui/layout/Page.svelte';
	import Section from '$lib/ui/layout/Section.svelte';
	import Card from '$lib/ui/primitives/Card.svelte';
	import CommandButton from '$lib/ui/components/CommandButton.svelte';
	
	let operations: OperationResult[] = [];
	
	operationsStore.subscribe(value => {
		operations = value;
	});
	
	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
	}
	
	function formatDate(isoString: string): string {
		const date = new Date(isoString);
		return date.toLocaleString();
	}
</script>

<Page title="Review" subtitle="Audit trail and operation history">
	{#if operations.length === 0}
		<!-- Empty State -->
		<Section title="Operation History" description="All changes are logged and reversible">
			<Card padding="lg">
				<div style="text-align: center; padding: var(--space-7) 0;">
					<div style="font-size: 48px; margin-bottom: var(--space-4);">📋</div>
					<h3 style="font-size: var(--text-xl); font-weight: var(--weight-semibold); margin-bottom: var(--space-2); color: var(--text);">
						No operations yet
					</h3>
					<p style="font-size: var(--text-base); color: var(--text-muted);">
						Operations will appear here after you organize files
					</p>
				</div>
			</Card>
		</Section>
	{:else}
		<!-- Operations List -->
		<Section title="Operation History" description="Recent file operations">
			<div style="display: flex; flex-direction: column; gap: var(--space-4);">
				{#each operations as operation, index}
					<Card variant={operation.success ? 'subtle' : 'danger'}>
						<div style="padding: var(--space-4);">
							<div style="display: flex; justify-content: space-between; align-items: start; margin-bottom: var(--space-3);">
								<div>
									<h3 style="font-size: var(--text-lg); font-weight: var(--weight-semibold); color: var(--text);">
										{operation.success ? '✓' : '⚠'} Operation #{operations.length - index}
									</h3>
									<p style="font-size: var(--text-sm); color: var(--text-muted);">
										{formatDate(operation.timestamp)}
									</p>
								</div>
								<div style="text-align: right;">
									<div style="font-size: var(--text-sm); color: var(--text-muted);">Duration</div>
									<div style="font-size: var(--text-md); font-weight: var(--weight-semibold); color: var(--text);">
										{(operation.duration_ms / 1000).toFixed(2)}s
									</div>
								</div>
							</div>
							
							<div style="display: grid; grid-template-columns: repeat(4, 1fr); gap: var(--space-3); margin-bottom: var(--space-3);">
								<div>
									<div style="font-size: var(--text-xs); color: var(--text-muted);">Successful</div>
									<div style="font-size: var(--text-lg); font-weight: var(--weight-bold); color: var(--success);">
										{operation.successful_count}
									</div>
								</div>
								<div>
									<div style="font-size: var(--text-xs); color: var(--text-muted);">Failed</div>
									<div style="font-size: var(--text-lg); font-weight: var(--weight-bold); color: var(--danger);">
										{operation.failed_count}
									</div>
								</div>
								<div>
									<div style="font-size: var(--text-xs); color: var(--text-muted);">Skipped</div>
									<div style="font-size: var(--text-lg); font-weight: var(--weight-bold); color: var(--text-muted);">
										{operation.skipped_count}
									</div>
								</div>
								<div>
									<div style="font-size: var(--text-xs); color: var(--text-muted);">Size Processed</div>
									<div style="font-size: var(--text-lg); font-weight: var(--weight-bold); color: var(--text);">
										{formatBytes(operation.total_size_processed)}
									</div>
								</div>
							</div>
							
							{#if operation.failed_count > 0}
								<details style="margin-top: var(--space-3);">
									<summary style="cursor: pointer; font-size: var(--text-sm); font-weight: var(--weight-medium); color: var(--text); margin-bottom: var(--space-2);">
										View {operation.failed_count} Failed Operations
									</summary>
									<div style="max-height: 200px; overflow-y: auto; font-size: var(--text-sm); color: var(--text-muted); margin-top: var(--space-2);">
										{#each operation.operations.filter(op => op.status === 'Failed') as failedOp}
											<div style="margin-bottom: var(--space-2); padding: var(--space-2); background: var(--danger-bg); border-radius: 6px;">
												<div style="font-family: var(--font-mono); font-size: var(--text-xs); color: var(--danger-text); word-break: break-all;">
													{failedOp.source_path}
												</div>
												<div style="color: var(--danger-text); margin-top: var(--space-1);">
													{failedOp.error_message || 'Unknown error'}
												</div>
											</div>
										{/each}
									</div>
								</details>
							{/if}
						</div>
					</Card>
				{/each}
			</div>
		</Section>
		
		<!-- Clear History -->
		<Section title="Actions">
			<CommandButton
				variant="danger"
				label="Clear History"
				description="Remove all operation logs"
				onClick={() => operationsStore.clear()}
			/>
		</Section>
	{/if}
	
	<!-- Trust Message -->
	<Section title="Trust & Safety">
		<Card variant="info">
			<div style="padding: var(--space-4);">
				<h3 style="font-size: var(--text-lg); font-weight: var(--weight-semibold); margin-bottom: var(--space-2); color: var(--info-text);">
					🛡️ Every operation is logged
				</h3>
				<p style="font-size: var(--text-sm); color: var(--info-text); line-height: var(--line-height-relaxed);">
					Miktos Kosmos records every file operation in detail. You can review operation logs at any time. Default mode is "Copy" to keep your originals safe.
				</p>
			</div>
		</Card>
	</Section>
</Page>
