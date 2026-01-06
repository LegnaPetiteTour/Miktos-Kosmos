<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import type { NavItem, StatusBadge } from '$lib/types';
	import { fileStore } from '$lib/stores/photoStore';
	import AppShell from '$lib/ui/layout/AppShell.svelte';
	import '../app.css';
	
	const navItems: NavItem[] = [
		{ id: 'home', label: 'Home', icon: '🏠', route: '/' },
		{ id: 'workspace', label: 'Workspace', icon: '📁', route: '/workspace' },
		{ id: 'analyze', label: 'Analyze', icon: '🔍', route: '/analyze' },
		{ id: 'transform', label: 'Transform', icon: '⚡', route: '/transform' },
		{ id: 'review', label: 'Review', icon: '📋', route: '/review' },
		{ id: 'learn', label: 'Learn', icon: '📚', route: '/learn' },
		{ id: 'settings', label: 'Settings', icon: '⚙️', route: '/settings' },
		{ id: 'about', label: 'About', icon: 'ℹ️', route: '/about' }
	];
	
	const statusBadges: StatusBadge[] = [
		{ id: 'local', tone: 'success', text: 'Local-only', icon: '●' },
		{ id: 'safe', tone: 'success', text: 'Safe mode', icon: '🛡️' },
		{ id: 'ready', tone: 'neutral', text: 'Ready', icon: '✓' }
	];
	
	let selectedNavId: string = 'home';
	let workspaceLabel: string | undefined = undefined;
	let scanResult: any = null;
	
	// Subscribe to file store to get workspace info
	fileStore.subscribe(value => {
		scanResult = value;
		if (value && value.files && value.files.length > 0) {
			workspaceLabel = `${value.files.length} files loaded`;
		} else {
			workspaceLabel = undefined; // Don't show "None"
		}
	});
	
	// Update selected nav based on current route
	$: {
		const pathname = $page.url.pathname;
		if (pathname === '/') {
			selectedNavId = 'home';
		} else {
			const match = navItems.find(item => pathname === item.route);
			if (match) {
				selectedNavId = match.id;
			}
		}
	}
	
	function handleNavSelect(id: string) {
		const item = navItems.find(i => i.id === id);
		if (item) {
			goto(item.route);
		}
	}
	
	onMount(() => {
		console.log('Command Center initialized');
	});
</script>

<AppShell
	{navItems}
	{selectedNavId}
	onNavSelect={handleNavSelect}
	{workspaceLabel}
	{statusBadges}
>
	<slot />
</AppShell>
