<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';
	import { fileStore } from '$lib/stores/photoStore';
	import { icons } from '$lib/ui/icons';
	import Page from '$lib/ui/layout/Page.svelte';
	import Section from '$lib/ui/layout/Section.svelte';
	import Card from '$lib/ui/primitives/Card.svelte';
	import CommandButton from '$lib/ui/components/CommandButton.svelte';
	
	let isScanning = false;
	let scanResult: any = null;
	
	fileStore.subscribe(value => {
		scanResult = value;
	});
	
	$: hasFiles = scanResult?.files?.length > 0;
	
	async function selectFolder() {
		try {
			const selected = await open({
				directory: true,
				multiple: false,
				title: 'Select folder to organize'
			});
			
			if (selected) {
				await scanFolder(selected as string);
			}
		} catch (error) {
			console.error('Error selecting folder:', error);
		}
	}
	
	async function scanFolder(path: string) {
		isScanning = true;
		try {
			const result = await invoke('scan_folder', { path });
			fileStore.set(result);
			console.log('Scan complete:', result);
		} catch (error) {
			console.error('Scan error:', error);
		} finally {
			isScanning = false;
		}
	}
</script>

<Page title="Workspace" subtitle="Select and scan folders to organize">
	{#if !hasFiles}
		<!-- Empty State -->
		<Section title="Get Started">
			<Card padding="lg">
				<div style="text-align: center; padding: var(--space-7) 0;">
					<div style="font-size: 64px; margin-bottom: var(--space-4); opacity: 0.2; color: var(--text-muted);">{icons.folder}</div>
					<h3 style="font-size: var(--text-xl); font-weight: var(--weight-semibold); margin-bottom: var(--space-2); color: var(--text);">
						No workspace loaded
					</h3>
					<p style="font-size: var(--text-base); color: var(--text-muted); margin-bottom: var(--space-5); max-width: 500px; margin-left: auto; margin-right: auto;">
						Select a folder to begin organizing your files. Miktos Kosmos will scan and analyze the contents.
					</p>
					<div style="display: flex; gap: var(--space-3); justify-content: center;">
						<CommandButton
							variant="primary"
							label={isScanning ? 'Scanning...' : 'Select Folder'}
							description="Choose a directory to organize"
							icon={icons.folder}
							disabled={isScanning}
							onClick={selectFolder}
						/>
					</div>
				</div>
			</Card>
		</Section>
	{:else}
		<!-- Files Loaded -->
		<Section title="Workspace Info">
			<div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: var(--space-4);">
				<Card>
					<div style="padding: var(--space-2);">
						<div style="font-size: var(--text-sm); color: var(--text-muted); margin-bottom: var(--space-2);">Total Files</div>
						<div style="font-size: var(--text-2xl); font-weight: var(--weight-bold); color: var(--text);">
							{scanResult.files.length}
						</div>
					</div>
				</Card>
				
				<Card>
					<div style="padding: var(--space-2);">
						<div style="font-size: var(--text-sm); color: var(--text-muted); margin-bottom: var(--space-2);">File Types</div>
						<div style="font-size: var(--text-base); color: var(--text);">
							{#if scanResult.stats?.file_types}
								{scanResult.stats.file_types.images} images, {scanResult.stats.file_types.videos} videos
							{/if}
						</div>
					</div>
				</Card>
				
				<Card>
					<div style="padding: var(--space-2);">
						<div style="font-size: var(--text-sm); color: var(--text-muted); margin-bottom: var(--space-2);">Status</div>
						<div style="font-size: var(--text-base); font-weight: var(--weight-medium); color: var(--success);">
							{icons.check} Scanned
						</div>
					</div>
				</Card>
			</div>
		</Section>
		
		<Section title="Actions">
			<div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: var(--space-4);">
				<CommandButton
					variant="secondary"
					label="Scan Another Folder"
					description="Load a different directory"
					icon={icons.folder}
					onClick={selectFolder}
				/>
				<CommandButton
					variant="secondary"
					label="Clear Workspace"
					description="Remove current files"
					icon={icons.remove}
					onClick={() => fileStore.clear()}
				/>
			</div>
		</Section>
		
		<Section title="Files">
			<Card>
				<div style="padding: var(--space-2); color: var(--text-muted);">
					{scanResult.files.length} files loaded. File browser coming soon.
				</div>
			</Card>
		</Section>
	{/if}
</Page>
