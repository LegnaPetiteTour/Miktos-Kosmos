<script lang="ts">
	import { fileStore } from '$lib/stores/photoStore';
	import { onMount } from 'svelte';
	
	let scanResult: any = null;
	
	// Subscribe to file store
	fileStore.subscribe(value => {
		scanResult = value;
	});
	
	// Derived values
	$: totalFiles = scanResult?.files?.length || 0;
	$: totalSize = scanResult?.stats?.total_size || 0;
	$: fileTypes = scanResult?.stats?.file_types;
	
	// Calculate date range
	$: dateRange = (() => {
		if (!scanResult?.files?.length) return '—';
		
		const dates = scanResult.files
			.map((f: any) => f.date_taken || f.modified_at || f.created_at)
			.filter((d: any) => d)
			.map((d: any) => new Date(d).getTime());
		
		if (!dates.length) return 'No dates';
		
		const min = new Date(Math.min(...dates));
		const max = new Date(Math.max(...dates));
		
		const minStr = min.toLocaleDateString();
		const maxStr = max.toLocaleDateString();
		
		return minStr === maxStr ? minStr : `${minStr} - ${maxStr}`;
	})();
	
	// Format file size
	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 MB';
		if (bytes < 1024) return bytes + ' B';
		if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
		if (bytes < 1024 * 1024 * 1024) return (bytes / 1024 / 1024).toFixed(1) + ' MB';
		return (bytes / 1024 / 1024 / 1024).toFixed(2) + ' GB';
	}
	
	// Get file type breakdown
	$: fileTypeBreakdown = (() => {
		if (!fileTypes) return [];
		
		const types = [];
		if (fileTypes.images > 0) types.push({ label: 'Images', count: fileTypes.images, color: 'bg-blue-100 text-blue-800' });
		if (fileTypes.videos > 0) types.push({ label: 'Videos', count: fileTypes.videos, color: 'bg-purple-100 text-purple-800' });
		if (fileTypes.documents > 0) types.push({ label: 'Docs', count: fileTypes.documents, color: 'bg-green-100 text-green-800' });
		if (fileTypes.audio > 0) types.push({ label: 'Audio', count: fileTypes.audio, color: 'bg-yellow-100 text-yellow-800' });
		if (fileTypes.archives > 0) types.push({ label: 'Archives', count: fileTypes.archives, color: 'bg-orange-100 text-orange-800' });
		if (fileTypes.other > 0) types.push({ label: 'Other', count: fileTypes.other, color: 'bg-gray-100 text-gray-800' });
		
		return types;
	})();
	
	// Check if we have data to analyze
	$: hasData = totalFiles > 0;
	
	onMount(() => {
		console.log('Miktos Kosmos is ready!');
	});
</script>

