<script lang="ts">
	import type { Board } from '$lib/models/board.model';
	import BoardComponent from '$lib/components/Board.svelte';
	import { onMount } from 'svelte';
	import EmptyState from './EmptyState.svelte';
	export let elements: Board[];
	let width: number;
	let scaleFactor: number = 1;
	let masonryWidth: number = 500;
	let mobile = false;

	const handleWidth = () => {
		if (width) {
			masonryWidth = width * 0.9;
			if (masonryWidth < 550) {
				scaleFactor = masonryWidth / 550;
				mobile = true;
			} else {
				mobile = false;
			}
			if (masonryWidth % 50 !== 0) {
				masonryWidth = Math.floor(masonryWidth / 50) * 50;
			}
		}
	};

	onMount(() => {
		handleWidth();
	});

	$: {
		width = width;
		handleWidth();
	}
</script>

<svelte:window bind:innerWidth={width} />

{#if elements.length === 0}
	<EmptyState />
{:else if mobile}
	<div class="flex">
		{#each elements as element, i}
			<div
				class="masonry-item"
				id={`board-${i}`}
				style={`transform: scale(${scaleFactor}); 
				transform-origin: top;
				${
					element.orientation === 'Landscape'
						? `width: ${550 * scaleFactor}px; height: ${350 * scaleFactor}px;`
						: `width: ${350 * scaleFactor}px; height: ${550 * scaleFactor}px;`
				};`}>
				<BoardComponent board={element} />
			</div>
		{/each}
	</div>
{:else}
	<div
		class="masonry"
		style={`width:${masonryWidth}px !important; min-width: ${masonryWidth}px !important; max-width: ${masonryWidth}px !important;`}>
		{#each elements as element, i}
			<div
				class="masonry-item"
				id={`board-${i}`}
				style={`${element.orientation === 'Landscape' ? 'grid-column: span 10; grid-row: span 7' : 'grid-column: span 7; grid-row: span 10'};  ${
					scaleFactor !== 1 ? `transform: scale(${scaleFactor}); transform-origin: top;` : ''
				} `}>
				<BoardComponent board={element} />
			</div>
		{/each}
	</div>
{/if}

<style lang="scss">
	.masonry {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(50px, 1rem));
		grid-template-rows: repeat(auto-fill, minmax(50px, 1rem));
		width: 100%;
		position: relative;
		column-gap: 1px;
		row-gap: 5px;
		place-content: center;
		grid-auto-flow: dense;
	}

	.flex {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
		align-items: center;
		flex-direction: column;
		gap: 1rem;
	}

	.masonry-item {
		transition: all 0.2s ease;
		display: flex;
		justify-content: center;
		align-content: center;
	}
</style>
