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
	.container {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		align-items: center;
		padding: 1rem;
		gap: 1rem;
	}
	.preview_container {
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
	}
</style>
