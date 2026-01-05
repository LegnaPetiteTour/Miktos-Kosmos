<script lang="ts">
	import type { Photo } from '$lib/stores/photoStore';
	
	export let photos: Photo[] = [];
	
	let sortBy: 'name' | 'size' | 'date' = 'name';
	let filterType: 'all' | 'photos' | 'screenshots' = 'all';
	let searchQuery = '';
	
	$: filteredPhotos = photos
		.filter(photo => {
			if (!photo) return false;
			
			// Filter by type
			if (filterType === 'screenshots' && !photo.is_screenshot) return false;
			if (filterType === 'photos' && photo.is_screenshot) return false;
			
			// Filter by search query
			if (searchQuery && photo.file_name && !photo.file_name.toLowerCase().includes(searchQuery.toLowerCase())) {
				return false;
			}
			
			return true;
		})
		.sort((a, b) => {
			if (!a || !b) return 0;
			
			switch (sortBy) {
				case 'name':
					return (a.file_name || '').localeCompare(b.file_name || '');
				case 'size':
					return (b.file_size || 0) - (a.file_size || 0);
				case 'date':
					if (!a.date_taken || !b.date_taken) return 0;
					return new Date(b.date_taken).getTime() - new Date(a.date_taken).getTime();
				default:
					return 0;
			}
		});
	
	function getParentFolder(filePath: string | undefined): string {
		if (!filePath) return 'root';
		const parts = filePath.split('/');
		return parts.length > 1 ? parts[parts.length - 2] : 'root';
	}
</script>

<div class="card space-y-4">
	<!-- Controls -->
	<div class="flex flex-wrap gap-4 items-center justify-between">
		<!-- Search -->
		<div class="flex-1 min-w-64">
			<input 
				type="text" 
				bind:value={searchQuery}
				placeholder="Search files..."
				class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent"
			/>
		</div>
		
		<!-- Filters -->
		<div class="flex gap-2">
			<select 
				bind:value={filterType}
				class="px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
			>
				<option value="all">All Files</option>
				<option value="photos">Photos Only</option>
				<option value="screenshots">Screenshots Only</option>
			</select>
			
			<select 
				bind:value={sortBy}
				class="px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500"
			>
				<option value="name">Sort by Name</option>
				<option value="size">Sort by Size</option>
				<option value="date">Sort by Date</option>
			</select>
		</div>
	</div>
	
	<!-- Results count -->
	<div class="text-sm text-gray-600">
		Showing {filteredPhotos.length} of {photos.length} files
	</div>
	
	<!-- File list -->
	<div class="max-h-96 overflow-y-auto space-y-2">
		{#each filteredPhotos as photo}
			{#if photo}
				<div class="flex items-center justify-between p-3 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors">
					<div class="flex-1 min-w-0">
						<p class="text-sm font-medium text-gray-900 truncate">
							{photo.file_name || 'Unknown'}
						</p>
						<div class="flex items-center gap-2 text-xs text-gray-500 mt-1">
							{#if photo.width && photo.height}
								<span>{photo.width} × {photo.height}</span>
								<span>•</span>
							{/if}
							<span>{((photo.file_size || 0) / 1024 / 1024).toFixed(2)} MB</span>
							{#if photo.date_taken}
								<span>•</span>
								<span>{new Date(photo.date_taken).toLocaleDateString()}</span>
							{/if}
							{#if photo.is_screenshot}
								<span class="ml-2 inline-block px-2 py-0.5 bg-yellow-100 text-yellow-800 rounded">
									Screenshot
								</span>
							{/if}
						</div>
					</div>
					
					<div class="text-xs text-gray-400 ml-4">
						{getParentFolder(photo.file_path)}
					</div>
				</div>
			{/if}
		{/each}
		
		{#if filteredPhotos.length === 0}
			<div class="text-center py-12 text-gray-500">
				No files match your filters
			</div>
		{/if}
	</div>
</div>
