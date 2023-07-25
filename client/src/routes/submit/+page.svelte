<script lang="ts">
	import { browser } from '$app/environment';
	import BoardComponent from '$lib/components/Board.svelte';
	import Button from '$lib/components/Button.svelte';
	import CreateBoard from '$lib/components/CreateBoard.svelte';
	import InfoPopOver from '$lib/components/InfoPopOver.svelte';
	import SignBoard from '$lib/components/SignBoard.svelte';
	import type { Board } from '$lib/models/board.model';
	import { modalStore } from '$lib/stores/modal.store';
	import { onMount } from 'svelte';
	import { scale } from 'svelte/transition';

	let board: Board = {
		body: '',
		timestamp: '',
		last_modified: '',
		signature: '',
		orientation: 'Portrait',
		key: ''
	};

	const signBoard = async () => {
		modalStore.add({
			title: 'Sign & Publish',
			component: SignBoard,
			props: { board },
			size: 'md'
		});
	};

	let width: number;
	let scaleFactor: number = 1;

	const handleWidth = () => {
		if (width) {
			console.log(width);
			if (width < 650) {
				scaleFactor = width / 650;
			} else {
				scaleFactor = 1;
			}
		}
	};

	$: {
		width = width;
		handleWidth();
	}

	onMount(() => {
		handleWidth();
		if (browser) {
			console.log(scaleFactor);
		}
	});

	let toggleInfo = false;
</script>

<svelte:window bind:outerWidth={width} />

<div class="container">
	<div class="preview_container">
		<CreateBoard
			bind:board
			on:toggle_info={() => {
				toggleInfo = !toggleInfo;
			}} />
		<section
			class="preview"
			style={`transform: scale(${scaleFactor});
		transform-origin: center; width: ${500 * scaleFactor}px; `}>
			{#if toggleInfo}
				<InfoPopOver />
			{/if}
			<BoardComponent bind:board clickable={false} />
		</section>
	</div>
	<div class="button">
		<Button action={() => signBoard()} text="Sign and Publish" style="flex-grow: 1;" />
	</div>
</div>

<style lang="scss">
	* {
		box-sizing: border-box;
	}

	@media (max-width: 900px) {
		.preview_container {
			flex-direction: column;
			gap: 1rem;
			justify-content: center;
			align-items: center;
		}
		.preview {
			max-width: unset !important;
			width: 100%;
		}
	}

	@media (max-width: 700px) {
		.preview {
			margin: 10px 0;
		}
	}

	@media (max-width: 600px) {
		.preview {
			margin: -50px 0;
		}
	}

	@media (max-width: 500px) {
		.preview {
			margin: -80px 0;
		}
	}

	@media (max-width: 450px) {
		.preview {
			margin: -100px 0;
		}
	}

	.container {
		width: 100%;
		height: fit-content;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		gap: 1rem;
	}
	.preview_container {
		height: 100%;
		width: 100%;
		flex-grow: 1;
		display: flex;
		justify-content: center;
		gap: 10px;
	}

	.preview {
		min-width: 600px;
		max-width: 600px;
		display: flex;
		justify-content: center;
		align-items: center;
		background-color: var(--text);
		aspect-ratio: 1/1;
		border-radius: 3px;
		position: relative;
	}

	.button {
		width: 100%;
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 1rem;
		height: 3rem;
	}
</style>
