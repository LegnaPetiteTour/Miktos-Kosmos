<script lang="ts">
	import { goto } from '$app/navigation';
	import { fileStore } from '$lib/stores/photoStore';
	import { icons } from '$lib/ui/icons';
	import Page from '$lib/ui/layout/Page.svelte';
	import Section from '$lib/ui/layout/Section.svelte';
	import StatCard from '$lib/ui/components/StatCard.svelte';
	import CommandButton from '$lib/ui/components/CommandButton.svelte';
	
	let scanResult: any = null;
	
	fileStore.subscribe(value => {
		scanResult = value;
	});
	
	// Derived values
	$: totalFiles = scanResult?.files?.length || 0;
	$: totalSize = scanResult?.stats?.total_size || 0;
	$: fileTypes = scanResult?.stats?.file_types;
	$: hasData = totalFiles > 0;
	
	// File type summary
	$: fileTypesSummary = (() => {
		if (!fileTypes) return 'No files';
		const parts = [];
		if (fileTypes.images > 0) parts.push(`${fileTypes.images} images`);
		if (fileTypes.videos > 0) parts.push(`${fileTypes.videos} videos`);
		if (fileTypes.documents > 0) parts.push(`${fileTypes.documents} docs`);
		return parts.length > 0 ? parts.join(', ') : 'No files';
	})();
	
	// Date range
	$: dateRange = (() => {
		if (!scanResult?.files?.length) return 'No data';
		
		const dates = scanResult.files
			.map((f: any) => f.date_taken || f.modified_at || f.created_at)
			.filter((d: any) => d)
			.map((d: any) => new Date(d).getTime());
		
		if (!dates.length) return 'No dates';
		
		const min = new Date(Math.min(...dates)).getFullYear();
		const max = new Date(Math.max(...dates)).getFullYear();
		
		return min === max ? `${min}` : `${min} → ${max}`;
	})();
	
	// Format file size
	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 MB';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		if (bytes < 1024 * 1024 * 1024) return (bytes / 1024 / 1024).toFixed(1) + ' MB';
		return (bytes / 1024 / 1024 / 1024).toFixed(2) + ' GB';
	}
</script>

<Page title="Command Center" subtitle="Local-first file organization and analysis">
	<!-- Overview Stats -->
	<Section title="Overview">
		<div style="display: grid; grid-template-columns: repeat(4, 1fr); gap: var(--space-4);">
			<StatCard
				label="Total Files"
				value={totalFiles}
				meta={fileTypesSummary}
				icon={icons.files}
			/>
			<StatCard
				label="Total Size"
				value={formatBytes(totalSize)}
				meta={hasData ? `Across ${totalFiles} files` : 'No data yet'}
				icon={icons.storage}
			/>
			<StatCard
				label="Date Range"
				value={dateRange}
				meta={hasData ? 'From file metadata' : 'Scan to discover'}
				icon={icons.calendar}
			/>
			<StatCard
				label="Status"
				value={hasData ? 'Ready' : 'No data'}
				meta={hasData ? `${totalFiles} files loaded` : 'No workspace loaded'}
				icon={icons.diamond}
			/>
		</div>
	</Section>
	
	<!-- Commands -->
	<Section title="Commands" description="Actions are previewed and reversible">
		<div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: var(--space-4);">
			<CommandButton
				variant="primary"
				label={hasData ? 'Scan Another Folder' : 'Scan Folder'}
				description="Select a directory to organize"
				icon={icons.folder}
				onClick={() => goto('/workspace')}
			/>
			<CommandButton
				variant="secondary"
				label="Analyze Files"
				description={hasData ? 'Find patterns and duplicates' : 'Scan files first'}
				icon={icons.search}
				disabled={!hasData}
				disabledReason={!hasData ? 'Scan a workspace first' : undefined}
				onClick={() => goto('/analyze')}
			/>
			<CommandButton
				variant="secondary"
				label="Create Structure"
				description={hasData ? 'Organize into folders' : 'Scan files first'}
				icon={icons.transform}
				disabled={!hasData}
				disabledReason={!hasData ? 'Run analysis first to validate dates and detect issues' : undefined}
				onClick={() => goto('/transform')}
			/>
		</div>
	</Section>
	
	<!-- Recent Activity -->
	<Section title="Recent Activity">
		<div style="padding: var(--space-5); text-align: center; color: var(--text-muted);">
			{#if hasData}
				<p>Last scanned: {totalFiles} files</p>
			{:else}
				<p>No recent activity</p>
			{/if}
		</div>
	</Section>
</Page>
