<script lang="ts">
	import FlexPanel from '$lib/components/panels/FlexPanel.svelte';
	import { fileStore } from '$lib/stores/photoStore';
	import FileBrowser from '$lib/layouts/panels/FileBrowser.svelte';
	import PreviewPanel from '$lib/layouts/panels/PreviewPanel.svelte';
	import MetadataPanel from '$lib/layouts/panels/MetadataPanel.svelte';
	import ToolsPanel from '$lib/layouts/panels/ToolsPanel.svelte';
	
	let scanResult: any = null;
	
	fileStore.subscribe(value => {
		scanResult = value;
	});
	
	$: hasFiles = scanResult?.files?.length > 0;
</script>

<style>
	.flex-workspace {
		display: flex;
		gap: 6px;
		padding: 6px;
		height: 100%;
		overflow: hidden;
		background-color: var(--bg);
	}
	
	.panel-column {
		display: flex;
		flex-direction: column;
		gap: 6px;
		min-width: 200px;
	}
</style>

<div class="flex-workspace">
	<!-- Left Column: Files -->
	<div class="panel-column" style="flex: 2;">
		<FlexPanel title="Files" minWidth={300} defaultFlex={1}>
			<FileBrowser />
		</FlexPanel>
	</div>
	
	<!-- Center Column: Tools (conditional) -->
	{#if hasFiles}
		<div class="panel-column" style="flex: 2;">
			<FlexPanel title="Tools" minWidth={300} defaultFlex={1}>
				<ToolsPanel />
			</FlexPanel>
		</div>
	{/if}
	
	<!-- Right Column: Preview + Metadata -->
	<div class="panel-column" style="flex: 1.5;">
		<FlexPanel title="Preview" minWidth={250} minHeight={200} defaultFlex={1.5}>
			<PreviewPanel />
		</FlexPanel>
		
		<FlexPanel title="Metadata" minWidth={250} minHeight={150} defaultFlex={1}>
			<MetadataPanel />
		</FlexPanel>
	</div>
</div>
