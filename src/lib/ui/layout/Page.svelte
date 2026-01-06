<script lang="ts">
	export let title: string;
	export let subtitle: string | undefined = undefined;
</script>

<style>
	.page {
		max-width: 1400px;
		margin: 0 auto;
		padding: var(--space-6);
	}
	
	.page-header {
		margin-bottom: var(--space-6);
	}
	
	.page-title {
		font-size: var(--text-3xl);
		font-weight: var(--weight-bold);
		color: var(--text);
		margin-bottom: var(--space-2);
	}
	
	.page-subtitle {
		font-size: var(--text-md);
		color: var(--text-muted);
	}
	
	.page-content {
		display: flex;
		flex-direction: column;
		gap: var(--space-6);
	}
</style>

<div class="page">
	<div class="page-header">
		<h1 class="page-title">{title}</h1>
		{#if subtitle}
			<p class="page-subtitle">{subtitle}</p>
		{/if}
	</div>
	
	<div class="page-content">
		<slot />
	</div>
</div>
