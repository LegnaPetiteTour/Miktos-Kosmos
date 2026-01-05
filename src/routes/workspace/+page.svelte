<script lang="ts">
	import DirectoryPicker from '$lib/components/DirectoryPicker.svelte';
	import ScanResults from '$lib/components/ScanResults.svelte';
	import { fileStore } from '$lib/stores/photoStore';
	
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
		<div>
			<h1 class="text-3xl font-bold text-gray-900">Workspace</h1>
			<p class="text-gray-600 mt-2">Your data, raw & honest</p>
		</div>
		
		<!-- Directory Picker -->
		<DirectoryPicker />
		
		<!-- Results -->
		{#if scanResult}
			<ScanResults stats={scanResult.stats} />
			
			<!-- File List Preview -->
			{#if files.length > 0}
				<div class="card">
					<h3 class="text-xl font-bold text-gray-900 mb-4">
						Found {files.length} Files
					</h3>
					
					<div class="max-h-96 overflow-y-auto space-y-2">
						{#each files.slice(0, 20) as file}
							<div class="flex items-center justify-between p-3 bg-gray-50 rounded-lg hover:bg-gray-100">
								<div class="flex-1 min-w-0">
									<p class="text-sm font-medium text-gray-900 truncate">
										{file.file_name}
									</p>
									<p class="text-xs text-gray-500">
										{file.width && file.height ? `${file.width} × ${file.height}` : ''}
										{#if file.file_type}
											<span class="ml-2 inline-block px-2 py-0.5 text-xs bg-blue-100 text-blue-800 rounded">
												{file.file_type}
											</span>
										{/if}
										{#if file.is_screenshot}
											<span class="ml-2 inline-block px-2 py-0.5 text-xs bg-yellow-100 text-yellow-800 rounded">
												Screenshot
											</span>
										{/if}
									</p>
								</div>
								<div class="text-xs text-gray-500">
									{(file.file_size / 1024 / 1024).toFixed(2)} MB
								</div>
							</div>
						{/each}
						
						{#if files.length > 20}
							<p class="text-center text-sm text-gray-500 py-4">
								...and {files.length - 20} more files
							</p>
						{/if}
					</div>
				</div>
			{/if}
		{:else}
			<!-- Empty State -->
			<div class="card text-center py-12">
				<h3 class="text-lg font-medium text-gray-900 mb-2">No scan results yet</h3>
				<p class="text-gray-500">Select a folder to start scanning your files</p>
			</div>
		{/if}
		
		<!-- Rule -->
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
			<p class="text-sm text-blue-900">
				<strong>Workspace = observation, not decision.</strong> View what exists before interpretation.
			</p>
		</div>
	</div>
</div>
