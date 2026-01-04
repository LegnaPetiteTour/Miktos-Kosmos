<script lang="ts">
	import { onMount } from 'svelte';
	
	let greeting = 'Welcome to Miktos Kosmos';
	
	onMount(() => {
		console.log('Miktos Kosmos is ready!');
	});
</script>

<div class="min-h-screen flex flex-col items-center justify-center p-8">
	<div class="card max-w-2xl w-full text-center space-y-6">
		<div class="flex items-center justify-center space-x-3 mb-8">
			<svg class="w-12 h-12 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
			</svg>
			<h1 class="text-4xl font-bold text-gray-900">Miktos Kosmos</h1>
		</div>
		
		<p class="text-xl text-gray-600">
			Transform your digital chaos into a beautifully organized family archive
		</p>
		
		<div class="flex flex-col sm:flex-row gap-4 justify-center mt-8">
			<a href="/scan" class="btn-primary text-lg px-8 py-3">
				Get Started
			</a>
			<a href="https://github.com/yourusername/photoarchive" target="_blank" class="btn-secondary text-lg px-8 py-3">
				Learn More
			</a>
		</div>
		
		<div class="mt-12 grid grid-cols-1 md:grid-cols-3 gap-6 text-left">
			<div class="space-y-2">
				<div class="w-10 h-10 bg-primary-100 rounded-lg flex items-center justify-center">
					<svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
					</svg>
				</div>
				<h3 class="font-semibold text-gray-900">Privacy First</h3>
				<p class="text-sm text-gray-600">All processing happens locally. Your photos never leave your computer.</p>
			</div>
			
			<div class="space-y-2">
				<div class="w-10 h-10 bg-primary-100 rounded-lg flex items-center justify-center">
					<svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
					</svg>
				</div>
				<h3 class="font-semibold text-gray-900">Blazing Fast</h3>
				<p class="text-sm text-gray-600">Process thousands of photos in minutes with Rust-powered performance.</p>
			</div>
			
			<div class="space-y-2">
				<div class="w-10 h-10 bg-primary-100 rounded-lg flex items-center justify-center">
					<svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
					</svg>
				</div>
				<h3 class="font-semibold text-gray-900">Safe by Default</h3>
				<p class="text-sm text-gray-600">Preview everything before committing. Easy undo for peace of mind.</p>
			</div>
		</div>
	</div>
	
	<footer class="mt-12 text-center text-gray-500 text-sm">
		<p>Built with ❤️ for families who want to preserve their memories</p>
	</footer>
</div>
