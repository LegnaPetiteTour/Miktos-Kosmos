<script lang="ts">
	import { onMount } from 'svelte';
	
	onMount(() => {
		console.log('Miktos Kosmos is ready!');
	});
</script>

<div class="p-8">
	<div class="max-w-4xl mx-auto space-y-8">
		<!-- Header -->
		<div>
			<h1 class="text-3xl font-bold text-gray-900">Home</h1>
			<p class="text-gray-600 mt-2">Orientation & Control</p>
		</div>
		
		<!-- Current Workspace Summary -->
		<div class="card">
			<h2 class="text-xl font-semibold text-gray-900 mb-4">Current Workspace</h2>
			<div class="grid grid-cols-2 md:grid-cols-4 gap-4">
				<div>
					<div class="text-2xl font-bold text-gray-900">0</div>
					<div class="text-sm text-gray-600">Total Files</div>
				</div>
				<div>
					<div class="text-2xl font-bold text-gray-900">—</div>
					<div class="text-sm text-gray-600">Date Span</div>
				</div>
				<div>
					<div class="text-2xl font-bold text-gray-900">—</div>
					<div class="text-sm text-gray-600">Last Operation</div>
				</div>
				<div>
					<div class="text-2xl font-bold text-gray-900">0 MB</div>
					<div class="text-sm text-gray-600">Total Size</div>
				</div>
			</div>
		</div>
		
		<!-- Primary Actions -->
		<div class="card">
			<h2 class="text-xl font-semibold text-gray-900 mb-4">Primary Actions</h2>
			<div class="grid grid-cols-1 md:grid-cols-3 gap-4">
				<a href="/workspace" class="btn-primary py-4 text-center">
					Scan Folder
				</a>
				<a href="/analyze" class="btn-secondary py-4 text-center">
					Analyze Files
				</a>
				<a href="/transform" class="btn-secondary py-4 text-center">
					Create Structure
				</a>
			</div>
		</div>
		
		<!-- Status Indicators -->
		<div class="card">
			<h2 class="text-xl font-semibold text-gray-900 mb-4">Status</h2>
			<div class="space-y-3">
				<div class="flex items-center justify-between py-2 border-b border-gray-100">
					<span class="text-gray-700">Processing Mode</span>
					<span class="text-green-600 font-medium">Local-only</span>
				</div>
				<div class="flex items-center justify-between py-2 border-b border-gray-100">
					<span class="text-gray-700">Safety Mode</span>
					<span class="text-green-600 font-medium">On (Copy)</span>
				</div>
				<div class="flex items-center justify-between py-2">
					<span class="text-gray-700">Disk Usage</span>
					<span class="text-gray-600">0 MB used</span>
				</div>
			</div>
		</div>
		
		<!-- Quick Info -->
		<div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
			<p class="text-sm text-blue-900">
				<strong>Home never edits anything.</strong> It orients and launches actions.
			</p>
		</div>
	</div>
</div>
