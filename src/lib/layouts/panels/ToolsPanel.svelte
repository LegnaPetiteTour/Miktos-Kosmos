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
					timestamp: new Date().toISOString(),
					folder_path: selected,
					total_files: result.files?.length || 0,
					total_size: result.stats?.total_size || 0,
					date_range_start: result.stats?.date_range_start,
					date_range_end: result.stats?.date_range_end,
					file_types: result.stats?.file_types || {
						images: 0,
						videos: 0,
						documents: 0,
						audio: 0,
						archives: 0,
						other: 0
					},
					errors: 0,
					warnings: 0,
					status: 'success'
				};
				
				historyStore.addEntry(historyEntry);
				console.log('History entry added');
			} else {
				console.log('No folder selected');
			}
		} catch (error) {
			console.error('Scan error:', error);
			alert(`Error scanning folder: ${error}`);
		}
	}
	
	// Organization settings
	let orgStrategy = 'ByYearMonth';
	let orgMode = 'Copy';
	
	// Analysis settings
	let findDuplicates = true;
	let findMissingDates = true;
	let findCorrupted = false;
</script>

<style>
	.tools-container {
		display: flex;
		flex-direction: column;
		gap: 0;
		height: 100%;
		overflow-y: auto;
		padding: 0;
	}
	
	.tool-section {
		border-bottom: 1px solid var(--panel-border);
	}
	
	.section-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--space-3) var(--space-4);
		background-color: var(--bg-subtle);
		cursor: pointer;
		user-select: none;
		transition: background-color var(--transition-fast);
	}
	
	.section-header:hover {
		background-color: var(--panel);
	}
	
	.section-header.expanded {
		background-color: var(--panel);
	}
	
	.section-title {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: var(--text);
	}
	
	.section-icon {
		font-size: 10px;
		color: var(--text-muted);
		transition: transform var(--transition-fast);
	}
	
	.section-icon.expanded {
		transform: rotate(90deg);
	}
	
	.section-content {
		padding: var(--space-4);
		background-color: var(--panel);
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
	}
	
	.tool-button {
		width: 100%;
		padding: var(--space-3);
		background-color: var(--panel);
		color: var(--text);
		border: 1px solid var(--border);
		border-radius: var(--radius-md);
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		cursor: pointer;
		transition: all var(--transition-fast);
	}
	
	.tool-button:hover:not(:disabled) {
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
		transform: translateY(-1px);
	}
	
	.tool-button:disabled {
		background-color: var(--bg-subtle);
		color: var(--text-muted);
		cursor: not-allowed;
		transform: none;
		opacity: 0.5;
	}
	
	.setting-group {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}
	
	.setting-label {
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
		color: var(--text);
		margin-bottom: var(--space-1);
	}
	
	.radio-group {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
	}
	
	.radio-option {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2);
		border-radius: var(--radius-sm);
		cursor: pointer;
		transition: all var(--transition-fast);
	}
	
	.radio-option:hover {
		background: rgba(255, 255, 255, 0.03);
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
	}
	
	.radio-option input[type="radio"] {
		cursor: pointer;
	}
	
	.radio-option label {
		font-size: var(--text-xs);
		color: var(--text);
		cursor: pointer;
	}
	
	.checkbox-option {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding: var(--space-2);
		border-radius: var(--radius-sm);
		cursor: pointer;
		transition: all var(--transition-fast);
	}
	
	.checkbox-option:hover {
		background: rgba(255, 255, 255, 0.03);
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
	}
	
	.checkbox-option input[type="checkbox"] {
		cursor: pointer;
	}
	
	.checkbox-option label {
		font-size: var(--text-xs);
		color: var(--text);
		cursor: pointer;
	}
	
	.info-text {
		font-size: var(--text-xs);
		color: var(--text-muted);
		padding: var(--space-2);
		background-color: var(--bg-subtle);
		border-radius: var(--radius-sm);
		border-left: 2px solid var(--text-muted);
	}
	
	.stat-item {
		display: flex;
		justify-content: space-between;
		padding: var(--space-2);
		background-color: var(--bg-subtle);
		border-radius: var(--radius-sm);
		font-size: var(--text-xs);
	}
	
	.stat-label {
		color: var(--text-muted);
	}
	
	.stat-value {
		color: var(--text);
		font-weight: var(--weight-medium);
	}
	
	.tool-group {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		padding: var(--space-3);
		background-color: var(--bg-subtle);
		border-radius: var(--radius-md);
		border: 1px solid var(--panel-border);
	}
	
	.tool-group-header {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		margin-bottom: var(--space-1);
	}
	
	.tool-icon {
		font-size: 16px;
		opacity: 0.8;
	}
	
	.tool-group-title {
		font-size: var(--text-sm);
		font-weight: var(--weight-semibold);
		color: var(--text);
	}
	
	.tool-group-description {
		font-size: var(--text-xs);
		color: var(--text-muted);
		margin-bottom: var(--space-2);
	}
	
	.tool-meta {
		font-size: var(--text-xs);
		color: var(--text-muted);
		padding-top: var(--space-1);
	}
</style>

