<script lang="ts">
	import type { StatusBadge } from '$lib/types';
	import Badge from '../primitives/Badge.svelte';
	
	export let workspaceLabel: string | undefined = undefined;
	export let statusBadges: StatusBadge[] = [];
</script>

<style>
	.topbar {
		height: var(--topbar-height);
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0 var(--space-5);
		background-color: var(--panel);
		border-bottom: 1px solid var(--panel-border);
		flex-shrink: 0;
	}
	
	.topbar-left {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		font-size: var(--text-sm);
	}
	
	.topbar-workspace-label {
		color: var(--text-muted);
	}
	
	.topbar-workspace-value {
		color: var(--text);
		font-weight: var(--weight-medium);
	}
	
	.topbar-right {
		display: flex;
		align-items: center;
		gap: var(--space-4);
	}
	
	.topbar-badges {
		display: flex;
		align-items: center;
		gap: var(--space-3);
	}
</style>

<header class="topbar">
	<div class="topbar-left">
		{#if workspaceLabel}
			<span class="topbar-workspace-label">Workspace:</span>
			<span class="topbar-workspace-value">{workspaceLabel}</span>
		{/if}
	</div>
	
	<div class="topbar-right">
		<div class="topbar-badges">
			{#each statusBadges.slice(0, 5) as badge (badge.id)}
				<Badge {...badge} />
			{/each}
		</div>
	</div>
</header>
