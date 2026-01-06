<script lang="ts">
	import { goto } from '$app/navigation';
	import CommandButton from '$lib/ui/components/CommandButton.svelte';
	import { icons } from '$lib/ui/icons';
	import { fileStore } from '$lib/stores/photoStore';
	
	let scanResult: any = null;
	
	fileStore.subscribe(value => {
		scanResult = value;
	});
	
	$: hasFiles = scanResult?.files?.length > 0;
</script>

<style>
	.tools-panel {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}
	
	.tool-section {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}
	
	.section-title {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: var(--text);
		margin-bottom: var(--space-2);
	}
</style>

<div class="tools-panel">
	<div class="tool-section">
		<div class="section-title">Actions</div>
		<CommandButton
			variant="primary"
			label="Organize Files"
			description="Create folder structure"
			icon={icons.transform}
			disabled={!hasFiles}
			disabledReason={!hasFiles ? 'Scan files first' : undefined}
			onClick={() => goto('/organize')}
		/>
		<CommandButton
			variant="secondary"
			label="Analyze Files"
			description="Find duplicates and issues"
			icon={icons.search}
			disabled={!hasFiles}
			disabledReason={!hasFiles ? 'Scan files first' : undefined}
			onClick={() => console.log('Analyze')}
		/>
	</div>
</div>
