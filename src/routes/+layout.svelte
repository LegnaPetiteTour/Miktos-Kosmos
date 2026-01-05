<script lang="ts">
	import { page } from '$app/stores';
	import { beforeNavigate, goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import '../app.css';
	
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
	
	let currentPath = '/';
	let isTauri = false;
	
	// Subscribe to page changes
	page.subscribe(p => {
		currentPath = p.url.pathname;
		console.log('Current path:', currentPath);
	});
	
	onMount(() => {
		isTauri = '__TAURI__' in window;
		console.log('Running in Tauri:', isTauri);
	});
	
	function isActive(href: string): boolean {
		if (href === '/') return currentPath === '/';
		return currentPath.startsWith(href);
	}
	
	async function navigate(href: string) {
		console.log('Navigating to:', href);
		try {
			await goto(href, { 
				replaceState: false,
				noScroll: false,
				keepFocus: false,
				invalidateAll: true
			});
		} catch (error) {
			console.error('Navigation error:', error);
		}
	}
</script>

<style>
	nav {
		display: flex;
		gap: 0.75rem;
		padding: 1rem;
		border-bottom: 1px solid #e5e7eb;
		background-color: white;
	}
	
	nav button {
		padding: 0.5rem 1rem;
		border-radius: 0.375rem;
		font-size: 0.875rem;
		font-weight: 500;
		white-space: nowrap;
		transition: all 0.2s;
		border: none;
		cursor: pointer;
	}
	
	nav button.inactive {
		background-color: #f3f4f6;
		color: #374151;
	}
	
	nav button.inactive:hover {
		background-color: #e5e7eb;
	}
	
	nav button.active {
		background-color: #0284c7;
		color: white;
	}
	
	.brand {
		padding: 1rem;
		border-bottom: 1px solid #e5e7eb;
		background-color: white;
	}
	
	.brand h1 {
		font-size: 1.25rem;
		font-weight: 700;
		margin: 0;
	}
	
	.footer-info {
		padding: 1rem;
		border-top: 1px solid #e5e7eb;
		background-color: white;
		font-size: 0.75rem;
		color: #6b7280;
	}
	
	.status-dot {
		display: inline-block;
		width: 0.5rem;
		height: 0.5rem;
		background-color: #10b981;
		border-radius: 9999px;
		margin-right: 0.5rem;
	}
</style>

<div class="min-h-screen flex flex-col">
	<!-- Top Bar with Brand and Navigation -->
	<div>
		<!-- Brand -->
		<div class="brand">
			<h1>Miktos Kosmos</h1>
		</div>
		
		<!-- Horizontal Navigation -->
		<nav>
			{#each navigation as item}
				<button
					type="button"
					on:click={() => navigate(item.href)}
					class={isActive(item.href) ? 'active' : 'inactive'}
				>
					{item.name}
				</button>
			{/each}
		</nav>
	</div>
	
	<!-- Main Content Area -->
	<main class="flex-1 overflow-auto bg-gray-50">
		<slot />
	</main>
	
	<!-- Footer Info -->
	<div class="footer-info">
		<div>
			<span class="status-dot"></span>
			<span>Local-only • Privacy first • Open source</span>
		</div>
	</div>
</div>
