<script lang="ts">
	import type { PageServerData } from './$types';
	import { onMount } from 'svelte';
	import MasonryGrid from '$lib/components/MasonryGrid.svelte';
	import type { Board } from '$lib/models/board.model';
	import { PUBLIC_API_URL } from '$env/static/public';
	export let data: PageServerData;
	let grid: HTMLElement;
	let currentOffset = data.offset;
	let keepRequesting = true;
	let debounce = false;
	let boards: Board[] = [];
	async function getBoards() {
		if (!keepRequesting) return Promise.resolve();
		const res = await fetch(`${PUBLIC_API_URL}/boards?offset=${currentOffset}`);
		const json = await res.json();
		if (json.length === 0) {
			keepRequesting = false;
			return Promise.resolve();
		}
		boards.push(...json);
		boards = boards;
		console.log(boards);
		currentOffset += 10;
	}
	onMount(() => {
		boards = data.recent_boards;
		document.addEventListener('scroll', async () => {
			console.log(grid?.getBoundingClientRect().bottom, window.innerHeight);
			if (grid?.getBoundingClientRect().bottom < window.innerHeight + 500 && !debounce) {
				debounce = true;
				await getBoards().then(() => {
					console.log('Debouncing');
					debounce = false;
				});
			}
		});
		console.log(data.recent_boards);
		return () => {
			document.removeEventListener('scroll', () => {});
		};
	});
</script>

<div bind:this={grid}>
	<MasonryGrid elements={boards} />
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
		padding: 30px 0px;
	}
</style>
