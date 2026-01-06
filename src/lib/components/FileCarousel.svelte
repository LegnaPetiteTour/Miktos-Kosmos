<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { goto } from '$app/navigation';
	import PreviewCard from './PreviewCard.svelte';
	
	export let files: any[] = [];
	
	let scrollerRef: HTMLDivElement;
	let currentIndex = 0;
	
	function scrollTo(index: number) {
		if (index < 0 || index >= files.length) return;
		currentIndex = index;
		
		const items = scrollerRef?.querySelectorAll('[data-carousel-item]');
		const targetItem = items?.[index] as HTMLElement;
		
		if (targetItem && scrollerRef) {
			targetItem.scrollIntoView({ behavior: 'smooth', inline: 'center', block: 'nearest' });
		}
	}
	
	// Calculate visible dots (show max 5 dots)
	$: visibleDots = (() => {
		if (files.length <= 5) return files.map((_, i) => i);
		
		const maxDots = 5;
		const half = Math.floor(maxDots / 2);
		
		if (currentIndex <= half) {
			return Array.from({ length: maxDots }, (_, i) => i);
		}
		
		if (currentIndex >= files.length - half - 1) {
			return Array.from({ length: maxDots }, (_, i) => files.length - maxDots + i);
		}
		
		return Array.from({ length: maxDots }, (_, i) => currentIndex - half + i);
	})();
	
	function handlePrev() {
		scrollTo(Math.max(0, currentIndex - 1));
	}
	
	function handleNext() {
		scrollTo(Math.min(files.length - 1, currentIndex + 1));
	}
	
	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'ArrowRight') {
			e.preventDefault();
			handleNext();
		} else if (e.key === 'ArrowLeft') {
			e.preventDefault();
			handlePrev();
		} else if (e.key === 'Home') {
			e.preventDefault();
			scrollTo(0);
		} else if (e.key === 'End') {
			e.preventDefault();
			scrollTo(files.length - 1);
		}
	}
	
	function handleFileClick(file: any) {
		// For now, just navigate to workspace
		// Later: open preview modal
		goto('/workspace');
	}
	
	onMount(() => {
		window.addEventListener('keydown', handleKeydown);
	});
	
	onDestroy(() => {
		window.removeEventListener('keydown', handleKeydown);
	});
</script>

<style>
	.carousel {
		position: relative;
		padding: var(--space-6) 0;
		overflow: hidden;
	}
	
	.carousel-frame {
		position: relative;
		padding: 0 80px;
	}
	
	.carousel-scroller {
		display: flex;
		gap: var(--space-6);
		overflow-x: auto;
		scroll-snap-type: x mandatory;
		scrollbar-width: none;
		-ms-overflow-style: none;
		padding: var(--space-4) 0;
		justify-content: center;
	}
	
	.carousel-scroller::-webkit-scrollbar {
		display: none;
	}
	
	.carousel-item {
		scroll-snap-align: center;
	}
	
	.carousel-nav {
		position: absolute;
		top: 50%;
		transform: translateY(-50%);
		width: 48px;
		height: 48px;
		background: var(--bg);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		font-size: var(--text-2xl);
		color: var(--text);
		z-index: 10;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1),
		            inset 0 1px 1px rgba(255, 255, 255, 0.1);
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
	}
	
	.carousel-nav::before {
		content: '';
		position: absolute;
		inset: 0;
		background: linear-gradient(
			135deg,
			rgba(255, 255, 255, 0.1) 0%,
			rgba(255, 255, 255, 0.05) 100%
		);
		border-radius: 50%;
		opacity: 0;
		transition: opacity 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}
	
	.carousel-nav:hover {
		transform: translateY(-50%) scale(1.1);
		box-shadow: 0 6px 20px rgba(0, 0, 0, 0.15),
		            inset 0 1px 1px rgba(255, 255, 255, 0.2);
		border-color: rgba(255, 255, 255, 0.2);
	}
	
	.carousel-nav:hover::before {
		opacity: 1;
	}
	
	.carousel-nav:active {
		transform: translateY(-50%) scale(0.95);
	}
	
	.carousel-nav.left {
		left: var(--space-4);
	}
	
	.carousel-nav.right {
		right: var(--space-4);
	}
	
	.carousel-nav:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}
	
	.carousel-nav:disabled:hover {
		transform: translateY(-50%) scale(1);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
	}
	
	.carousel-dots {
		display: flex;
		justify-content: center;
		gap: var(--space-2);
		margin-top: var(--space-6);
	}
	
	.dot {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: var(--border);
		cursor: pointer;
		transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
	}
	
	.dot.active {
		background: var(--accent);
		width: 24px;
		border-radius: 4px;
	}
	
	.dot:hover:not(.active) {
		background: var(--text-subtle);
		transform: scale(1.2);
	}
	
	.carousel-empty {
		padding: var(--space-8);
		text-align: center;
		color: var(--text-subtle);
	}
</style>

{#if files.length > 0}
	<div class="carousel">
		<div class="carousel-frame">
			<button
				type="button"
				class="carousel-nav left"
				disabled={currentIndex === 0}
				on:click={handlePrev}
				aria-label="Previous"
			>
				←
			</button>
			
			<div class="carousel-scroller" bind:this={scrollerRef}>
				{#each files.slice(0, 10) as file, i (file.path)}
					<div class="carousel-item" data-carousel-item>
						<PreviewCard {file} onClick={() => handleFileClick(file)} />
					</div>
				{/each}
			</div>
			
			<button
				type="button"
				class="carousel-nav right"
				disabled={currentIndex === Math.min(files.length, 10) - 1}
				on:click={handleNext}
				aria-label="Next"
			>
				→
			</button>
		</div>
		
		{#if files.length > 1}
			<div class="carousel-dots">
				{#each visibleDots as dotIndex (dotIndex)}
					<button
						type="button"
						class="dot"
						class:active={dotIndex === currentIndex}
						on:click={() => scrollTo(dotIndex)}
						aria-label="Go to item {dotIndex + 1}"
					/>
				{/each}
			</div>
		{/if}
	</div>
{/if}
