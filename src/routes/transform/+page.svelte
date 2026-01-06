<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { open } from '@tauri-apps/plugin-dialog';
	import { fileStore } from '$lib/stores/photoStore';
	import { operationsStore } from '$lib/stores/operationsStore';
	import { icons } from '$lib/ui/icons';
	import type { OrganizationPlan, OrganizationStrategy, OperationMode, OperationResult } from '$lib/types';
	import Page from '$lib/ui/layout/Page.svelte';
	import Section from '$lib/ui/layout/Section.svelte';
	import Card from '$lib/ui/primitives/Card.svelte';
	import CommandButton from '$lib/ui/components/CommandButton.svelte';
	
	let scanResult: any = null;
	let organizationPlan: OrganizationPlan | null = null;
	let isGeneratingPlan = false;
	let isExecuting = false;
	let executionResult: OperationResult | null = null;
	let destinationPath: string = '';
	let selectedStrategy: OrganizationStrategy = 'Date';
	let selectedMode: OperationMode = 'Copy';
	
	fileStore.subscribe(value => {
		scanResult = value;
	});
	
	$: hasFiles = scanResult?.files?.length > 0;
	$: totalFiles = scanResult?.files?.length || 0;
	
	async function selectDestination() {
		try {
			const selected = await open({
				directory: true,
				multiple: false,
				title: 'Select destination folder for organized files'
			});
			
			if (selected) {
				destinationPath = selected as string;
			}
		} catch (error) {
			console.error('Error selecting destination:', error);
		}
	}
	
	async function generatePlan() {
		if (!hasFiles || !destinationPath) {
			return;
		}
		
		isGeneratingPlan = true;
		try {
			const plan = await invoke<OrganizationPlan>('create_organization_plan', {
				files: scanResult.files,
				destinationRoot: destinationPath,
				strategy: selectedStrategy,
				mode: selectedMode
			});
			
			organizationPlan = plan;
			console.log('Organization plan generated:', plan);
		} catch (error) {
			console.error('Error generating plan:', error);
			alert(`Failed to generate plan: ${error}`);
		} finally {
			isGeneratingPlan = false;
		}
	}
	
	async function executeOrganization() {
		if (!organizationPlan) return;
		
		isExecuting = true;
		try {
			const result = await invoke<OperationResult>('execute_organization', {
				plan: organizationPlan,
				sourceFiles: scanResult.files
			});
			
			executionResult = result;
			
			// Log the operation for review
			operationsStore.addOperation(result);
			
			console.log('Organization complete:', result);
			
			if (result.success) {
				alert(`Success! Organized ${result.successful_count} files in ${(result.duration_ms / 1000).toFixed(2)}s`);
			} else {
				alert(`Completed with errors. ${result.successful_count} succeeded, ${result.failed_count} failed.`);
			}
		} catch (error) {
			console.error('Error executing organization:', error);
			alert(`Failed to execute: ${error}`);
		} finally {
			isExecuting = false;
		}
	}
	
	function cancelPlan() {
		organizationPlan = null;
		executionResult = null;
	}
	
	function formatBytes(bytes: number): string {
		if (bytes === 0) return '0 B';
		const k = 1024;
		const sizes = ['B', 'KB', 'MB', 'GB'];
		const i = Math.floor(Math.log(bytes) / Math.log(k));
		return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
	}
</script>

