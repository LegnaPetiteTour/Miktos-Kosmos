<script lang="ts">
	import { fileStore } from '$lib/stores/photoStore';
	import Page from '$lib/ui/layout/Page.svelte';
	import Section from '$lib/ui/layout/Section.svelte';
	import Card from '$lib/ui/primitives/Card.svelte';
	import CommandButton from '$lib/ui/components/CommandButton.svelte';
	
	let scanResult: any = null;
	
	fileStore.subscribe(value => {
		scanResult = value;
	});
	
	$: hasFiles = scanResult?.files?.length > 0;
	$: totalFiles = scanResult?.files?.length || 0;
</script>

<Page title="Analyze" subtitle="Detect patterns, duplicates, and issues">
	{#if !hasFiles}
		<!-- Empty State -->
		<Section title="No Files to Analyze">
			<Card padding="lg">
				<div style="text-align: center; padding: var(--space-7) 0;">
					<div style="font-size: 48px; margin-bottom: var(--space-4); opacity: 0.3;">◉</div>
					<h3 style="font-size: var(--text-xl); font-weight: var(--weight-semibold); margin-bottom: var(--space-2); color: var(--text);">
						No workspace loaded
					</h3>
					<p style="font-size: var(--text-base); color: var(--text-muted); margin-bottom: var(--space-5);">
						Scan a folder first to analyze your files.
					</p>
				</div>
			</Card>
		</Section>
	{:else}
		<!-- Analysis Options -->
		<Section title="Analysis Options" description="Choose what to analyze">
			<div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: var(--space-4);">
				<Card>
					<div style="padding: var(--space-2);">
						<h3 style="font-size: var(--text-lg); font-weight: var(--weight-semibold); margin-bottom: var(--space-2); color: var(--text);">
							◎ Find Duplicates
						</h3>
						<p style="font-size: var(--text-sm); color: var(--text-muted); margin-bottom: var(--space-4);">
							Detect exact and similar files based on content hash
						</p>
						<CommandButton
							variant="secondary"
							label="Scan for Duplicates"
							description={`Analyze ${totalFiles} files`}
							onClick={() => console.log('Find duplicates')}
						/>
					</div>
				</Card>
				
				<Card>
					<div style="padding: var(--space-2);">
						<h3 style="font-size: var(--text-lg); font-weight: var(--weight-semibold); margin-bottom: var(--space-2); color: var(--text);">
							◷ Check Dates
						</h3>
						<p style="font-size: var(--text-sm); color: var(--text-muted); margin-bottom: var(--space-4);">
							Verify metadata dates and detect missing information
						</p>
						<CommandButton
							variant="secondary"
							label="Validate Dates"
							description={`Check ${totalFiles} files`}
							onClick={() => console.log('Check dates')}
						/>
					</div>
				</Card>
				
				<Card>
					<div style="padding: var(--space-2);">
						<h3 style="font-size: var(--text-lg); font-weight: var(--weight-semibold); margin-bottom: var(--space-2); color: var(--text);">
							◈ Generate Report
						</h3>
						<p style="font-size: var(--text-sm); color: var(--text-muted); margin-bottom: var(--space-4);">
							Create a comprehensive analysis report
						</p>
						<CommandButton
							variant="secondary"
							label="Create Report"
							description="Export analysis results"
							onClick={() => console.log('Generate report')}
						/>
					</div>
				</Card>
				
				<Card>
					<div style="padding: var(--space-2);">
						<h3 style="font-size: var(--text-lg); font-weight: var(--weight-semibold); margin-bottom: var(--space-2); color: var(--text);">
							⚠ Find Issues
						</h3>
						<p style="font-size: var(--text-sm); color: var(--text-muted); margin-bottom: var(--space-4);">
							Detect corrupted files and naming problems
						</p>
						<CommandButton
							variant="secondary"
							label="Scan Issues"
							description="Check file integrity"
							onClick={() => console.log('Find issues')}
						/>
					</div>
				</Card>
			</div>
		</Section>
		
		<Section title="Results">
			<Card>
				<div style="padding: var(--space-4); text-align: center; color: var(--text-muted);">
					Run an analysis to see results here
				</div>
			</Card>
		</Section>
	{/if}
</Page>
