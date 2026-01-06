<script lang="ts">
	import { themeStore } from '$lib/stores/themeStore';
	import { icons } from '$lib/ui/icons';
	
	let currentTheme: 'light' | 'dark' = 'light';
	
	themeStore.subscribe(value => {
		currentTheme = value;
	});
	
	function toggleTheme() {
		themeStore.toggle();
	}
</script>

<style>
	.theme-toggle {
		width: 100%;
		display: flex;
		align-items: center;
		gap: var(--space-3);
		padding: var(--space-3);
		border: none;
		background: none;
		color: var(--text-muted);
		font-size: var(--text-sm);
		font-weight: var(--weight-medium);
		cursor: pointer;
		border-radius: var(--radius-md);
		transition: all var(--transition-fast);
		text-align: left;
	}
	
	.theme-toggle:hover {
		background-color: var(--bg-subtle);
		color: var(--text);
	}
	
	.theme-icon {
		font-size: 18px;
		line-height: 1;
	}
	
	.theme-label {
		flex: 1;
	}
</style>

<button type="button" class="theme-toggle" on:click={toggleTheme}>
	<span class="theme-icon">{currentTheme === 'dark' ? icons.sun : icons.moon}</span>
	<span class="theme-label">{currentTheme === 'dark' ? 'Light Mode' : 'Dark Mode'}</span>
</button>
