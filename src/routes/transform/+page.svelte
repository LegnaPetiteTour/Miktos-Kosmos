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

<Page title="Transform" subtitle="Organize files into structure">
	{#if !hasFiles}
		<!-- Empty State -->
		<Section title="No Files to Transform">
			<Card padding="lg">
				<div style="text-align: center; padding: var(--space-7) 0;">
					<div style="font-size: 48px; margin-bottom: var(--space-4);">⚡</div>
					<h3 style="font-size: var(--text-xl); font-weight: var(--weight-semibold); margin-bottom: var(--space-2); color: var(--text);">
						No workspace loaded
					</h3>
					<p style="font-size: var(--text-base); color: var(--text-muted); margin-bottom: var(--space-5);">
						Scan and analyze files before organizing them.
					</p>
				</div>
			</Card>
		</Section>
	{:else}
		<!-- Organization Preview -->
		<Section title="Organization Preview" description="How your files will be structured">
			<Card>
				<div style="padding: var(--space-4);">
					<div style="font-family: var(--font-mono); font-size: var(--text-sm); color: var(--text-muted); line-height: 1.8;">
						📁 organized/<br/>
						&nbsp;&nbsp;📁 2020/<br/>
						&nbsp;&nbsp;&nbsp;&nbsp;📁 01-January/<br/>
						&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;📷 IMG_0001.jpg<br/>
						&nbsp;&nbsp;📁 2023/<br/>
						&nbsp;&nbsp;&nbsp;&nbsp;📁 04-April/<br/>
						&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;📷 IMG_0002.jpg
					</div>
				</div>
			</Card>
		</Section>
		
		<!-- Operation Mode -->
		<Section title="Operation Mode" description="Choose how to organize files">
			<div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: var(--space-4);">
				<Card variant="subtle">
					<div style="padding: var(--space-4);">
						<div style="display: flex; align-items: center; gap: var(--space-3); margin-bottom: var(--space-3);">
							<span style="font-size: 28px;">📋</span>
							<h3 style="font-size: var(--text-lg); font-weight: var(--weight-semibold); color: var(--text);">
								Copy Files (Safe)
							</h3>
						</div>
						<p style="font-size: var(--text-sm); color: var(--text-muted); margin-bottom: var(--space-4);">
							Create organized copies while keeping originals untouched. Recommended for first-time use.
						</p>
						<CommandButton
							variant="primary"
							label="Copy & Organize"
							description={`${totalFiles} files`}
							onClick={() => console.log('Copy mode')}
						/>
					</div>
				</Card>
				
				<Card variant="subtle">
					<div style="padding: var(--space-4);">
						<div style="display: flex; align-items: center; gap: var(--space-3); margin-bottom: var(--space-3);">
							<span style="font-size: 28px;">🔀</span>
							<h3 style="font-size: var(--text-lg); font-weight: var(--weight-semibold); color: var(--text);">
								Move Files
							</h3>
						</div>
						<p style="font-size: var(--text-sm); color: var(--text-muted); margin-bottom: var(--space-4);">
							Relocate files to new structure. Original locations will be empty.
						</p>
						<CommandButton
							variant="secondary"
							label="Move & Organize"
							description={`${totalFiles} files`}
							onClick={() => console.log('Move mode')}
						/>
					</div>
				</Card>
			</div>
		</Section>
		
		<!-- Rules -->
		<Section title="Organization Rules">
			<Card>
				<div style="padding: var(--space-4);">
					<ul style="list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: var(--space-2);">
						<li style="display: flex; align-items: flex-start; gap: var(--space-2);">
							<span style="color: var(--success);">✓</span>
							<span style="color: var(--text-muted); font-size: var(--text-sm);">
								Organize by date (YYYY/MM-Month)
							</span>
						</li>
						<li style="display: flex; align-items: flex-start; gap: var(--space-2);">
							<span style="color: var(--success);">✓</span>
							<span style="color: var(--text-muted); font-size: var(--text-sm);">
								Preserve original filenames
							</span>
						</li>
						<li style="display: flex; align-items: flex-start; gap: var(--space-2);">
							<span style="color: var(--success);">✓</span>
							<span style="color: var(--text-muted); font-size: var(--text-sm);">
								Handle duplicates automatically
							</span>
						</li>
						<li style="display: flex; align-items: flex-start; gap: var(--space-2);">
							<span style="color: var(--success);">✓</span>
							<span style="color: var(--text-muted); font-size: var(--text-sm);">
								Log all operations for review
							</span>
						</li>
					</ul>
				</div>
			</Card>
		</Section>
	{/if}
</Page>
