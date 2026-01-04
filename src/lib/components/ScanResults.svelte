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
	
	<div class="grid grid-cols-2 md:grid-cols-4 gap-4">
		<div class="bg-blue-50 p-4 rounded-lg">
			<div class="text-3xl font-bold text-blue-600">{stats.total_photos}</div>
			<div class="text-sm text-blue-900 font-medium">Photos</div>
		</div>
		
		<div class="bg-green-50 p-4 rounded-lg">
			<div class="text-3xl font-bold text-green-600">{stats.total_videos}</div>
			<div class="text-sm text-green-900 font-medium">Videos</div>
		</div>
		
		<div class="bg-yellow-50 p-4 rounded-lg">
			<div class="text-3xl font-bold text-yellow-600">{stats.screenshots}</div>
			<div class="text-sm text-yellow-900 font-medium">Screenshots</div>
		</div>
		
		<div class="bg-purple-50 p-4 rounded-lg">
			<div class="text-3xl font-bold text-purple-600">{stats.duplicates}</div>
			<div class="text-sm text-purple-900 font-medium">Duplicates</div>
		</div>
	</div>
	
	<div class="mt-6 space-y-3">
		<div class="flex justify-between items-center py-2 border-b">
			<span class="text-gray-600">Total Size</span>
			<span class="font-semibold text-gray-900">{formatBytes(stats.total_size)}</span>
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
