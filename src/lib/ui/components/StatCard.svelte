<script lang="ts">
	import Card from '../primitives/Card.svelte';
	
	export let label: string;
	export let value: string | number;
	export let meta: string | undefined = undefined;
	export let icon: string | undefined = undefined;
	export let onClick: (() => void) | undefined = undefined;
</script>

<style>
	.statcard-wrapper {
		height: 100%;
	}
	
	.statcard {
		width: 100%;
		height: 100%;
		border: none;
		background: none;
		padding: 0;
		text-align: left;
		cursor: default;
		transition: all var(--transition-base);
	}
	
	.statcard.clickable {
		cursor: pointer;
	}
	
	.statcard.clickable:hover {
		transform: translateY(-2px);
	}
	
	.statcard.clickable:active {
		transform: translateY(0);
	}
	
	.statcard-content {
		display: flex;
		flex-direction: column;
		gap: var(--space-3);
		height: 100%;
	}
	
	.statcard-top {
		display: flex;
		align-items: flex-start;
		gap: var(--space-3);
	}
	
	.statcard-icon {
		font-size: var(--text-2xl);
		line-height: 1;
		opacity: 0.9;
	}
	
	.statcard-label {
		font-size: var(--text-sm);
		color: var(--text-muted);
		font-weight: var(--weight-medium);
		letter-spacing: 0.01em;
	}
	
	.statcard-value {
		font-size: var(--text-3xl);
		font-weight: var(--weight-bold);
		color: var(--text);
		line-height: var(--line-height-tight);
		letter-spacing: -0.02em;
	}
	
	.statcard-meta {
		font-size: var(--text-sm);
		color: var(--text-subtle);
		line-height: var(--line-height-relaxed);
	}
</style>

<div class="statcard-wrapper">
	<Card padding="md">
		<button
			type="button"
			class="statcard"
			class:clickable={onClick}
			on:click={onClick}
			disabled={!onClick}
		>
			<div class="statcard-content">
				<div class="statcard-top">
					{#if icon}
						<div class="statcard-icon">{icon}</div>
					{/if}
					<div class="statcard-label">{label}</div>
				</div>
				<div class="statcard-value">{value}</div>
				{#if meta}
					<div class="statcard-meta">{meta}</div>
				{/if}
			</div>
		</button>
	</Card>
</div>