<div class="p-8">
	<div class="max-w-7xl mx-auto space-y-6">
		<!-- Header -->
		<div>
			<h1 class="text-3xl font-bold text-gray-900">Home</h1>
			<p class="text-gray-600 mt-2">Orientation & Control</p>
		</div>
		
		<!-- Stats Grid - Horizontal Layout -->
		<div class="grid grid-cols-1 md:grid-cols-4 gap-4">
			<!-- Total Files -->
			<div class="card">
				<div class="text-sm text-gray-600 mb-1">Total Files</div>
				<div class="text-3xl font-bold text-gray-900 mb-2">{totalFiles}</div>
				<div class="flex flex-wrap gap-1">
					{#each fileTypeBreakdown as type}
						<span class="text-xs px-2 py-1 rounded-full {type.color}">
							{type.count} {type.label}
						</span>
					{/each}
					{#if fileTypeBreakdown.length === 0}
						<span class="text-xs text-gray-400">No files</span>
					{/if}
				</div>
			</div>
			
			<!-- Total Size -->
			<div class="card">
				<div class="text-sm text-gray-600 mb-1">Total Size</div>
				<div class="text-3xl font-bold text-gray-900">{formatBytes(totalSize)}</div>
				<div class="text-xs text-gray-500 mt-2">
					{hasData ? `Across ${totalFiles} files` : 'No data yet'}
				</div>
			</div>
			
			<!-- Date Range -->
			<div class="card">
				<div class="text-sm text-gray-600 mb-1">Date Span</div>
				<div class="text-lg font-bold text-gray-900 leading-tight">{dateRange}</div>
				<div class="text-xs text-gray-500 mt-2">
					{hasData ? 'From file metadata' : 'Scan to discover'}
				</div>
			</div>
			
			<!-- Status -->
			<div class="card">
				<div class="text-sm text-gray-600 mb-1">Status</div>
				<div class="flex items-center gap-2 mb-2">
					<span class="w-3 h-3 bg-green-500 rounded-full"></span>
					<span class="text-lg font-bold text-gray-900">Ready</span>
				</div>
				<div class="text-xs text-gray-500">
					{hasData ? `${totalFiles} files loaded` : 'No workspace loaded'}
				</div>
			</div>
		</div>
		
		<!-- Primary Actions - Horizontal Cards -->
		<div class="card">
			<h2 class="text-xl font-semibold text-gray-900 mb-4">Quick Actions</h2>
			<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
				<a 
					href="/workspace" 
					class="group p-4 bg-blue-50 hover:bg-blue-100 rounded-lg transition-all border-2 border-blue-200 hover:border-blue-300"
				>
					<div class="text-blue-600 font-semibold mb-1">
						{hasData ? '🔄 Scan Another Folder' : '📁 Scan Folder'}
					</div>
					<div class="text-sm text-blue-700">
						Select a directory to organize
					</div>
				</a>
				
				<button 
					on:click={() => window.location.href = '/analyze'}
					disabled={!hasData}
					class="group p-4 rounded-lg transition-all border-2 text-left {hasData 
						? 'bg-purple-50 hover:bg-purple-100 border-purple-200 hover:border-purple-300 cursor-pointer' 
						: 'bg-gray-50 border-gray-200 cursor-not-allowed opacity-50'}"
				>
					<div class="font-semibold mb-1 {hasData ? 'text-purple-600' : 'text-gray-400'}">
						🔍 Analyze Files {hasData ? `(${totalFiles})` : ''}
					</div>
					<div class="text-sm {hasData ? 'text-purple-700' : 'text-gray-400'}">
						{hasData ? 'Find patterns and duplicates' : 'Scan files first'}
					</div>
				</button>
				
				<button 
					on:click={() => window.location.href = '/transform'}
					disabled={!hasData}
					class="group p-4 rounded-lg transition-all border-2 text-left {hasData 
						? 'bg-green-50 hover:bg-green-100 border-green-200 hover:border-green-300 cursor-pointer' 
						: 'bg-gray-50 border-gray-200 cursor-not-allowed opacity-50'}"
				>
					<div class="font-semibold mb-1 {hasData ? 'text-green-600' : 'text-gray-400'}">
						⚡ Create Structure {hasData ? `(${totalFiles})` : ''}
					</div>
					<div class="text-sm {hasData ? 'text-green-700' : 'text-gray-400'}">
						{hasData ? 'Organize into folders' : 'Scan files first'}
					</div>
				</button>
			</div>
		</div>
		
		<!-- System Status - Horizontal -->
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
			<div class="card">
				<div class="flex items-center justify-between">
					<span class="text-sm text-gray-600">Processing Mode</span>
					<div class="flex items-center gap-2">
						<span class="w-2 h-2 bg-green-500 rounded-full"></span>
						<span class="text-sm font-medium text-green-600">Local-only</span>
					</div>
				</div>
			</div>
			
			<div class="card">
				<div class="flex items-center justify-between">
					<span class="text-sm text-gray-600">Safety Mode</span>
					<div class="flex items-center gap-2">
						<span class="w-2 h-2 bg-green-500 rounded-full"></span>
						<span class="text-sm font-medium text-green-600">On (Copy)</span>
					</div>
				</div>
			</div>
			
			<div class="card">
				<div class="flex items-center justify-between">
					<span class="text-sm text-gray-600">Last Operation</span>
					<span class="text-sm font-medium text-gray-900">
						{hasData ? 'Scanned' : 'None'}
					</span>
				</div>
			</div>
		</div>
		
		<!-- Philosophy Box -->
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
			<p class="text-sm text-blue-900">
				<strong>💡 Home never edits anything.</strong> It orients and launches actions.
			</p>
		</div>
	</div>
</div>
