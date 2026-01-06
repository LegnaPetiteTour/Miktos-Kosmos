<script lang="ts">
	export let title: string;
	export let description: string | undefined = undefined;
</script>

<style>
	.section {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}
	
	.section-header {
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}
	
	.section-title {
		font-size: var(--text-xl);
		font-weight: var(--weight-semibold);
		color: var(--text);
	}
	
	.section-description {
		font-size: var(--text-sm);
		color: var(--text-muted);
	}
	
	.section-content {
		display: flex;
		flex-direction: column;
		gap: var(--space-4);
	}
</style>

<div class="section">
	<div class="section-header">
		<h2 class="section-title">{title}</h2>
		{#if description}
			<p class="section-description">{description}</p>
		{/if}
	</div>
	<div class="section-content">
		<slot />
	</div>
</div>
