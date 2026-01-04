<script lang="ts">
	import { onMount } from 'svelte';
	
	onMount(() => {
		console.log('Miktos Kosmos is ready!');
	});
</script>

<div class="min-h-screen flex flex-col items-center justify-center p-8 bg-gradient-to-br from-gray-50 to-gray-100">
	<div class="card max-w-3xl w-full text-center space-y-8">
		<!-- Title Only -->
		<div class="mb-6">
			<h1 class="text-5xl font-bold text-gray-900">Miktos Kosmos</h1>
		</div>
		
		<p class="text-xl text-gray-600 max-w-2xl mx-auto">
			Transform your digital chaos into a beautifully organized family archive
		</p>
		
		<!-- CTA Buttons -->
		<div class="flex flex-col sm:flex-row gap-4 justify-center mt-8">
			<a href="/scan" class="btn-primary text-lg px-8 py-3">
				Get Started
			</a>
			<a href="https://github.com/yourusername/miktos-kosmos" target="_blank" class="btn-secondary text-lg px-8 py-3">
				Learn More
			</a>
		</div>
		
		<!-- Feature Cards -->
		<div class="mt-16 grid grid-cols-1 md:grid-cols-3 gap-8 text-left">
			<!-- Privacy First -->
			<div class="space-y-4">
				<h3 class="text-xl font-semibold text-gray-900">Privacy First</h3>
				<p class="text-gray-600 leading-relaxed">
					All processing happens locally. Your photos never leave your computer. Zero cloud uploads, zero tracking.
				</p>
			</div>
			
			<!-- Blazing Fast -->
			<div class="space-y-4">
				<h3 class="text-xl font-semibold text-gray-900">Blazing Fast</h3>
				<p class="text-gray-600 leading-relaxed">
					Process thousands of photos in minutes with Rust-powered performance. Built for speed and efficiency.
				</p>
			</div>
			
			<!-- Safe by Default -->
			<div class="space-y-4">
				<h3 class="text-xl font-semibold text-gray-900">Safe by Default</h3>
				<p class="text-gray-600 leading-relaxed">
					Preview everything before committing. Non-destructive operations. Easy undo for peace of mind.
				</p>
			</div>
		</div>
	</div>
	
	<footer class="mt-16 text-center text-gray-500 text-sm">
		<p>Built with ❤️ for families who want to preserve their memories</p>
		<p class="mt-2 text-xs">Open Source • MIT License • Privacy Focused</p>
	</footer>
</div>
