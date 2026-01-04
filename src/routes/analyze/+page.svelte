<script lang="ts">
	import { photoStore, screenshots, nonScreenshots } from '$lib/stores/photoStore';
	import FileBrowser from '$lib/components/FileBrowser.svelte';
	import ScanResults from '$lib/components/ScanResults.svelte';
	import { goto } from '$app/navigation';
	
	let scanResult: any = null;
	let photos: any[] = [];
	
	// Subscribe to the global store
	photoStore.subscribe(value => {
		scanResult = value;
		photos = value?.photos || [];
	});
</script>

<div class="p-8">
	<div class="max-w-6xl mx-auto space-y-8">
		<!-- Header -->
		<div class="flex items-center justify-between">
			<div>
				<h1 class="text-3xl font-bold text-gray-900">Analyze</h1>
				<p class="text-gray-600 mt-2">Understand patterns & make decisions</p>
			</div>
			<a 
				href="/workspace"
				class="btn-secondary inline-block"
			>
				← Back to Workspace
			</a>
		</div>
		
		{#if scanResult && photos.length > 0}
			<!-- Stats Summary -->
			<ScanResults stats={scanResult.stats} />
			
			<!-- Insights -->
			<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
				<div class="card text-center">
					<div class="text-3xl font-bold text-blue-600">{$nonScreenshots.length}</div>
					<div class="text-sm text-gray-600 mt-1">Regular Photos</div>
				</div>
				
				<div class="card text-center">
					<div class="text-3xl font-bold text-yellow-600">{$screenshots.length}</div>
					<div class="text-sm text-gray-600 mt-1">Screenshots</div>
				</div>
				
				<div class="card text-center">
					<div class="text-3xl font-bold text-purple-600">
						{((photos.reduce((sum, p) => sum + p.file_size, 0) / 1024 / 1024) / photos.length).toFixed(1)} MB
					</div>
					<div class="text-sm text-gray-600 mt-1">Avg File Size</div>
				</div>
			</div>
			
			<!-- File Browser -->
			<div>
				<h2 class="text-2xl font-bold text-gray-900 mb-4">Browse Files</h2>
				<FileBrowser {photos} />
			</div>
			
			<!-- Next Step -->
			<div class="flex justify-between items-center">
				<div class="bg-blue-50 border border-blue-200 rounded-lg p-4 flex-1 mr-4">
					<p class="text-sm text-blue-900">
						<strong>Ready to organize?</strong> Review patterns and proceed to transform your library.
					</p>
				</div>
				<a 
					href="/transform"
					class="btn-primary whitespace-nowrap inline-block"
				>
					Next: Transform →
				</a>
			</div>
		{:else}
			<!-- Empty State -->
			<div class="card text-center py-16">
				<h3 class="text-xl font-medium text-gray-900 mb-2">No scan data available</h3>
				<p class="text-gray-500 mb-6">Scan a folder first to see analysis</p>
				<a 
					href="/workspace"
					class="btn-primary inline-block"
				>
					Go to Workspace
				</a>
			</div>
		{/if}
	</div>
</div>
