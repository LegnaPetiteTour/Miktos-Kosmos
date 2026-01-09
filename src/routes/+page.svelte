<script lang="ts">
	import FlexWorkspace from '$lib/layouts/FlexWorkspace.svelte';
</script>

<style>
	.home-page {
		display: flex;
		flex-direction: column;
		height: 100%;
		background-color: var(--bg);
	}
</style>

<div class="home-page">
	<FlexWorkspace />
</div>
