<script lang="ts">
	import { page } from '$app/stores';
	
	const navigation = [
		{ name: 'Home', href: '/' },
		{ name: 'Workspace', href: '/workspace' },
		{ name: 'Analyze', href: '/analyze' },
		{ name: 'Transform', href: '/transform' },
		{ name: 'Review', href: '/review' },
		{ name: 'Learn', href: '/learn' },
		{ name: 'Settings', href: '/settings' },
		{ name: 'About', href: '/about' }
	];
	
	$: currentPath = $page.url.pathname;
	
	function isActive(href: string): boolean {
		if (href === '/') return currentPath === '/';
		return currentPath.startsWith(href);
	}
</script>

<div class="min-h-screen flex">
	<!-- Sidebar Navigation -->
	<aside class="w-64 bg-white border-r border-gray-200 flex flex-col">
		<!-- Brand -->
		<div class="p-6 border-b border-gray-200">
			<h1 class="text-xl font-bold text-gray-900">Miktos Kosmos</h1>
		</div>
		
		<!-- Navigation -->
		<nav class="flex-1 p-4 space-y-3">
			{#each navigation as item}
				<a
					href={item.href}
					class="block px-4 py-3 rounded-lg text-sm font-medium transition-all {isActive(item.href)
						? 'bg-primary-600 text-white shadow-md'
						: 'bg-gray-100 text-gray-700 hover:bg-gray-200 hover:shadow-sm'}"
				>
					{item.name}
				</a>
			{/each}
		</nav>
		
		<!-- Footer Info -->
		<div class="p-4 border-t border-gray-200">
			<div class="text-xs text-gray-500 space-y-1">
				<div class="flex items-center gap-2">
					<div class="w-2 h-2 bg-green-500 rounded-full"></div>
					<span>Local-only</span>
				</div>
				<div>Privacy first • Open source</div>
			</div>
		</div>
	</aside>
	
	<!-- Main Content Area -->
	<main class="flex-1 overflow-auto bg-gray-50">
		<slot />
	</main>
</div>
