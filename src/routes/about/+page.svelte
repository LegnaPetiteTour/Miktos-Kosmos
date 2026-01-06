<script lang="ts">
	import Page from '$lib/ui/layout/Page.svelte';
	import Section from '$lib/ui/layout/Section.svelte';
	import Card from '$lib/ui/primitives/Card.svelte';
</script>

<Page title="About" subtitle="Miktos Kosmos">
	<Section title="Project Info">
		<Card>
			<div style="padding: var(--space-4);">
				<h3 style="font-size: var(--text-xl); font-weight: var(--weight-bold); margin-bottom: var(--space-3); color: var(--text);">
					Miktos Kosmos
				</h3>
				<p style="font-size: var(--text-base); color: var(--text-muted); margin-bottom: var(--space-2); line-height: var(--line-height-relaxed);">
					Privacy-first file organization and analysis.
				</p>
				<p style="font-size: var(--text-sm); color: var(--text-subtle); margin-bottom: var(--space-4);">
					Version 0.1.0 (Alpha)
				</p>
				
				<div style="padding-top: var(--space-4); border-top: 1px solid var(--border);">
					<h4 style="font-size: var(--text-md); font-weight: var(--weight-semibold); margin-bottom: var(--space-3); color: var(--text);">
						Built With
					</h4>
					<div style="display: flex; flex-wrap: gap; gap: var(--space-2);">
						<span style="padding: 6px 12px; background-color: var(--bg-subtle); border-radius: var(--radius-md); font-size: var(--text-sm); color: var(--text-muted);">
							Tauri 2.0
						</span>
						<span style="padding: 6px 12px; background-color: var(--bg-subtle); border-radius: var(--radius-md); font-size: var(--text-sm); color: var(--text-muted);">
							SvelteKit
						</span>
						<span style="padding: 6px 12px; background-color: var(--bg-subtle); border-radius: var(--radius-md); font-size: var(--text-sm); color: var(--text-muted);">
							Rust
						</span>
						<span style="padding: 6px 12px; background-color: var(--bg-subtle); border-radius: var(--radius-md); font-size: var(--text-sm); color: var(--text-muted);">
							TypeScript
						</span>
					</div>
				</div>
			</div>
		</Card>
	</Section>
	
	<Section title="Principles">
		<div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: var(--space-4);">
			<Card>
				<div style="padding: var(--space-4);">
					<div style="font-size: 32px; margin-bottom: var(--space-3);">🔒</div>
					<h4 style="font-size: var(--text-md); font-weight: var(--weight-semibold); margin-bottom: var(--space-2); color: var(--text);">
						Local-only
					</h4>
					<p style="font-size: var(--text-sm); color: var(--text-muted);">
						Your files never leave your computer
					</p>
				</div>
			</Card>
			
			<Card>
				<div style="padding: var(--space-4);">
					<div style="font-size: 32px; margin-bottom: var(--space-3);">🌐</div>
					<h4 style="font-size: var(--text-md); font-weight: var(--weight-semibold); margin-bottom: var(--space-2); color: var(--text);">
						Open Source
					</h4>
					<p style="font-size: var(--text-sm); color: var(--text-muted);">
						Transparent and auditable code
					</p>
				</div>
			</Card>
			
			<Card>
				<div style="padding: var(--space-4);">
					<div style="font-size: 32px; margin-bottom: var(--space-3);">🛡️</div>
					<h4 style="font-size: var(--text-md); font-weight: var(--weight-semibold); margin-bottom: var(--space-2); color: var(--text);">
						Safe by Default
					</h4>
					<p style="font-size: var(--text-sm); color: var(--text-muted);">
						Copy mode protects your originals
					</p>
				</div>
			</Card>
		</div>
	</Section>
</Page>
