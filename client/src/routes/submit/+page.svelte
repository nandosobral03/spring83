<script lang="ts">
	import BoardComponent from '$lib/components/Board.svelte';
	import Button from '$lib/components/Button.svelte';
	import CreateBoard from '$lib/components/CreateBoard.svelte';
	import InfoPopOver from '$lib/components/InfoPopOver.svelte';
	import SignBoard from '$lib/components/SignBoard.svelte';
	import type { Board } from '$lib/models/board.model';
	import { modalStore } from '$lib/stores/modal.store';

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

	let toggleInfo = false;
</script>

<div class="container">
	<div class="preview_container">
		<CreateBoard
			bind:board
			on:toggle_info={() => {
				toggleInfo = !toggleInfo;
			}} />
		<section class="preview">
			{#if toggleInfo}
				<InfoPopOver />
			{/if}
			<BoardComponent bind:board />
		</section>
	</div>
	<Button action={() => signBoard()} text="Sign and Publish" style="flex-grow: 1;" />
</div>

<style lang="scss">
	* {
		box-sizing: border-box;
	}
	.container {
		width: 100%;
		min-height: 90%;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		gap: 1rem;
	}
	.preview_container {
		width: 100%;
		height: 90%;
		display: flex;
		justify-content: center;
		gap: 10px;
	}

	section {
		width: 50%;
		border: 1px solid black;
	}

	.preview {
		display: flex;
		justify-content: center;
		align-items: center;
		background-color: var(--text);
		aspect-ratio: 1/1;
		border-radius: 3px;
		position: relative;
	}
</style>
