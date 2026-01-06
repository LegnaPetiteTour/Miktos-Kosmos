<script lang="ts">
	export let variant: 'default' | 'subtle' | 'info' | 'danger' = 'default';
	export let padding: 'sm' | 'md' | 'lg' = 'md';
</script>

<style>
	.card {
		background-color: var(--panel);
		border: 1px solid var(--panel-border);
		border-radius: var(--radius-lg);
		transition: all var(--transition-base);
		box-shadow: var(--shadow-sm);
	}
	
	.card:hover {
		box-shadow: var(--shadow-md);
	}
	
	.card-default {
		/* Default styling */
	}
	
	.card-subtle {
		background-color: var(--bg-subtle);
		border-color: var(--border);
		box-shadow: var(--shadow-xs);
	}
	
	.card-info {
		border-color: var(--info);
		background-color: var(--info-bg);
	}
	
	.card-danger {
		border-color: var(--danger);
		background-color: var(--danger-bg);
	}
	
	.card-pad-sm {
		padding: var(--space-4);
	}
	
	.card-pad-md {
		padding: var(--space-5);
	}
	
	.card-pad-lg {
		padding: var(--space-6);
	}
</style>

<div class="card card-{variant} card-pad-{padding}">
	<slot />
</div>
