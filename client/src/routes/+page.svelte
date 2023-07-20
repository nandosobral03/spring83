<script lang="ts">
	import type { PageServerData } from './$types';
	import Board from '$lib/components/Board.svelte';

	import { onMount } from 'svelte';
	import MasonryGrid from '$lib/components/MasonryGrid.svelte';
	export let data: PageServerData;
	let grid: HTMLElement;

	onMount(async () => {
		// const Masonry = (await import('masonry-layout')).default;
		// let msn = new Masonry(grid, {
		// 	itemSelector: '.masonry-item',
		// 	columnWidth: 5,
		// 	stagger: 30,
		// 	transitionDuration: '0.2s',
		// 	initLayout: true
		// });
		console.log(data.recent_boards);
	});
</script>

<div>
	<!-- <main bind:this={grid} class="masonry">
		{#each data.recent_boards as board}
			<Board {board} />
		{/each}
	</main> -->
	<MasonryGrid elements={data.recent_boards} />
</div>

<style lang="scss">
	* {
		box-sizing: border-box;
	}
	div {
		width: 90%;
		height: fit-content;
		display: flex;
		justify-content: center;
		align-items: center;
	}

	main {
		width: 100%;
		height: 100%;
	}
</style>
