<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import type { NavItem, StatusBadge } from '$lib/types';
	import { fileStore } from '$lib/stores/photoStore';
	import { themeStore } from '$lib/stores/themeStore';
	import { icons } from '$lib/ui/icons';
	import { listen } from '@tauri-apps/api/event';
	import AppShell from '$lib/ui/layout/AppShell.svelte';
	import '../app.css';
	
	const navItems: NavItem[] = [
		{ id: 'home', label: 'Home', route: '/' },
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
	
	// Dialogs
	let showAboutDialog = false;
	let showSettingsDialog = false;
	let showLearnDialog = false;
	
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
		
		// Listen for menu events from Tauri
		let unlistenAbout: (() => void) | undefined;
		let unlistenSettings: (() => void) | undefined;
		let unlistenLearn: (() => void) | undefined;
		
		listen('show-about', () => {
			showAboutDialog = true;
		}).then((unlisten) => {
			unlistenAbout = unlisten;
		});
		
		listen('show-settings', () => {
			showSettingsDialog = true;
		}).then((unlisten) => {
			unlistenSettings = unlisten;
		});
		
		listen('show-learn', () => {
			showLearnDialog = true;
		}).then((unlisten) => {
			unlistenLearn = unlisten;
		});
		
		console.log('Command Center initialized');
		
		return () => {
			unlistenAbout?.();
			unlistenSettings?.();
			unlistenLearn?.();
		};
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

<!-- Simple Dialog Overlays -->
{#if showAboutDialog}
	<div class="dialog-overlay" on:click={() => showAboutDialog = false} on:keydown={(e) => e.key === 'Escape' && (showAboutDialog = false)} role="button" tabindex="0">
		<div class="dialog-content" on:click|stopPropagation on:keydown role="button" tabindex="-1">
			<h2>About Miktos Kosmos</h2>
			<p class="version">Version 0.3.0 Alpha</p>
			<p>Privacy-first family photo organizer.</p>
			<p>Transform 10 years of digital chaos into a beautifully organized family archive.</p>
			<p class="copyright">Copyright © 2025 Angel Torrella</p>
			<button class="dialog-close" on:click={() => showAboutDialog = false}>Close</button>
		</div>
	</div>
{/if}

{#if showSettingsDialog}
	<div class="dialog-overlay" on:click={() => showSettingsDialog = false} on:keydown={(e) => e.key === 'Escape' && (showSettingsDialog = false)} role="button" tabindex="0">
		<div class="dialog-content" on:click|stopPropagation on:keydown role="button" tabindex="-1">
			<h2>Settings</h2>
			<p>Settings panel coming soon...</p>
			<button class="dialog-close" on:click={() => showSettingsDialog = false}>Close</button>
		</div>
	</div>
{/if}

{#if showLearnDialog}
	<div class="dialog-overlay" on:click={() => showLearnDialog = false} on:keydown={(e) => e.key === 'Escape' && (showLearnDialog = false)} role="button" tabindex="0">
		<div class="dialog-content" on:click|stopPropagation on:keydown role="button" tabindex="-1">
			<h2>Learn Miktos Kosmos</h2>
			<p>Learning resources coming soon...</p>
			<button class="dialog-close" on:click={() => showLearnDialog = false}>Close</button>
		</div>
	</div>
{/if}

<style>
	.dialog-overlay {
		position: fixed;
		inset: 0;
		background: rgba(0, 0, 0, 0.6);
		backdrop-filter: blur(8px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
	}
	
	.dialog-content {
		background: var(--panel);
		border: 1px solid var(--panel-border);
		border-radius: 12px;
		padding: 32px;
		max-width: 500px;
		width: 90%;
		box-shadow: 0 20px 60px rgba(0, 0, 0, 0.4);
	}
	
	.dialog-content h2 {
		margin: 0 0 16px 0;
		font-size: 24px;
		color: var(--text);
	}
	
	.dialog-content p {
		margin: 12px 0;
		color: var(--text-muted);
		line-height: 1.6;
	}
	
	.version {
		font-size: 14px;
		color: var(--text-subtle);
		margin-bottom: 24px;
	}
	
	.copyright {
		font-size: 12px;
		color: var(--text-subtle);
		margin-top: 24px;
	}
	
	.dialog-close {
		margin-top: 24px;
		padding: 10px 24px;
		background: var(--accent);
		color: white;
		border: none;
		border-radius: 6px;
		cursor: pointer;
		font-weight: 500;
		transition: all 0.2s;
	}
	
	.dialog-close:hover {
		background: var(--accent-hover);
		transform: translateY(-1px);
	}
</style>
