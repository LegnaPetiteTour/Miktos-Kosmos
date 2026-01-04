<script lang="ts">
	import DirectoryPicker from '$lib/components/DirectoryPicker.svelte';
	import ScanResults from '$lib/components/ScanResults.svelte';
	
	let scanResult: any = null;
	let photos: any[] = [];
	
	function handleScanComplete(result: any) {
		console.log('Scan complete:', result);
		scanResult = result;
		photos = result.photos || [];
	}
</script>

<div class="min-h-screen p-8">
	<div class="max-w-6xl mx-auto space-y-8">
		<!-- Header -->
		<div class="flex items-center justify-between">
			<div>
				<h1 class="text-3xl font-bold text-gray-900">Scan Your Photos</h1>
				<p class="text-gray-600 mt-1">Select a folder to analyze your photo library</p>
			</div>
			<a href="/" class="btn-secondary">
				← Back to Home
			</a>
		</div>
		
		<!-- Directory Picker -->
		<DirectoryPicker onScanComplete={handleScanComplete} />
		
		<!-- Results -->
		{#if scanResult}
			<ScanResults stats={scanResult.stats} />
			
			<!-- Photo List Preview -->
			{#if photos.length > 0}
				<div class="card">
					<h3 class="text-xl font-bold text-gray-900 mb-4">
						Found {photos.length} Photos
					</h3>
					
					<div class="max-h-96 overflow-y-auto space-y-2">
						{#each photos.slice(0, 20) as photo}
							<div class="flex items-center justify-between p-3 bg-gray-50 rounded-lg hover:bg-gray-100">
								<div class="flex-1 min-w-0">
									<p class="text-sm font-medium text-gray-900 truncate">
										{photo.file_name}
									</p>
									<p class="text-xs text-gray-500">
										{photo.width && photo.height ? `${photo.width} × ${photo.height}` : 'Unknown size'}
										{#if photo.is_screenshot}
											<span class="ml-2 inline-block px-2 py-0.5 text-xs bg-yellow-100 text-yellow-800 rounded">
												Screenshot
											</span>
										{/if}
									</p>
								</div>
								<div class="text-xs text-gray-500">
									{(photo.file_size / 1024 / 1024).toFixed(2)} MB
								</div>
							</div>
						{/each}
						
						{#if photos.length > 20}
							<p class="text-center text-sm text-gray-500 py-4">
								...and {photos.length - 20} more photos
							</p>
						{/if}
					</div>
				</div>
			{/if}
		{:else}
			<!-- Empty State -->
			<div class="card text-center py-12">
				<svg class="w-16 h-16 mx-auto text-gray-300 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
				</svg>
				<h3 class="text-lg font-medium text-gray-900 mb-2">No scan results yet</h3>
				<p class="text-gray-500">Select a folder above to start scanning your photos</p>
			</div>
		{/if}
	</div>
</div>
