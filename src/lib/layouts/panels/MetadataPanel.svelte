<script lang="ts">
	// Placeholder for metadata panel
</script>

<style>
	.metadata-panel {
		display: flex;
		flex-direction: column;
		gap: var(--space-2);
		font-size: var(--text-sm);
		color: var(--text-muted);
	}
	
	.metadata-item {
		display: flex;
		justify-content: space-between;
		padding: var(--space-2);
		background-color: var(--bg-subtle);
		border-radius: 6px;
	}
	
	.metadata-label {
		font-weight: var(--weight-medium);
	}
</style>

<div class="metadata-panel">
	<div class="metadata-item">
		<span class="metadata-label">Type:</span>
		<span>—</span>
	</div>
	<div class="metadata-item">
		<span class="metadata-label">Size:</span>
		<span>—</span>
	</div>
	<div class="metadata-item">
		<span class="metadata-label">Date:</span>
		<span>—</span>
	</div>
	<div class="metadata-item">
		<span class="metadata-label">Dimensions:</span>
		<span>—</span>
	</div>
</div>
