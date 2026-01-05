<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';
	import { photoStore, selectedFolderStore, scanningStore } from '$lib/stores/photoStore';
	
	let error = '';
	let selectedPath = '';
	let isScanning = false;
	let isTauri = false;
	
	// Check if running in Tauri
	import { onMount } from 'svelte';
	onMount(() => {
		isTauri = '__TAURI__' in window;
	});
	
	selectedFolderStore.subscribe(value => selectedPath = value);
	scanningStore.subscribe(value => isScanning = value);
	
	// Tauri directory selection
	async function selectDirectoryTauri() {
		try {
			const selected = await open({
				directory: true,
				multiple: false,
				title: 'Select folder to scan'
			});
			
			if (selected && typeof selected === 'string') {
				selectedFolderStore.set(selected);
				await startScan();
			}
		} catch (e) {
			error = `Failed to select directory: ${e}`;
		}
	}
	
	// Browser directory selection (HTML5 File API)
	async function selectDirectoryBrowser() {
		try {
			// @ts-ignore - showDirectoryPicker is experimental
			const dirHandle = await window.showDirectoryPicker();
			const path = dirHandle.name;
			
			selectedFolderStore.set(path);
			error = 'Browser mode: Scanning not available. Use the desktop app for full functionality.';
		} catch (e) {
			if ((e as Error).name !== 'AbortError') {
				error = `Failed to select directory: ${e}`;
			}
		}
	}
	
	async function selectDirectory() {
		if (isTauri) {
			await selectDirectoryTauri();
		} else {
			await selectDirectoryBrowser();
		}
	}
	
	async function startScan() {
		if (!selectedPath) {
			error = 'Please select a directory first';
			return;
		}
		
		if (!isTauri) {
			error = 'Scanning only works in the desktop app. You are in browser mode.';
			return;
		}
		
		scanningStore.set(true);
		error = '';
		
		try {
			const result = await invoke('scan_directory', { path: selectedPath });
			photoStore.setScanResult(result);
		} catch (e) {
			error = `Scan failed: ${e}`;
		} finally {
			scanningStore.set(false);
		}
	}
</script>

<div class="card space-y-4">
	<h2 class="text-2xl font-bold text-gray-900">Select Folder to Scan</h2>
	
	{#if !isTauri}
		<div class="bg-yellow-50 border border-yellow-200 p-3 rounded-lg">
			<p class="text-sm text-yellow-800">
				<strong>Browser Mode:</strong> You're viewing in the browser. For full functionality (scanning, organizing), use the desktop app.
			</p>
		</div>
	{/if}
	
	<div class="flex gap-3">
		<button 
			class="btn-secondary flex-1"
			on:click={selectDirectory}
			disabled={isScanning}
		>
			<svg class="w-5 h-5 inline mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
				<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
			</svg>
			Choose Folder
		</button>
		
		{#if isTauri}
			<button 
				class="btn-primary flex-1"
				on:click={startScan}
				disabled={!selectedPath || isScanning}
			>
				{#if isScanning}
					<svg class="animate-spin w-5 h-5 inline mr-2" fill="none" viewBox="0 0 24 24">
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
						<path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
					</svg>
					Scanning...
				{:else}
					<svg class="w-5 h-5 inline mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
					</svg>
					Start Scan
				{/if}
			</button>
		{/if}
	</div>
	
	{#if selectedPath}
		<div class="bg-gray-50 p-3 rounded-lg">
			<p class="text-sm text-gray-600">Selected folder:</p>
			<p class="text-sm font-mono text-gray-900 break-all">{selectedPath}</p>
		</div>
	{/if}
	
	{#if error}
		<div class="bg-red-50 border border-red-200 p-3 rounded-lg">
			<p class="text-sm text-red-800">{error}</p>
		</div>
	{/if}
</div>
