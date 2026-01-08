<script lang="ts">
	import { goto } from '$app/navigation';
	import { fileStore } from '$lib/stores/photoStore';
	import { historyStore } from '$lib/stores/historyStore';
	import type { HistoryEntry } from '$lib/stores/historyStore';
	import { invoke } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';

	let scanResult: any = null;

	fileStore.subscribe(value => {
		scanResult = value;
	});

	$: hasFiles = scanResult?.files?.length > 0;
	$: totalFiles = scanResult?.files?.length || 0;

	// Calculate statistics
	$: scanStats = calculateStats(scanResult);

	function calculateStats(result: any) {
		if (!result || !result.files || result.files.length === 0) {
			return null;
		}

		const files = result.files;
		const totalSize = files.reduce((sum: number, file: any) => sum + (file.size || 0), 0);
		
		// Get date range
		const dates = files
			.map((f: any) => f.modified)
			.filter((d: any) => d)
			.sort();
		const oldestDate = dates.length > 0 ? dates[0] : null;
		const newestDate = dates.length > 0 ? dates[dates.length - 1] : null;

		// Count file types
		const typeCounts: Record<string, number> = {};
		files.forEach((file: any) => {
			if (!file || !file.name) return;
			const ext = file.name.split('.').pop()?.toLowerCase() || 'unknown';
			const type = getFileType(ext);
			typeCounts[type] = (typeCounts[type] || 0) + 1;
		});

		// Quality analysis
		const screenshots = files.filter((f: any) => 
			f && f.name && (
				f.name.toLowerCase().includes('screenshot') || 
				f.name.toLowerCase().includes('screen shot')
			)
		).length;
		
		// Simple duplicate detection (same size)
		const sizeCounts: Record<number, number> = {};
		files.forEach((file: any) => {
			if (!file) return;
			const size = file.size || 0;
			sizeCounts[size] = (sizeCounts[size] || 0) + 1;
		});
		const duplicates = Object.values(sizeCounts).filter(count => count > 1).reduce((sum, count) => sum + count - 1, 0);

		return {
			totalFiles: files.length,
			totalSize,
			oldestDate,
			newestDate,
			typeCounts,
			screenshots,
			screenshotPercentage: files.length > 0 ? ((screenshots / files.length) * 100).toFixed(1) : '0.0',
			duplicates,
			duplicatePercentage: files.length > 0 ? ((duplicates / files.length) * 100).toFixed(1) : '0.0'
		};
	}

	function getFileType(ext: string): string {
		const imageExts = ['jpg', 'jpeg', 'png', 'gif', 'bmp', 'webp', 'heic', 'heif', 'svg'];
		const videoExts = ['mp4', 'mov', 'avi', 'mkv', 'webm', 'm4v'];
		const docExts = ['pdf', 'doc', 'docx', 'txt', 'md'];
		const audioExts = ['mp3', 'wav', 'flac', 'm4a', 'aac'];

		if (imageExts.includes(ext)) return 'images';
		if (videoExts.includes(ext)) return 'videos';
		if (docExts.includes(ext)) return 'documents';
		if (audioExts.includes(ext)) return 'audio';
		return 'other';
	}

	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return (bytes / Math.pow(k, i)).toFixed(1) + ' ' + sizes[i];
	}

	function formatNumber(num: number): string {
		return num.toLocaleString();
	}

	function formatDate(timestamp: number): string {
		const date = new Date(timestamp * 1000);
		return date.toLocaleDateString('en-US', { month: 'short', year: 'numeric' });
	}

	function getTypeIcon(type: string): string {
		const icons: Record<string, string> = {
			images: '🖼️',
			videos: '🎬',
			documents: '📄',
			audio: '🎵',
			other: '📦'
		};
		return icons[type] || '📦';
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
		console.log('Scan folder clicked');
		try {
			const selected = await open({
				directory: true,
				multiple: false,
				title: 'Select folder to scan'
			});

			console.log('Selected path:', selected);

			if (selected) {
				console.log('Invoking scan_directory...');
				const result = await invoke('scan_directory', { path: selected });
				console.log('Scan result:', result);
				fileStore.setScanResult(result);
				console.log('Store updated');

				// Add to history
				const historyEntry: HistoryEntry = {
					id: Date.now().toString(),
					timestamp: Date.now(),
					path: selected,
					action: 'scan',
					fileCount: (result as any).files?.length || 0
				};
				historyStore.add(historyEntry);
			}
		} catch (error) {
			console.error('Error scanning folder:', error);
		}
	}

	// Navigate to Organize page
	function handleOrganize() {
		console.log('Organize clicked');
		goto('/organize');
	}

	// Feature buttons (placeholder)
	function handleFeature(feature: string) {
		console.log(`${feature} clicked - coming soon!`);
	}
</script>