<Page title="Transform" subtitle="Organize files into structure">
	{#if !hasFiles}
		<!-- Empty State -->
		<Section title="No Files to Transform">
			<Card padding="lg">
				<div style="text-align: center; padding: var(--space-7) 0;">
					<div style="font-size: 64px; margin-bottom: var(--space-4); opacity: 0.2; color: var(--text-muted);">{icons.transform}</div>
					<h3 style="font-size: var(--text-xl); font-weight: var(--weight-semibold); margin-bottom: var(--space-2); color: var(--text);">
						No workspace loaded
					</h3>
					<p style="font-size: var(--text-base); color: var(--text-muted); margin-bottom: var(--space-5);">
						Scan and analyze files before organizing them.
					</p>
				</div>
			</Card>
		</Section>
	{:else if !organizationPlan && !executionResult}
		<!-- Configuration Step -->
		<Section title="Configuration" description="Choose how to organize your files">
			<div style="display: grid; gap: var(--space-4);">
				<!-- Strategy Selection -->
				<Card>
					<div style="padding: var(--space-4);">
						<h3 style="font-size: var(--text-lg); font-weight: var(--weight-semibold); margin-bottom: var(--space-3); color: var(--text);">
							Organization Strategy
						</h3>
						<div style="display: flex; flex-direction: column; gap: var(--space-2);">
							<label style="display: flex; align-items: center; gap: var(--space-2); cursor: pointer;">
								<input type="radio" bind:group={selectedStrategy} value="ByDate" />
								<span style="font-size: var(--text-base); color: var(--text);">By Date (YYYY/MM-Month)</span>
							</label>
							<label style="display: flex; align-items: center; gap: var(--space-2); cursor: pointer;">
								<input type="radio" bind:group={selectedStrategy} value="ByYear" />
								<span style="font-size: var(--text-base); color: var(--text);">By Year (YYYY)</span>
							</label>
							<label style="display: flex; align-items: center; gap: var(--space-2); cursor: pointer;">
								<input type="radio" bind:group={selectedStrategy} value="ByYearMonth" />
								<span style="font-size: var(--text-base); color: var(--text);">By Year/Month (YYYY/MM)</span>
							</label>
							<label style="display: flex; align-items: center; gap: var(--space-2); cursor: pointer;">
								<input type="radio" bind:group={selectedStrategy} value="ByFileType" />
								<span style="font-size: var(--text-base); color: var(--text);">By File Type</span>
							</label>
						</div>
					</div>
				</Card>
				
				<!-- Mode Selection -->
				<Card>
					<div style="padding: var(--space-4);">
						<h3 style="font-size: var(--text-lg); font-weight: var(--weight-semibold); margin-bottom: var(--space-3); color: var(--text);">
							Operation Mode
						</h3>
						<div style="display: flex; flex-direction: column; gap: var(--space-2);">
							<label style="display: flex; align-items: center; gap: var(--space-2); cursor: pointer;">
								<input type="radio" bind:group={selectedMode} value="Copy" />
								<span style="font-size: var(--text-base); color: var(--text);">Copy (Safe - Recommended)</span>
							</label>
							<label style="display: flex; align-items: center; gap: var(--space-2); cursor: pointer;">
								<input type="radio" bind:group={selectedMode} value="Move" />
								<span style="font-size: var(--text-base); color: var(--text);">Move (Destructive)</span>
							</label>
						</div>
						<p style="font-size: var(--text-sm); color: var(--text-muted); margin-top: var(--space-3);">
							Copy mode keeps your original files safe. You can delete them manually after verification.
						</p>
					</div>
				</Card>
				
				<!-- Destination Selection -->
				<Card>
					<div style="padding: var(--space-4);">
						<h3 style="font-size: var(--text-lg); font-weight: var(--weight-semibold); margin-bottom: var(--space-3); color: var(--text);">
							Destination Folder
						</h3>
						<div style="display: flex; gap: var(--space-3); align-items: center;">
							<input 
								type="text" 
								value={destinationPath} 
								placeholder="Select destination folder..." 
								readonly
								style="flex: 1; padding: var(--space-3); border: 1px solid var(--border); border-radius: 6px; font-size: var(--text-base); background: var(--bg-subtle); color: var(--text);"
							/>
							<CommandButton
								variant="secondary"
								label="Browse"
								icon={icons.folder}
								onClick={selectDestination}
							/>
						</div>
					</div>
				</Card>
				
				<!-- Generate Plan Button -->
				<div style="display: flex; justify-content: flex-end;">
					<CommandButton
						variant="primary"
						label={isGeneratingPlan ? 'Generating Preview...' : 'Preview Organization'}
						description="See how files will be organized"
						disabled={!destinationPath || isGeneratingPlan}
						disabledReason={!destinationPath ? 'Select a destination folder first' : undefined}
						onClick={generatePlan}
					/>
				</div>
			</div>
		</Section>
	{:else if organizationPlan && !executionResult}
		<!-- Preview Step -->
		<Section title="Preview" description="Review the organization plan before executing">
			<Card>
				<div style="padding: var(--space-4);">
					<div style="display: grid; grid-template-columns: repeat(4, 1fr); gap: var(--space-4); margin-bottom: var(--space-4);">
						<div>
							<div style="font-size: var(--text-sm); color: var(--text-muted); margin-bottom: var(--space-1);">Total Files</div>
							<div style="font-size: var(--text-xl); font-weight: var(--weight-bold); color: var(--text);">{organizationPlan.total_files}</div>
						</div>
						<div>
							<div style="font-size: var(--text-sm); color: var(--text-muted); margin-bottom: var(--space-1);">Total Size</div>
							<div style="font-size: var(--text-xl); font-weight: var(--weight-bold); color: var(--text);">{formatBytes(organizationPlan.total_size)}</div>
						</div>
						<div>
							<div style="font-size: var(--text-sm); color: var(--text-muted); margin-bottom: var(--space-1);">Folders</div>
							<div style="font-size: var(--text-xl); font-weight: var(--weight-bold); color: var(--text);">{organizationPlan.folders.length}</div>
						</div>
						<div>
							<div style="font-size: var(--text-sm); color: var(--text-muted); margin-bottom: var(--space-1);">Without Dates</div>
							<div style="font-size: var(--text-xl); font-weight: var(--weight-bold); color: var(--text);">{organizationPlan.files_without_dates}</div>
						</div>
					</div>
					
					<h4 style="font-size: var(--text-md); font-weight: var(--weight-semibold); margin-bottom: var(--space-3); color: var(--text);">
						Folder Structure
					</h4>
					<div style="max-height: 400px; overflow-y: auto; font-family: var(--font-mono); font-size: var(--text-sm); color: var(--text-muted); line-height: 1.8;">
						{#each organizationPlan.folders as folder}
							<div style="margin-bottom: var(--space-2);">
								<div style="color: var(--text); font-weight: var(--weight-medium);">{folder.path.replace(organizationPlan.destination_root + '/', '')}/</div>
								<div style="padding-left: var(--space-4); color: var(--text-muted);">
									{folder.file_count} files ({formatBytes(folder.total_size)})
								</div>
							</div>
						{/each}
					</div>
				</div>
			</Card>
			
			<!-- Action Buttons -->
			<div style="display: flex; gap: var(--space-3); justify-content: flex-end; margin-top: var(--space-4);">
				<CommandButton
					variant="ghost"
					label="Cancel"
					onClick={cancelPlan}
				/>
				<CommandButton
					variant="primary"
					label={isExecuting ? 'Organizing Files...' : `${organizationPlan.mode} ${organizationPlan.total_files} Files`}
					description={isExecuting ? 'Please wait...' : 'Execute the organization plan'}
					disabled={isExecuting}
					onClick={executeOrganization}
				/>
			</div>
		</Section>
	{:else if executionResult}
		<!-- Results Step -->
		<Section title="Results" description="Organization completed">
			<Card variant={executionResult.success ? 'info' : 'danger'}>
				<div style="padding: var(--space-4);">
					<h3 style="font-size: var(--text-xl); font-weight: var(--weight-bold); margin-bottom: var(--space-3); color: var(--text);">
						{executionResult.success ? '✓ Success!' : '⚠ Completed with Errors'}
					</h3>
					<div style="display: grid; grid-template-columns: repeat(4, 1fr); gap: var(--space-4); margin-bottom: var(--space-4);">
						<div>
							<div style="font-size: var(--text-sm); color: var(--text-muted);">Successful</div>
							<div style="font-size: var(--text-xl); font-weight: var(--weight-bold); color: var(--success);">{executionResult.successful_count}</div>
						</div>
						<div>
							<div style="font-size: var(--text-sm); color: var(--text-muted);">Failed</div>
							<div style="font-size: var(--text-xl); font-weight: var(--weight-bold); color: var(--danger);">{executionResult.failed_count}</div>
						</div>
						<div>
							<div style="font-size: var(--text-sm); color: var(--text-muted);">Skipped</div>
							<div style="font-size: var(--text-xl); font-weight: var(--weight-bold); color: var(--text-muted);">{executionResult.skipped_count}</div>
						</div>
						<div>
							<div style="font-size: var(--text-sm); color: var(--text-muted);">Duration</div>
							<div style="font-size: var(--text-xl); font-weight: var(--weight-bold); color: var(--text);">{(executionResult.duration_ms / 1000).toFixed(2)}s</div>
						</div>
					</div>
					
					{#if executionResult.failed_count > 0}
						<h4 style="font-size: var(--text-md); font-weight: var(--weight-semibold); margin-bottom: var(--space-2); color: var(--text);">
							Failed Operations
						</h4>
						<div style="max-height: 300px; overflow-y: auto; font-size: var(--text-sm); color: var(--text-muted);">
							{#each executionResult.operations.filter(op => op.status === 'Failed') as operation}
								<div style="margin-bottom: var(--space-2); padding: var(--space-2); background: var(--danger-bg); border-radius: 6px;">
									<div style="font-weight: var(--weight-medium); color: var(--danger-text);">{operation.source_path}</div>
									<div style="color: var(--danger-text);">{operation.error_message || 'Unknown error'}</div>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			</Card>
			
			<div style="display: flex; justify-content: flex-end; margin-top: var(--space-4);">
				<CommandButton
					variant="secondary"
					label="Organize More Files"
					onClick={cancelPlan}
				/>
			</div>
		</Section>
	{/if}
</Page>
