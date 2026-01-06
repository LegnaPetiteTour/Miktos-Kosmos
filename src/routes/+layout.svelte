<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import type { NavItem, StatusBadge } from '$lib/types';
	import { fileStore } from '$lib/stores/photoStore';
	import { themeStore } from '$lib/stores/themeStore';
	import { icons } from '$lib/ui/icons';
	import AppShell from '$lib/ui/layout/AppShell.svelte';
	import '../app.css';
	
	const navItems: NavItem[] = [
		{ id: 'home', label: 'Home', route: '/' },
		{ id: 'workspace', label: 'Workspace', route: '/workspace' },
		{ id: 'review', label: 'Review', route: '/review' },
		{ id: 'learn', label: 'Learn', route: '/learn' },
		{ id: 'settings', label: 'Settings', route: '/settings' },
		{ id: 'about', label: 'About', route: '/about' }
	];
	
	const statusBadges: StatusBadge[] = [
		{ id: 'local', tone: 'success', text: 'Local-only', icon: icons.dot },
		{ id: 'safe', tone: 'success', text: 'Safe mode', icon: icons.check },
		{ id: 'ready', tone: 'neutral', text: 'Ready', icon: icons.check }
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
			workspaceLabel = undefined;
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
		// Initialize theme
		themeStore.initialize();
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
