<script lang="ts">
	import Page from '$lib/ui/layout/Page.svelte';
	import Section from '$lib/ui/layout/Section.svelte';
	import Card from '$lib/ui/primitives/Card.svelte';
</script>

<Page title="Learn" subtitle="Documentation and guides">
	<Section title="Getting Started">
		<Card>
			<div style="padding: var(--space-4);">
				<h3 style="font-size: var(--text-lg); font-weight: var(--weight-semibold); margin-bottom: var(--space-3); color: var(--text);">
					📚 Quick Start Guide
				</h3>
				<ol style="margin: 0; padding-left: var(--space-5); display: flex; flex-direction: column; gap: var(--space-2);">
					<li style="color: var(--text-muted);">Select a folder in Workspace</li>
					<li style="color: var(--text-muted);">Analyze files to detect patterns</li>
					<li style="color: var(--text-muted);">Choose organization structure in Transform</li>
					<li style="color: var(--text-muted);">Review changes before applying</li>
				</ol>
			</div>
		</Card>
	</Section>
	
	<Section title="Features">
		<div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: var(--space-4);">
			<Card>
				<div style="padding: var(--space-4);">
					<h4 style="font-size: var(--text-md); font-weight: var(--weight-semibold); margin-bottom: var(--space-2); color: var(--text);">
						🔒 Privacy First
					</h4>
					<p style="font-size: var(--text-sm); color: var(--text-muted); line-height: var(--line-height-relaxed);">
						All processing happens locally on your machine. No cloud uploads, no tracking.
					</p>
				</div>
			</Card>
			
			<Card>
				<div style="padding: var(--space-4);">
					<h4 style="font-size: var(--text-md); font-weight: var(--weight-semibold); margin-bottom: var(--space-2); color: var(--text);">
						📂 File Agnostic
					</h4>
					<p style="font-size: var(--text-sm); color: var(--text-muted); line-height: var(--line-height-relaxed);">
						Works with images, videos, documents, and any file type.
					</p>
				</div>
			</Card>
		</div>
	</Section>
</Page>
