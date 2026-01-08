<script lang="ts">
	import { goto } from '$app/navigation';
	import { fileStore } from '$lib/stores/photoStore';
	import { historyStore } from '$lib/stores/historyStore';
	import type { HistoryEntry } from '$lib/stores/historyStore';
	import { invoke } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';
	import { listen } from '@tauri-apps/api/event';
	import { onMount, onDestroy } from 'svelte';
	
	let scanResult: any = null;
	let isScanning = false;
	let scanProgress: any = null;
	let unlisten: any = null;
	
	// Set up event listener for real-time progress
	onMount(async () => {
		unlisten = await listen('scan-progress', (event) => {
			if (isScanning) {
				scanProgress = event.payload;
			}
		});
	});
	
	onDestroy(() => {
		if (unlisten) {
			unlisten();
		}
	});
	
	fileStore.subscribe(value => {
		scanResult = value;
	});
	
	$: hasFiles = scanResult?.files?.length > 0;
	$: totalFiles = scanResult?.files?.length || 0;
	
	// Use stats from scanner directly (already calculated in Rust)
	$: scanStats = scanResult?.stats || null;
	
	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return `${(bytes / Math.pow(k, i)).toFixed(1)} ${sizes[i]}`;
	}
	
	function formatDateRange(dateRange: any): string {
		if (!dateRange || !dateRange[0] || !dateRange[1]) return 'Unknown';
		
		const start = new Date(dateRange[0]).toLocaleDateString('en-US', { 
			month: 'short', 
			year: 'numeric' 
		});
		const end = new Date(dateRange[1]).toLocaleDateString('en-US', { 
			month: 'short', 
			year: 'numeric' 
		});
		
		if (start === end) return start;
		return `${start} → ${end}`;
	}

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}
	
	function formatPath(path: string): string {
		if (!path) return '';
		
		// Replace home directory with ~
		const homeDir = '/Users/' + (path.split('/')[2] || '');
		if (path.startsWith(homeDir)) {
			path = '~' + path.substring(homeDir.length);
		}
		
		// Split by / and join with arrow
		const parts = path.split('/').filter(p => p);
		if (parts.length === 0) return '/';
		
		// Show last 3 parts if path is too long
		if (parts.length > 4) {
			return '.../' + parts.slice(-3).join('/');
		}
		
		return parts.join('/');
	}


	
	// Accordion state
	let expandedSections: Record<string, boolean> = {
		workspace: true,
		analyze: false,
		organize: false,
		review: false,
		export: false
	};
	
	function toggleSection(section: string) {
		expandedSections[section] = !expandedSections[section];
	}
	
	// Scan folder
	async function handleScanFolder() {
		try {
			const selected = await open({
				directory: true,
				multiple: false,
				title: 'Select folder to scan'
			});
			
			if (selected) {
				// Set scanning state
				isScanning = true;
				scanProgress = { 
					total_files: 0,
					processed_files: 0,
					current_file: 'Initializing...',
					percentage: 0 
				};
				
				// Start scan - progress will be updated via events
				const result = await invoke('scan_directory', { path: selected });
				
				// Final update
				scanProgress = { 
					total_files: (result as any).stats.total_files,
					processed_files: (result as any).stats.total_files,
					current_file: 'Complete!',
					percentage: 100 
				};
				
				fileStore.setScanResult(result);
				
				// Add to history
				const stats = (result as any).stats;
				const historyEntry: HistoryEntry = {
					id: Date.now().toString(),
					timestamp: new Date().toISOString(),
					folder_path: selected,
					total_files: stats.total_files,
					total_size: stats.total_size,
					date_range_start: stats.date_range?.[0],
					date_range_end: stats.date_range?.[1],
					file_types: stats.file_types,
					errors: 0,
					warnings: 0,
					status: 'success'
				};
				historyStore.addEntry(historyEntry);
				
				// Clear loading state after brief delay
				setTimeout(() => {
					isScanning = false;
					scanProgress = null;
				}, 500);
			}
		} catch (error) {
			console.error('Scan error:', error);
			isScanning = false;
			scanProgress = null;
		}
	}
	
	// Organization settings
	let orgStrategy = 'ByYearMonth';
	let orgMode = 'Copy';
	
	// Analysis settings
	let findDuplicates = true;
	let findMissingDates = true;
	let findCorrupted = false;

	// Navigate to Organize page
	function handleOrganize() {
		goto('/organize');
	}

	// Feature buttons (placeholder)
	function handleFeature(feature: string) {
		console.log(`${feature} clicked - coming soon!`);
	}