<div class="tools-container">
	<!-- WORKSPACE SECTION -->
	<div class="tool-section">
		<div 
			class="section-header"
			class:expanded={expandedSections.workspace}
			on:click={() => toggleSection('workspace')}
		>
			<h3 class="section-title">Workspace</h3>
			<span class="section-icon" class:expanded={expandedSections.workspace}>▶</span>
		</div>
		
		{#if expandedSections.workspace}
			<div class="section-content">
				<button class="tool-button" on:click={handleScanFolder}>
					📁 Scan Folder
				</button>
				
				{#if hasFiles}
					<div class="stat-item">
						<span class="stat-label">Files Loaded:</span>
						<span class="stat-value">{totalFiles}</span>
					</div>
					
					<button class="tool-button" on:click={() => fileStore.clear()}>
						Clear Workspace
					</button>
				{:else}
					<div class="info-text">
						No files loaded. Scan a folder to begin.
					</div>
				{/if}
			</div>
		{/if}
	</div>
	
	<!-- ANALYZE SECTION -->
	<div class="tool-section">
		<div 
			class="section-header"
			class:expanded={expandedSections.analyze}
			on:click={() => toggleSection('analyze')}
		>
			<h3 class="section-title">Analyze</h3>
			<span class="section-icon" class:expanded={expandedSections.analyze}>▶</span>
		</div>
		
		{#if expandedSections.analyze}
			<div class="section-content">
				<div class="setting-group">
					<div class="setting-label">Detection Options:</div>
					
					<label class="checkbox-option">
						<input type="checkbox" bind:checked={findDuplicates} />
						<span>Find duplicate files</span>
					</label>
					
					<label class="checkbox-option">
						<input type="checkbox" bind:checked={findMissingDates} />
						<span>Find missing dates</span>
					</label>
					
					<label class="checkbox-option">
						<input type="checkbox" bind:checked={findCorrupted} />
						<span>Check for corrupted files</span>
					</label>
					
					<label class="checkbox-option">
						<input type="checkbox" bind:checked={findCorrupted} />
						<span>Generate analysis report</span>
					</label>
				</div>
				
				{#if !hasFiles}
					<div class="info-text">
						Load files first to run analysis.
					</div>
				{/if}
			</div>
		{/if}
	</div>
	
	<!-- ORGANIZE SECTION -->
	<div class="tool-section">
		<div 
			class="section-header"
			class:expanded={expandedSections.organize}
			on:click={() => toggleSection('organize')}
		>
			<h3 class="section-title">Organize</h3>
			<span class="section-icon" class:expanded={expandedSections.organize}>▶</span>
		</div>
		
		{#if expandedSections.organize}
			<div class="section-content">
				<div class="setting-group">
					<div class="setting-label">Organization Strategy:</div>
					
					<div class="radio-group">
						<label class="radio-option">
							<input type="radio" bind:group={orgStrategy} value="ByDate" />
							<span>By Date (YYYY-MM-DD)</span>
						</label>
						
						<label class="radio-option">
							<input type="radio" bind:group={orgStrategy} value="ByYear" />
							<span>By Year (YYYY)</span>
						</label>
						
						<label class="radio-option">
							<input type="radio" bind:group={orgStrategy} value="ByYearMonth" />
							<span>By Year-Month (YYYY-MM)</span>
						</label>
						
						<label class="radio-option">
							<input type="radio" bind:group={orgStrategy} value="ByFileType" />
							<span>By File Type</span>
						</label>
						
						<label class="radio-option">
							<input type="radio" bind:group={orgStrategy} value="ByDateAndType" />
							<span>By Date + Type</span>
						</label>
					</div>
				</div>
				
				<div class="setting-group">
					<div class="setting-label">Operation Mode:</div>
					
					<div class="radio-group">
						<label class="radio-option">
							<input type="radio" bind:group={orgMode} value="Copy" />
							<span>Copy (keeps originals)</span>
						</label>
						
						<label class="radio-option">
							<input type="radio" bind:group={orgMode} value="Move" />
							<span>Move (removes originals)</span>
						</label>
					</div>
				</div>
				
				{#if !hasFiles}
					<div class="info-text">
						Load files first to organize them.
					</div>
				{/if}
			</div>
		{/if}
	</div>
	
	<!-- REVIEW SECTION -->
	<div class="tool-section">
		<div 
			class="section-header"
			class:expanded={expandedSections.review}
			on:click={() => toggleSection('review')}
		>
			<h3 class="section-title">Review</h3>
			<span class="section-icon" class:expanded={expandedSections.review}>▶</span>
		</div>
		
		{#if expandedSections.review}
			<div class="section-content">
				<button 
					class="tool-button" 
					disabled={!hasFiles}
					on:click={() => goto('/review')}
				>
					📋 View Audit Trail
				</button>
				
				<div class="info-text">
					Audit trail and operation history. All changes are logged and reversible.
				</div>
				
				{#if !hasFiles}
					<div class="info-text">
						No operations yet.
					</div>
				{/if}
			</div>
		{/if}
	</div>
	
	<!-- EXPORT SECTION -->
	<div class="tool-section">
		<div 
			class="section-header"
			class:expanded={expandedSections.export}
			on:click={() => toggleSection('export')}
		>
			<h3 class="section-title">Export</h3>
			<span class="section-icon" class:expanded={expandedSections.export}>▶</span>
		</div>
		
		{#if expandedSections.export}
			<div class="section-content">
				<button 
					class="tool-button" 
					disabled={!hasFiles}
				>
					📤 Export Report
				</button>
				
				<button 
					class="tool-button" 
					disabled={!hasFiles}
				>
					📊 Generate Statistics
				</button>
				
				{#if !hasFiles}
					<div class="info-text">
						Load files first to export data.
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>
