<script lang="ts">
	import type { Board } from '$lib/models/board.model';
	import BoardComponent from '$lib/components/Board.svelte';
	export let elements: Board[] & { columnStart: number; columnEnd: number; rowStart: number; rowEnd: number }[];
	let width: number;
	let masonryWidth: number = 900;
	$: {
		if (width) {
			masonryWidth = width * 0.9;
			if (masonryWidth % 50 !== 0) {
				masonryWidth = Math.floor(masonryWidth / 50) * 50;
			}
			console.log(masonryWidth);
		}
	}
</script>

<svelte:window bind:innerWidth={width} />

<div class="masonry" style={`${masonryWidth}px !important; min-width: ${masonryWidth}px !important; max-width: ${masonryWidth}px !important;`}>
	{#each elements as element, i}
		<!-- Take either 7 or 10 columns depending on screen width -->
		<div
			class="masonry-item"
			id={`board-${i}`}
			style={`${element.orientation === 'Landscape' ? 'grid-column: span 10; grid-row: span 7' : 'grid-column: span 7; grid-row: span 10'}`}>
			<BoardComponent board={element} />
		</div>
	{/each}
</div>

<style lang="scss">
	.masonry {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(50px, 1rem));
		grid-template-rows: repeat(auto-fill, minmax(50px, 1rem));
		width: 100%;
		position: relative;
	}

	.masonry-item {
		transition: all 0.2s ease;
		display: flex;
		justify-content: center;
		align-content: center;
	}
</style>