</script>

<style>
	.tools-panel {
		height: 100%;
		display: flex;
		flex-direction: column;
		background: #1a1a1a;
		color: #e0e0e0;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Helvetica Neue', Arial, sans-serif;
		overflow: hidden;
	}
	
	/* Scrollable content */
	.panel-content {
		flex: 1;
		overflow-y: auto;
		overflow-x: hidden;
	}
	
	.panel-content::-webkit-scrollbar {
		width: 8px;
	}
	
	.panel-content::-webkit-scrollbar-track {
		background: transparent;
	}
	
	.panel-content::-webkit-scrollbar-thumb {
		background: #3a3a3a;
		border-radius: 4px;
	}
	
	.panel-content::-webkit-scrollbar-thumb:hover {
		background: #4a4a4a;
	}
	
	/* Section */
	.section {
		border-bottom: 1px solid #2a2a2a;
	}
	
	.section:last-child {
		border-bottom: none;
	}
	
	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 14px 20px;
		cursor: pointer;
		user-select: none;
		transition: background 0.15s ease;
		background: #1a1a1a;
		border: none;
		width: 100%;
		text-align: left;
	}
	
	.section-header:hover {
		background: #222;
	}
	
	.section-header.active {
		background: #242424;
	}
	
	.section-title-group {
		display: flex;
		align-items: center;
		gap: 10px;
	}
	

	
	.section-title {
		font-size: 14px;
		font-weight: 500;
		color: #e0e0e0;
		margin: 0;
	}
	
	.section-chevron {
		font-size: 10px;
		color: #707070;
		transition: transform 0.2s ease;
	}
	
	.section-chevron.expanded {
		transform: rotate(90deg);
	}
	
	/* Section Content */
	.section-body {
		padding: 16px 20px 20px 20px;
		background: #1e1e1e;
	}
	
	/* Primary Button */
	.primary-btn {
		width: 100%;
		padding: 12px 16px;
		background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
		color: white;
		border: none;
		border-radius: 8px;
		font-size: 13px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.2s ease;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		box-shadow: 0 2px 8px rgba(59, 130, 246, 0.2);
	}
	
	.primary-btn:hover:not(:disabled) {
		background: linear-gradient(135deg, #2563eb 0%, #1d4ed8 100%);
		box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
		transform: translateY(-1px);
	}
	
	.primary-btn:active:not(:disabled) {
		transform: translateY(0);
	}
	
	.primary-btn:disabled {
		opacity: 0.7;
		cursor: not-allowed;
		background: linear-gradient(135deg, #6b7280 0%, #4b5563 100%);
	}
	
	/* Spinner Animation */
	.spinner {
		width: 16px;
		height: 16px;
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-top-color: white;
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}
	
	@keyframes spin {
		0% { transform: rotate(0deg); }
		100% { transform: rotate(360deg); }
	}
	
	/* Loading State */
	.loading-state {
		margin-top: 16px;
		padding: 16px;
		background: #242424;
		border-radius: 8px;
		border: 1px solid #3a3a3a;
	}
	
	.loading-bar {
		width: 100%;
		height: 4px;
		background: #3a3a3a;
		border-radius: 2px;
		overflow: hidden;
		margin-bottom: 12px;
	}
	
	.loading-bar-fill {
		height: 100%;
		background: linear-gradient(90deg, #3b82f6, #60a5fa);
		border-radius: 2px;
		transition: width 0.3s ease;
		animation: pulse 1.5s ease-in-out infinite;
	}
	
	@keyframes pulse {
		0%, 100% { opacity: 1; }
		50% { opacity: 0.7; }
	}
	
	.loading-text {
		margin: 0;
		font-size: 12px;
		color: #a0a0a0;
		text-align: center;
	}
	
	.loading-stats {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 8px;
	}
	
	.loading-count {
		font-size: 13px;
		color: #d0d0d0;
		font-weight: 500;
		font-variant-numeric: tabular-nums;
	}
	
	.loading-percentage {
		font-size: 13px;
		color: #3b82f6;
		font-weight: 600;
		font-variant-numeric: tabular-nums;
	}
	
	.loading-file {
		margin: 0;
		font-size: 11px;
		color: #707070;
		text-align: left;
		font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}
	
	/* Secondary Button */
	.secondary-btn {
		width: 100%;
		padding: 10px 16px;
		background: #262626;
		color: #e0e0e0;
		border: 1px solid #3a3a3a;
		border-radius: 6px;
		font-size: 13px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
	}
	
	.secondary-btn:hover {
		background: #2a2a2a;
		border-color: #4a4a4a;
	}
	
	.secondary-btn:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}
	
	/* Stats Group */
	.stats-group {
		margin-bottom: 16px;
	}
	
	.stats-group:last-child {
		margin-bottom: 0;
	}
	
	.stats-header {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-bottom: 12px;
		padding-bottom: 8px;
		border-bottom: 1px solid #2a2a2a;
	}
	

	
	.stats-header-title {
		font-size: 12px;
		font-weight: 600;
		color: #a0a0a0;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}
	
	/* Stat Row */
	.stat-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 8px 0;
	}
	
	/* Folder Path */
	.folder-path {
		display: flex;
		flex-direction: column;
		gap: 4px;
		padding: 12px;
		background: #242424;
		border-radius: 6px;
		margin-bottom: 12px;
		border-left: 3px solid #3b82f6;
	}
	
	.path-label {
		font-size: 11px;
		font-weight: 600;
		color: #707070;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}
	
	.path-value {
		font-size: 12px;
		color: #d0d0d0;
		font-family: 'Monaco', 'Menlo', 'Consolas', monospace;
		word-break: break-all;
		line-height: 1.5;
	}
	
	.stat-label {
		font-size: 13px;
		color: #909090;
		display: flex;
		align-items: center;
		gap: 6px;
	}
	
	.stat-value {
		font-size: 13px;
		font-weight: 500;
		color: #e0e0e0;
		font-variant-numeric: tabular-nums;
	}
	
	.stat-meta {
		font-size: 12px;
		color: #707070;
		margin-left: 6px;
	}
	
	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 24px 16px;
	}
	
	.empty-text {
		font-size: 13px;
		color: #707070;
		line-height: 1.5;
	}
	
	/* Button Stack */
	.button-stack {
		display: flex;
		flex-direction: column;
		gap: 10px;
		margin-top: 16px;
	}
	
	/* Settings */
	.setting-group {
		margin-bottom: 16px;
	}
	
	.setting-group:last-child {
		margin-bottom: 0;
	}
	
	.setting-label {
		font-size: 12px;
		font-weight: 600;
		color: #a0a0a0;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		margin-bottom: 10px;
		display: block;
	}
	
	.option {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 8px 12px;
		margin-bottom: 6px;
		border-radius: 6px;
		cursor: pointer;
		transition: background 0.15s ease;
	}
	
	.option:hover {
		background: #242424;
	}
	
	.option input {
		cursor: pointer;
		margin: 0;
	}
	
	.option label {
		font-size: 13px;
		color: #d0d0d0;
		cursor: pointer;
		flex: 1;
		margin: 0;
	}
	
	/* Info Box */
	.info-box {
		padding: 12px;
		background: #242424;
		border-left: 3px solid #3b82f6;
		border-radius: 4px;
		font-size: 12px;
		color: #a0a0a0;
		line-height: 1.5;
		margin-top: 16px;
	}
	
	/* Spacing utilities */
	.mt-16 { margin-top: 16px; }
	.mb-16 { margin-bottom: 16px; }
</style>

<div class="tools-panel">
	<!-- Scrollable Content -->
	<div class="panel-content">
		<!-- WORKSPACE -->
		<div class="section">
			<button 
				class="section-header"
				class:active={expandedSections.workspace}
				on:click={() => toggleSection('workspace')}
			>
				<div class="section-title-group">
					<h3 class="section-title">Workspace</h3>
				</div>
				<span class="section-chevron" class:expanded={expandedSections.workspace}>▶</span>
			</button>
			
			{#if expandedSections.workspace}
				<div class="section-body">
					<!-- Scan Button with Loading State -->
					<button 
						class="primary-btn" 
						on:click={handleScanFolder}
						disabled={isScanning}
					>
						{#if isScanning}
							<span class="spinner"></span>
							<span>Scanning...</span>
						{:else}
							Scan Folder
						{/if}
					</button>
					
					<!-- Loading Progress -->
					{#if isScanning && scanProgress}
						<div class="loading-state">
							<!-- Progress Bar -->
							<div class="loading-bar">
								<div class="loading-bar-fill" style="width: {scanProgress.percentage}%"></div>
							</div>
							
							<!-- File Count -->
							<div class="loading-stats">
								<span class="loading-count">
									{formatNumber(scanProgress.processed_files)} / {formatNumber(scanProgress.total_files)} files
								</span>
								<span class="loading-percentage">
									{scanProgress.percentage.toFixed(0)}%
								</span>
							</div>
							
							<!-- Current File -->
							{#if scanProgress.current_file}
								<p class="loading-file">{scanProgress.current_file}</p>
							{/if}
						</div>
					{/if}
					
					{#if scanStats}
						<!-- Scan Summary -->
						<div class="stats-group mt-16">
							<div class="stats-header">
								<span class="stats-header-title">Summary</span>
							</div>
							
							<!-- Folder Path -->
							{#if scanResult?.root_path}
								<div class="folder-path">
									<span class="path-label">Working Folder</span>
									<span class="path-value" title="{scanResult.root_path}">
										{formatPath(scanResult.root_path)}
									</span>
								</div>
							{/if}
							
							<div class="stat-row">
								<span class="stat-label">Total Files</span>
								<span class="stat-value">{formatNumber(scanStats.total_files)}</span>
							</div>
							
							<div class="stat-row">
								<span class="stat-label">Total Size</span>
								<span class="stat-value">{formatBytes(scanStats.total_size)}</span>
							</div>
							
							{#if scanStats.date_range}
								<div class="stat-row">
									<span class="stat-label">Date Range</span>
									<span class="stat-value">{formatDateRange(scanStats.date_range)}</span>
								</div>
							{/if}
						</div>
						
						<!-- File Types -->
						{#if Object.values(scanStats.file_types).some(count => count > 0)}
							<div class="stats-group">
								<div class="stats-header">
									<span class="stats-header-title">File Types</span>
								</div>
								
								{#each Object.entries(scanStats.file_types) as [type, count]}
									{#if count > 0}
										<div class="stat-row">
											<span class="stat-label">{type.charAt(0).toUpperCase() + type.slice(1)}</span>
											<span class="stat-value">{formatNumber(count)}</span>
										</div>
									{/if}
								{/each}
							</div>
						{/if}
						
						<!-- Quality -->
						<div class="stats-group">
							<div class="stats-header">
								<span class="stats-header-title">Quality</span>
							</div>
							
                                                        {#if scanStats.quality}
                                                                {#if scanStats.quality.screenshots > 0}
                                                                        <div class="stat-row">
                                                                                <span class="stat-label">📱 Screenshots</span>
                                                                                <span class="stat-value">
                                                                                        {formatNumber(scanStats.quality.screenshots)} 
                                                                                        <span class="stat-meta">
                                                                                                ({((scanStats.quality.screenshots / scanStats.total_files) * 100).toFixed(1)}%)
                                                                                        </span>
                                                                                </span>
                                                                        </div>
                                                                {/if}

                                                                {#if scanStats.quality.duplicates > 0}
                                                                        <div class="stat-row">
                                                                                <span class="stat-label">🔄 Duplicates</span>
                                                                                <span class="stat-value">
                                                                                        {formatNumber(scanStats.quality.duplicates)}
                                                                                        <span class="stat-meta">
                                                                                                ({((scanStats.quality.duplicates / scanStats.total_files) * 100).toFixed(1)}%)
                                                                                        </span>
                                                                                </span>
                                                                        </div>
                                                                {/if}

                                                                {#if scanStats.quality.low_resolution > 0}
                                                                        <div class="stat-row">
                                                                                <span class="stat-label">📉 Low Resolution</span>
                                                                                <span class="stat-value">
                                                                                        {formatNumber(scanStats.quality.low_resolution)}
                                                                                        <span class="stat-meta">
                                                                                                ({((scanStats.quality.low_resolution / scanStats.file_types.images) * 100).toFixed(1)}%)
                                                                                        </span>
                                                                                </span>
                                                                        </div>
                                                                {/if}

                                                                {#if scanStats.quality.small_files > 0}
                                                                        <div class="stat-row">
                                                                                <span class="stat-label">💾 Small Files</span>
                                                                                <span class="stat-value">
                                                                                        {formatNumber(scanStats.quality.small_files)}
                                                                                        <span class="stat-meta">
                                                                                                ({((scanStats.quality.small_files / scanStats.file_types.images) * 100).toFixed(1)}%)
                                                                                        </span>
                                                                                </span>
                                                                        </div>
                                                                {/if}

                                                                {#if scanStats.quality.missing_metadata > 0}
                                                                        <div class="stat-row">
                                                                                <span class="stat-label">📅 Missing Date</span>
                                                                                <span class="stat-value">
                                                                                        {formatNumber(scanStats.quality.missing_metadata)}
                                                                                        <span class="stat-meta">
                                                                                                ({((scanStats.quality.missing_metadata / scanStats.file_types.images) * 100).toFixed(1)}%)
                                                                                        </span>
                                                                                </span>
                                                                        </div>
                                                                {/if}

                                                                {#if scanStats.quality.potential_memes > 0}
                                                                        <div class="stat-row">
                                                                                <span class="stat-label">🎭 Potential Memes</span>
                                                                                <span class="stat-value">
                                                                                        {formatNumber(scanStats.quality.potential_memes)}
                                                                                        <span class="stat-meta">
                                                                                                ({((scanStats.quality.potential_memes / scanStats.total_files) * 100).toFixed(1)}%)
                                                                                        </span>
                                                                                </span>
                                                                        </div>
                                                                {/if}

                                                                {#if scanStats.quality.screenshots === 0 && 
                                                                     scanStats.quality.duplicates === 0 && 
                                                                     scanStats.quality.low_resolution === 0 &&
                                                                     scanStats.quality.small_files === 0 &&
                                                                     scanStats.quality.missing_metadata === 0 &&
                                                                     scanStats.quality.potential_memes === 0}
                                                                        <div class="stat-row">
                                                                                <span class="stat-value" style="color: #10b981; font-style: italic;">
                                                                                        ✨ No issues detected
                                                                                </span>
                                                                        </div>
                                                                {/if}
                                                        {:else}
                                                                <!-- Fallback for old format -->
                                                                <div class="stat-row">
                                                                        <span class="stat-label">Screenshots</span>
                                                                        <span class="stat-value">
                                                                                {formatNumber(scanStats.screenshots)} 
                                                                                <span class="stat-meta">
                                                                                        ({((scanStats.screenshots / scanStats.total_files) * 100).toFixed(1)}%)
                                                                                </span>
                                                                        </span>
                                                                </div>

                                                                <div class="stat-row">
                                                                        <span class="stat-label">Duplicates</span>
                                                                        <span class="stat-value">
                                                                                {formatNumber(scanStats.duplicates)}
                                                                                <span class="stat-meta">
                                                                                        ({((scanStats.duplicates / scanStats.total_files) * 100).toFixed(1)}%)
                                                                                </span>
                                                                        </span>
                                                                </div>
                                                        {/if}
							</div>

							<!-- Actions -->
							<div class="button-stack">
								<button class="secondary-btn" on:click={handleScanFolder}>
									Scan Another
								</button>
							<button class="secondary-btn" on:click={() => fileStore.clear()}>
								Clear Workspace
							</button>
							<button class="secondary-btn" on:click={handleOrganize} disabled={!hasFiles}>
								Organize
							</button>
						</div>
					{:else}
						<div class="empty-state">
						<p class="empty-text">No files loaded.<br/>Scan a folder to begin.</p>
						</div>
					{/if}
				</div>
			{/if}
		</div>
		
		<!-- ANALYZE -->
		<div class="section">
			<button 
				class="section-header"
				class:active={expandedSections.analyze}
				on:click={() => toggleSection('analyze')}
			>
				<div class="section-title-group">
					<h3 class="section-title">Analyze</h3>
				</div>
				<span class="section-chevron" class:expanded={expandedSections.analyze}>▶</span>
			</button>
			
			{#if expandedSections.analyze}
				<div class="section-body">
					<div class="setting-group">
						<span class="setting-label">Detection Options</span>
						
						<label class="option">
							<input type="checkbox" bind:checked={findDuplicates} />
							<span>Find duplicate files</span>
						</label>
						
						<label class="option">
							<input type="checkbox" bind:checked={findMissingDates} />
							<span>Find missing dates</span>
						</label>
						
						<label class="option">
							<input type="checkbox" bind:checked={findCorrupted} />
							<span>Check for corrupted files</span>
						</label>
					</div>
					
					{#if !hasFiles}
						<div class="info-box">
							Load files first to run analysis.
						</div>
					{/if}
				</div>
			{/if}
		</div>
		
		<!-- ORGANIZE -->
		<div class="section">
			<button 
				class="section-header"
				class:active={expandedSections.organize}
				on:click={() => toggleSection('organize')}
			>
				<div class="section-title-group">
					<h3 class="section-title">Organize</h3>
				</div>
				<span class="section-chevron" class:expanded={expandedSections.organize}>▶</span>
			</button>
			
			{#if expandedSections.organize}
				<div class="section-body">
					<div class="setting-group">
						<span class="setting-label">Strategy</span>
						
						<label class="option">
							<input type="radio" bind:group={orgStrategy} value="ByDate" />
							<span>By Date (YYYY-MM-DD)</span>
						</label>
						
						<label class="option">
							<input type="radio" bind:group={orgStrategy} value="ByYear" />
							<span>By Year (YYYY)</span>
						</label>
						
						<label class="option">
							<input type="radio" bind:group={orgStrategy} value="ByYearMonth" />
							<span>By Year-Month (YYYY-MM)</span>
						</label>
						
						<label class="option">
							<input type="radio" bind:group={orgStrategy} value="ByFileType" />
							<span>By File Type</span>
						</label>
					</div>
					
					<div class="setting-group">
						<span class="setting-label">Mode</span>
						
						<label class="option">
							<input type="radio" bind:group={orgMode} value="Copy" />
							<span>Copy (keeps originals)</span>
						</label>
						
						<label class="option">
							<input type="radio" bind:group={orgMode} value="Move" />
							<span>Move (removes originals)</span>
						</label>
					</div>
					
					{#if !hasFiles}
						<div class="info-box">
							Load files first to organize them.
						</div>
					{/if}
				</div>
			{/if}
		</div>
		
		<!-- REVIEW -->
		<div class="section">
			<button 
				class="section-header"
				class:active={expandedSections.review}
				on:click={() => toggleSection('review')}
			>
				<div class="section-title-group">
					<h3 class="section-title">Review</h3>
				</div>
				<span class="section-chevron" class:expanded={expandedSections.review}>▶</span>
			</button>
			
			{#if expandedSections.review}
				<div class="section-body">
					<button 
						class="secondary-btn" 
						disabled={!hasFiles}
						on:click={() => goto('/review')}
					>
						View Audit Trail
					</button>
					
					<div class="info-box">
						All changes are logged and reversible.
					</div>
				</div>
			{/if}
		</div>
		
		<!-- EXPORT -->
		<div class="section">
			<button 
				class="section-header"
				class:active={expandedSections.export}
				on:click={() => toggleSection('export')}
			>
				<div class="section-title-group">
					<h3 class="section-title">Export</h3>
				</div>
				<span class="section-chevron" class:expanded={expandedSections.export}>▶</span>
			</button>
			
			{#if expandedSections.export}
				<div class="section-body">
					<button class="secondary-btn" disabled={!hasFiles}>
						Export Report
					</button>
					
					<button class="secondary-btn mt-16" disabled={!hasFiles}>
						Generate Statistics
					</button>
					
					{#if !hasFiles}
						<div class="info-box">
							Load files first to export data.
						</div>
					{/if}
				</div>
			{/if}
		</div>
	</div>
</div>
