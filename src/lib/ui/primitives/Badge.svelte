<script lang="ts">
	import type { Tone } from '$lib/types';
	
	export let tone: Tone = 'neutral';
	export let text: string;
	export let tooltip: string | undefined = undefined;
	export let icon: string | undefined = undefined;
</script>

<style>
	.badge {
		display: inline-flex;
		align-items: center;
		gap: var(--space-1);
		padding: 4px 10px;
		border-radius: 12px;
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
		white-space: nowrap;
	}
	
	.badge-neutral {
		background-color: var(--neutral-bg);
		color: var(--neutral-text);
	}
	
	.badge-info {
		background-color: var(--info-bg);
		color: var(--info-text);
	}
	
	.badge-success {
		background-color: var(--success-bg);
		color: var(--success-text);
	}
	
	.badge-warning {
		background-color: var(--warning-bg);
		color: var(--warning-text);
	}
	
	.badge-danger {
		background-color: var(--danger-bg);
		color: var(--danger-text);
	}
	
	.badge-icon {
		font-size: var(--text-sm);
		line-height: 1;
	}
</style>

<span class="badge badge-{tone}" title={tooltip}>
	{#if icon}
		<span class="badge-icon">{icon}</span>
	{/if}
	<span>{text}</span>
</span>
