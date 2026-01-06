<script lang="ts">
	import Page from '$lib/ui/layout/Page.svelte';
	import Section from '$lib/ui/layout/Section.svelte';
	import Card from '$lib/ui/primitives/Card.svelte';
</script>

<Page title="Settings" subtitle="Configure your preferences">
	<Section title="Appearance">
		<Card>
			<div style="padding: var(--space-4); color: var(--text-muted);">
				Use the theme toggle in the sidebar to switch between light and dark mode.
			</div>
		</Card>
	</Section>
	
	<Section title="File Operations">
		<Card>
			<div style="padding: var(--space-4);">
				<h4 style="font-size: var(--text-md); font-weight: var(--weight-semibold); margin-bottom: var(--space-2); color: var(--text);">
					Default Operation Mode
				</h4>
				<p style="font-size: var(--text-sm); color: var(--text-muted); margin-bottom: var(--space-3);">
					Currently: Copy (Safe Mode)
				</p>
				<p style="font-size: var(--text-sm); color: var(--text-subtle);">
					Settings panel coming soon
				</p>
			</div>
		</Card>
	</Section>
</Page>
