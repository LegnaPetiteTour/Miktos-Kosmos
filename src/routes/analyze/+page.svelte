<script lang="ts">
	import { fileStore, screenshots, nonScreenshots } from '$lib/stores/photoStore';
	import FileBrowser from '$lib/components/FileBrowser.svelte';
	import ScanResults from '$lib/components/ScanResults.svelte';
	
	let scanResult: any = null;
	let files: any[] = [];
	
	// Subscribe to the global store
	fileStore.subscribe(value => {
		scanResult = value;
		files = value?.files || [];
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
			<button 
				class="btn-secondary"
				on:click={() => window.location.href = '/workspace'}
			>
				← Back to Workspace
			</button>
		</div>
		
		{#if scanResult && files.length > 0}
			<!-- Stats Summary -->
			<ScanResults stats={scanResult.stats} />
			
			<!-- Insights -->
			<div class="grid grid-cols-3 gap-6">
				<!-- Screenshots Detection -->
				<div class="card">
					<div class="text-2xl mb-2">📸</div>
					<h3 class="text-lg font-semibold text-gray-900 mb-1">Screenshots Detected</h3>
					<p class="text-3xl font-bold text-primary-600">{$screenshots.length}</p>
					<p class="text-sm text-gray-500 mt-1">
						{(($screenshots.length / files.length) * 100).toFixed(1)}% of total
					</p>
				</div>
				
				<!-- Regular Photos -->
				<div class="card">
					<div class="text-2xl mb-2">📷</div>
					<h3 class="text-lg font-semibold text-gray-900 mb-1">Regular Photos</h3>
					<p class="text-3xl font-bold text-primary-600">{$nonScreenshots.length}</p>
					<p class="text-sm text-gray-500 mt-1">
						{(($nonScreenshots.length / files.length) * 100).toFixed(1)}% of total
					</p>
				</div>
				
				<!-- Organization Potential -->
				<div class="card">
					<div class="text-2xl mb-2">📁</div>
					<h3 class="text-lg font-semibold text-gray-900 mb-1">Organization Potential</h3>
					<p class="text-3xl font-bold text-primary-600">High</p>
					<p class="text-sm text-gray-500 mt-1">Ready to transform</p>
				</div>
			</div>
			
			<!-- File Browser -->
			<FileBrowser photos={files} />
			
			<!-- Next Step -->
			<div class="flex justify-between items-center">
				<div class="bg-blue-50 border border-blue-200 rounded-lg p-4 flex-1 mr-4">
					<p class="text-sm text-blue-900">
						<strong>Ready to organize?</strong> Review patterns and proceed to transform your library.
					</p>
				</div>
				<button 
					class="btn-primary whitespace-nowrap"
					on:click={() => window.location.href = '/transform'}
				>
					Next: Transform →
				</button>
			</div>
		{:else}
			<!-- Empty State -->
			<div class="card text-center py-16">
				<h3 class="text-xl font-medium text-gray-900 mb-2">No scan data available</h3>
				<p class="text-gray-500 mb-6">Scan a folder first to see analysis</p>
				<button 
					class="btn-primary"
					on:click={() => window.location.href = '/workspace'}
				>
					Go to Workspace
				</button>
			</div>
		{/if}
	</div>
</div>
