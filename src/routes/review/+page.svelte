<script lang="ts">
	import Page from '$lib/ui/layout/Page.svelte';
	import Section from '$lib/ui/layout/Section.svelte';
	import Card from '$lib/ui/primitives/Card.svelte';
	import CommandButton from '$lib/ui/components/CommandButton.svelte';
</script>

<Page title="Review" subtitle="Audit trail and operation history">
	<!-- Operation History -->
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
	
	<!-- Quick Actions -->
	<Section title="Review Tools">
		<div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: var(--space-4);">
			<CommandButton
				variant="secondary"
				label="Undo Last Operation"
				description="Reverse the most recent change"
				icon="↩️"
				disabled={true}
				disabledReason="No operations to undo"
				onClick={() => console.log('Undo')}
			/>
			<CommandButton
				variant="secondary"
				label="View Changes"
				description="See before/after comparison"
				icon="👁️"
				disabled={true}
				disabledReason="No operations yet"
				onClick={() => console.log('View changes')}
			/>
			<CommandButton
				variant="secondary"
				label="Export Report"
				description="Save operation log"
				icon="📄"
				disabled={true}
				disabledReason="No operations yet"
				onClick={() => console.log('Export')}
			/>
		</div>
	</Section>
	
	<!-- Trust Message -->
	<Section title="Trust & Safety">
		<Card variant="info">
			<div style="padding: var(--space-4);">
				<h3 style="font-size: var(--text-lg); font-weight: var(--weight-semibold); margin-bottom: var(--space-2); color: var(--info-text);">
					🛡️ Every operation is logged
				</h3>
				<p style="font-size: var(--text-sm); color: var(--info-text); line-height: var(--line-height-relaxed);">
					Miktos Kosmos records every file operation in detail. You can review, undo, and export operation logs at any time. Default mode is "Copy" to keep your originals safe.
				</p>
			</div>
		</Card>
	</Section>
</Page>
