<script lang="ts">
	import { photoStore } from '$lib/stores/photoStore';
	import { goto } from '$app/navigation';
	
	let scanResult: any = null;
	
	photoStore.subscribe(value => {
		scanResult = value;
	});
</script>

<div class="p-8">
	<div class="max-w-6xl mx-auto space-y-8">
		<!-- Header -->
		<div class="flex items-center justify-between">
			<div>
				<h1 class="text-3xl font-bold text-gray-900">Transform</h1>
				<p class="text-gray-600 mt-2">Turn insight into structure</p>
			</div>
			<a 
				href="/analyze"
				class="btn-secondary inline-block"
			>
				← Back to Analyze
			</a>
		</div>
		
		{#if scanResult && scanResult.photos.length > 0}
			<!-- Organization Preview -->
			<div class="card">
				<h2 class="text-xl font-semibold text-gray-900 mb-4">Organization Preview</h2>
				<p class="text-gray-600 mb-4">
					Preview how your {scanResult.photos.length} photos will be organized
				</p>
				
				<div class="bg-gray-50 rounded-lg p-4 space-y-2">
					<div class="font-mono text-sm text-gray-700">
						📁 organized/<br/>
						&nbsp;&nbsp;📁 2020/<br/>
						&nbsp;&nbsp;&nbsp;&nbsp;📁 07-July/<br/>
						&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;📷 photo_001.jpg<br/>
						&nbsp;&nbsp;📁 2022/<br/>
						&nbsp;&nbsp;&nbsp;&nbsp;📁 09-September/<br/>
						&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;📷 photo_002.jpg
					</div>
				</div>
			</div>
			
			<!-- Actions -->
			<div class="card">
				<h2 class="text-xl font-semibold text-gray-900 mb-4">Choose Action</h2>
				<div class="space-y-3">
					<button class="w-full text-left p-4 border-2 border-blue-300 bg-blue-50 rounded-lg hover:border-blue-400 transition">
						<div class="font-medium text-gray-900">📋 Copy Files (Recommended)</div>
						<div class="text-sm text-gray-600">Keep originals safe, create organized copies</div>
					</button>
					<button class="w-full text-left p-4 border border-gray-300 rounded-lg hover:border-gray-400 transition">
						<div class="font-medium text-gray-900">🔀 Move Files</div>
						<div class="text-sm text-gray-600">Relocate files to new structure</div>
					</button>
				</div>
			</div>
			
			<!-- Next Step -->
			<div class="flex justify-between items-center">
				<div class="bg-blue-50 border border-blue-200 rounded-lg p-4 flex-1 mr-4">
					<p class="text-sm text-blue-900">
						<strong>Almost there!</strong> Review your organization plan before applying changes.
					</p>
				</div>
				<a 
					href="/review"
					class="btn-primary whitespace-nowrap inline-block"
				>
					Next: Review →
				</a>
			</div>
		{:else}
			<!-- Empty State -->
			<div class="card text-center py-16">
				<h3 class="text-xl font-medium text-gray-900 mb-2">No scan data available</h3>
				<p class="text-gray-500 mb-6">Scan and analyze a folder first</p>
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
