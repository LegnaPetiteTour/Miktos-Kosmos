<script lang="ts">
	export let stats: any;
	
	function formatBytes(bytes: number): string {
		const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
		if (bytes === 0) return '0 Bytes';
		const i = Math.floor(Math.log(bytes) / Math.log(1024));
		return Math.round(bytes / Math.pow(1024, i) * 100) / 100 + ' ' + sizes[i];
	}
	
	function formatDate(dateStr: string | null): string {
		if (!dateStr) return 'Unknown';
		return new Date(dateStr).toLocaleDateString();
	}
</script>

<div class="card">
	<h2 class="text-2xl font-bold text-gray-900 mb-6">Scan Results</h2>
	
	<div class="grid grid-cols-2 md:grid-cols-5 gap-4">
		<div class="bg-blue-50 p-4 rounded-lg">
			<div class="text-3xl font-bold text-blue-600">{stats.total_files || 0}</div>
			<div class="text-sm text-blue-900 font-medium">Total Files</div>
		</div>
		
		<div class="bg-indigo-50 p-4 rounded-lg">
			<div class="text-3xl font-bold text-indigo-600">{stats.file_types?.images || 0}</div>
			<div class="text-sm text-indigo-900 font-medium">Images</div>
		</div>
		
		<div class="bg-purple-50 p-4 rounded-lg">
			<div class="text-3xl font-bold text-purple-600">{stats.file_types?.videos || 0}</div>
			<div class="text-sm text-purple-900 font-medium">Videos</div>
		</div>
		
		<div class="bg-yellow-50 p-4 rounded-lg">
			<div class="text-3xl font-bold text-yellow-600">{stats.screenshots || 0}</div>
			<div class="text-sm text-yellow-900 font-medium">Screenshots</div>
		</div>
		
		<div class="bg-red-50 p-4 rounded-lg">
			<div class="text-3xl font-bold text-red-600">{stats.duplicates || 0}</div>
			<div class="text-sm text-red-900 font-medium">Duplicates</div>
		</div>
	</div>
	
	{#if stats.file_types?.documents > 0 || stats.file_types?.audio > 0 || stats.file_types?.archives > 0 || stats.file_types?.other > 0}
		<div class="grid grid-cols-2 md:grid-cols-4 gap-4 mt-4">
			{#if stats.file_types.documents > 0}
				<div class="bg-green-50 p-4 rounded-lg">
					<div class="text-2xl font-bold text-green-600">{stats.file_types.documents}</div>
					<div class="text-sm text-green-900 font-medium">Documents</div>
				</div>
			{/if}
			
			{#if stats.file_types.audio > 0}
				<div class="bg-orange-50 p-4 rounded-lg">
					<div class="text-2xl font-bold text-orange-600">{stats.file_types.audio}</div>
					<div class="text-sm text-orange-900 font-medium">Audio</div>
				</div>
			{/if}
			
			{#if stats.file_types.archives > 0}
				<div class="bg-amber-50 p-4 rounded-lg">
					<div class="text-2xl font-bold text-amber-600">{stats.file_types.archives}</div>
					<div class="text-sm text-amber-900 font-medium">Archives</div>
				</div>
			{/if}
			
			{#if stats.file_types.other > 0}
				<div class="bg-gray-50 p-4 rounded-lg">
					<div class="text-2xl font-bold text-gray-600">{stats.file_types.other}</div>
					<div class="text-sm text-gray-900 font-medium">Other</div>
				</div>
			{/if}
		</div>
	{/if}
	
	<div class="mt-6 space-y-3">
		<div class="flex justify-between items-center py-2 border-b">
			<span class="text-gray-600">Total Size</span>
			<span class="font-semibold text-gray-900">{formatBytes(stats.total_size || 0)}</span>
		</div>
		
		{#if stats.date_range}
			<div class="flex justify-between items-center py-2 border-b">
				<span class="text-gray-600">Date Range</span>
				<span class="font-semibold text-gray-900">
					{formatDate(stats.date_range[0])} - {formatDate(stats.date_range[1])}
				</span>
			</div>
		{/if}
	</div>
</div>