<div class="tools-panel">
	<div class="tools-header">
		<h2>🛠️ Tools</h2>
	</div>

	<!-- Workspace Section -->
	<div class="tool-section">
		<button class="section-header" on:click={() => toggleSection('workspace')}>
			<span class="section-title">
				<span class="icon">📁</span>
				Workspace
			</span>
			<span class="toggle-icon">{expandedSections.workspace ? '▼' : '▶'}</span>
		</button>

		{#if expandedSections.workspace}
			<div class="section-content">
				<button class="tool-button primary" on:click={handleScanFolder}>
					🔍 Scan Folder
				</button>

				{#if scanStats}
					<div class="scan-results">
						<!-- Summary Section -->
						<div class="stat-group">
							<h4>📊 Scan Summary</h4>
							<div class="stat-item">
								<span class="stat-label">Total Files:</span>
								<span class="stat-value">{formatNumber(scanStats.totalFiles)}</span>
							</div>
							<div class="stat-item">
								<span class="stat-label">Total Size:</span>
								<span class="stat-value">{formatBytes(scanStats.totalSize)}</span>
							</div>
							{#if scanStats.oldestDate && scanStats.newestDate}
								<div class="stat-item">
									<span class="stat-label">Date Range:</span>
									<span class="stat-value">
										{formatDate(scanStats.oldestDate)} - {formatDate(scanStats.newestDate)}
									</span>
								</div>
							{/if}
						</div>

						<!-- File Types Section -->
						{#if Object.keys(scanStats.typeCounts).length > 0}
							<div class="stat-group">
								<h4>📁 File Types</h4>
								{#each Object.entries(scanStats.typeCounts) as [type, count]}
									<div class="stat-item">
										<span class="stat-label">
											{getTypeIcon(type)} {type.charAt(0).toUpperCase() + type.slice(1)}:
										</span>
										<span class="stat-value">{formatNumber(count)}</span>
									</div>
								{/each}
							</div>
						{/if}

						<!-- Quality Analysis Section -->
						<div class="stat-group">
							<h4>🔍 Quality Analysis</h4>
							{#if scanStats.screenshots > 0}
								<div class="stat-item">
									<span class="stat-label">📱 Screenshots:</span>
									<span class="stat-value">
										{formatNumber(scanStats.screenshots)} ({scanStats.screenshotPercentage}%)
									</span>
								</div>
							{/if}
							{#if scanStats.duplicates > 0}
								<div class="stat-item">
									<span class="stat-label">🔄 Duplicates:</span>
									<span class="stat-value">
										{formatNumber(scanStats.duplicates)} ({scanStats.duplicatePercentage}%)
									</span>
								</div>
							{/if}
							{#if scanStats.screenshots === 0 && scanStats.duplicates === 0}
								<div class="stat-item">
									<span class="stat-value subtle">✨ No issues detected</span>
								</div>
							{/if}
						</div>
					</div>
				{:else if totalFiles > 0}
					<p class="file-count">Files Loaded: {totalFiles}</p>
				{/if}

				<button 
					class="tool-button" 
					on:click={handleOrganize}
					disabled={!hasFiles}
				>
					📂 Organize
				</button>
			</div>
		{/if}
	</div>

	<!-- Analyze Section -->
	<div class="tool-section">
		<button class="section-header" on:click={() => toggleSection('analyze')}>
			<span class="section-title">
				<span class="icon">🔍</span>
				Analyze
			</span>
			<span class="toggle-icon">{expandedSections.analyze ? '▼' : '▶'}</span>
		</button>

		{#if expandedSections.analyze}
			<div class="section-content">
				<button class="tool-button" on:click={() => handleFeature('Find Duplicates')}>
					🔄 Find Duplicates
				</button>
				<button class="tool-button" on:click={() => handleFeature('Detect Screenshots')}>
					📱 Detect Screenshots
				</button>
				<button class="tool-button" on:click={() => handleFeature('Check Quality')}>
					⭐ Check Quality
				</button>
			</div>
		{/if}
	</div>

	<!-- Organize Section -->
	<div class="tool-section">
		<button class="section-header" on:click={() => toggleSection('organize')}>
			<span class="section-title">
				<span class="icon">📂</span>
				Organize
			</span>
			<span class="toggle-icon">{expandedSections.organize ? '▼' : '▶'}</span>
		</button>

		{#if expandedSections.organize}
			<div class="section-content">
				<button class="tool-button" on:click={() => handleFeature('By Date')}>
					📅 By Date
				</button>
				<button class="tool-button" on:click={() => handleFeature('By Type')}>
					📁 By Type
				</button>
				<button class="tool-button" on:click={() => handleFeature('By Location')}>
					📍 By Location
				</button>
			</div>
		{/if}
	</div>

	<!-- Review Section -->
	<div class="tool-section">
		<button class="section-header" on:click={() => toggleSection('review')}>
			<span class="section-title">
				<span class="icon">👁️</span>
				Review
			</span>
			<span class="toggle-icon">{expandedSections.review ? '▼' : '▶'}</span>
		</button>

		{#if expandedSections.review}
			<div class="section-content">
				<button class="tool-button" on:click={() => handleFeature('Preview Changes')}>
					👀 Preview Changes
				</button>
				<button class="tool-button" on:click={() => handleFeature('Mark for Deletion')}>
					🗑️ Mark for Deletion
				</button>
			</div>
		{/if}
	</div>

	<!-- Export Section -->
	<div class="tool-section">
		<button class="section-header" on:click={() => toggleSection('export')}>
			<span class="section-title">
				<span class="icon">💾</span>
				Export
			</span>
			<span class="toggle-icon">{expandedSections.export ? '▼' : '▶'}</span>
		</button>

		{#if expandedSections.export}
			<div class="section-content">
				<button class="tool-button" on:click={() => handleFeature('Generate Report')}>
					📊 Generate Report
				</button>
				<button class="tool-button" on:click={() => handleFeature('Export List')}>
					📋 Export List
				</button>
			</div>
		{/if}
	</div>
</div>

<style>
	.tools-panel {
		height: 100%;
		background: var(--bg-secondary);
		overflow-y: auto;
		padding: var(--spacing-sm);
		display: flex;
		flex-direction: column;
		gap: var(--spacing-xs);
	}

	.tools-header {
		padding: var(--spacing-sm);
		border-bottom: 1px solid var(--border-color);
		margin-bottom: var(--spacing-sm);
	}

	.tools-header h2 {
		margin: 0;
		font-size: 1.25rem;
		color: var(--text-primary);
	}

	.tool-section {
		background: var(--bg-primary);
		border-radius: var(--border-radius);
		overflow: hidden;
		border: 1px solid var(--border-color);
	}

	.section-header {
		width: 100%;
		padding: var(--spacing-sm);
		background: var(--bg-primary);
		border: none;
		cursor: pointer;
		display: flex;
		justify-content: space-between;
		align-items: center;
		color: var(--text-primary);
		font-size: 1rem;
		transition: background 0.2s;
	}

	.section-header:hover {
		background: var(--bg-hover);
	}

	.section-title {
		display: flex;
		align-items: center;
		gap: var(--spacing-xs);
		font-weight: 500;
	}

	.icon {
		font-size: 1.2rem;
	}

	.toggle-icon {
		font-size: 0.8rem;
		color: var(--text-secondary);
	}

	.section-content {
		padding: var(--spacing-sm);
		display: flex;
		flex-direction: column;
		gap: var(--spacing-xs);
		background: var(--bg-secondary);
	}

	.tool-button {
		padding: var(--spacing-sm);
		background: var(--bg-primary);
		border: 1px solid var(--border-color);
		border-radius: var(--border-radius);
		color: var(--text-primary);
		cursor: pointer;
		transition: all 0.2s;
		font-size: 0.9rem;
		text-align: left;
	}

	.tool-button:hover:not(:disabled) {
		background: var(--bg-hover);
		border-color: var(--accent);
	}

	.tool-button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.tool-button.primary {
		background: var(--accent);
		color: white;
		border-color: var(--accent);
		font-weight: 500;
	}

	.tool-button.primary:hover:not(:disabled) {
		background: var(--accent-hover);
		transform: translateY(-1px);
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
	}

	.scan-results {
		margin-top: var(--spacing-sm);
		padding: var(--spacing-sm);
		background: var(--bg-primary);
		border-radius: var(--border-radius);
		border: 1px solid var(--border-color);
		display: flex;
		flex-direction: column;
		gap: var(--spacing-md);
	}

	.stat-group {
		display: flex;
		flex-direction: column;
		gap: var(--spacing-xs);
	}

	.stat-group h4 {
		margin: 0;
		font-size: 0.9rem;
		color: var(--text-secondary);
		font-weight: 600;
		padding-bottom: var(--spacing-xs);
		border-bottom: 1px solid var(--border-color);
	}

	.stat-item {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: var(--spacing-xs) 0;
		font-size: 0.85rem;
	}

	.stat-label {
		color: var(--text-secondary);
		flex: 1;
	}

	.stat-value {
		color: var(--text-primary);
		font-weight: 500;
		text-align: right;
	}

	.stat-value.subtle {
		color: var(--text-secondary);
		font-style: italic;
		font-weight: normal;
	}

	.file-count {
		margin: var(--spacing-sm) 0;
		padding: var(--spacing-sm);
		background: var(--bg-primary);
		border-radius: var(--border-radius);
		border: 1px solid var(--border-color);
		color: var(--text-primary);
		font-size: 0.9rem;
	}

	/* Scrollbar styling */
	.tools-panel::-webkit-scrollbar {
		width: 8px;
	}

	.tools-panel::-webkit-scrollbar-track {
		background: var(--bg-secondary);
	}

	.tools-panel::-webkit-scrollbar-thumb {
		background: var(--border-color);
		border-radius: 4px;
	}

	.tools-panel::-webkit-scrollbar-thumb:hover {
		background: var(--text-secondary);
	}
</style>
