<script lang="ts">
	import { themeStore } from '$lib/stores/themeStore';
	
	let currentTheme: 'light' | 'dark' = 'light';
	
	themeStore.subscribe(value => {
		currentTheme = value;
	});
	
	function toggleTheme() {
		themeStore.toggle();
	}
</script>

<style>
	.theme-toggle-wrapper {
		width: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--space-4) var(--space-3);
	}
	
	.toggle-container {
		display: flex;
		align-items: center;
		gap: var(--space-4);
	}
	
	.toggle-label {
		font-size: calc(var(--text-xs) * 1.4);
		font-weight: var(--weight-medium);
		letter-spacing: 0.02em;
		color: var(--text-subtle);
		user-select: none;
	}
	
	.toggle-track {
		position: relative;
		width: 64px;
		height: 32px;
		background: var(--bg-subtle);
		border: 2px solid var(--border);
		border-radius: 999px;
		cursor: pointer;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.1);
		outline: none;
	}
	
	.toggle-track:hover {
		border-color: rgba(0, 0, 0, 0.15);
		outline: none;
	}
	
	.toggle-track:focus {
		outline: none;
	}
	
	.toggle-track:focus-visible {
		outline: 2px solid var(--accent);
		outline-offset: 2px;
	}
	
	.toggle-thumb {
		position: absolute;
		top: 2px;
		left: 2px;
		width: 24px;
		height: 24px;
		background: var(--bg);
		border-radius: 50%;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2),
		            0 1px 2px rgba(0, 0, 0, 0.1);
	}
	
	.toggle-track.dark .toggle-thumb {
		transform: translateX(32px);
	}
	
	.toggle-track:active .toggle-thumb {
		width: 28px;
	}
	
	.toggle-track.dark:active .toggle-thumb {
		transform: translateX(28px);
	}
</style>

<div class="theme-toggle-wrapper">
	<div class="toggle-container">
		<span class="toggle-label">Light</span>
		<button 
			type="button" 
			class="toggle-track"
			class:dark={currentTheme === 'dark'}
			on:click={toggleTheme}
			aria-label="Toggle theme"
		>
			<div class="toggle-thumb"></div>
		</button>
		<span class="toggle-label">Dark</span>
	</div>
</div>
