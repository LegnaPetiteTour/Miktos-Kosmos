<script lang="ts">
	export let label: string;
	export let description: string | undefined = undefined;
	export let icon: string | undefined = undefined;
	export let variant: 'primary' | 'secondary' | 'ghost' | 'danger' = 'secondary';
	export let disabled: boolean = false;
	export let disabledReason: string | undefined = undefined;
	export let onClick: () => void;
</script>

<style>
	.cmd-btn {
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-5);
		border-radius: var(--radius-lg);
		border: 1.5px solid transparent;
		background: none;
		cursor: pointer;
		transition: all var(--transition-base);
		text-align: left;
		width: 100%;
		box-shadow: var(--shadow-xs);
	}
	
	.cmd-btn:disabled {
		cursor: not-allowed;
		opacity: 0.4;
	}
	
	.cmd-primary {
		background: linear-gradient(135deg, var(--accent) 0%, var(--accent-hover) 100%);
		color: var(--accent-contrast);
		border-color: transparent;
		box-shadow: var(--shadow-md);
	}
	
	.cmd-primary:hover:not(:disabled) {
		box-shadow: var(--shadow-lg);
		transform: translateY(-2px);
	}
	
	.cmd-primary:active:not(:disabled) {
		transform: translateY(0);
	}
	
	.cmd-secondary {
		background-color: var(--panel);
		color: var(--text);
		border-color: var(--panel-border);
	}
	
	.cmd-secondary:hover:not(:disabled) {
		background-color: var(--bg-subtle);
		border-color: var(--border-strong);
		box-shadow: var(--shadow-sm);
		transform: translateY(-1px);
	}
	
	.cmd-ghost {
		background-color: transparent;
		color: var(--text);
		box-shadow: none;
	}
	
	.cmd-ghost:hover:not(:disabled) {
		background-color: var(--bg-subtle);
	}
	
	.cmd-danger {
		background-color: var(--danger-bg);
		color: var(--danger-text);
		border-color: var(--danger);
	}
	
	.cmd-danger:hover:not(:disabled) {
		background-color: var(--danger);
		color: white;
		box-shadow: var(--shadow-md);
	}
	
	.cmd-icon {
		font-size: 24px;
		line-height: 1;
		flex-shrink: 0;
	}
	
	.cmd-text {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: var(--space-1);
	}
	
	.cmd-label {
		font-size: var(--text-md);
		font-weight: var(--weight-semibold);
		line-height: var(--line-height-tight);
	}
	
	.cmd-desc {
		font-size: var(--text-sm);
		opacity: 0.8;
		line-height: var(--line-height-normal);
	}
</style>

<button
	type="button"
	class="cmd-btn cmd-{variant}"
	{disabled}
	title={disabled && disabledReason ? disabledReason : undefined}
	on:click={onClick}
>
	{#if icon}
		<span class="cmd-icon">{icon}</span>
	{/if}
	<span class="cmd-text">
		<span class="cmd-label">{label}</span>
		{#if description}
			<span class="cmd-desc">{description}</span>
		{/if}
	</span>
</button>
