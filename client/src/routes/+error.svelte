<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { PUBLIC_API_URL } from '$env/static/public';
	import BoardComponent from '$lib/components/Board.svelte';
	import type { PageServerData } from './$types';
	import moment from 'moment';
	import type { Board } from '$lib/models/board.model';
	export let data: PageServerData;

	let board: Board;
	let loading = true;
	onMount(async () => {
		const res = await fetch(`${PUBLIC_API_URL}/ab589f4dde9fce4180fcf42c7b05185b0a02a5d682e353fa39177995083e0583`);
		board = {
			body: await res.text(),
			timestamp: moment().unix().toString(),
			last_modified: moment().unix().toString(),
			signature: await res.headers.get('spring-signature')!,
			orientation: 'Portrait',
			key: 'ab589f4dde9fce4180fcf42c7b05185b0a02a5d682e353fa39177995083e0583'
		};
		loading = false;
	});
</script>

<div>
	<h1>Uh oh!</h1>
	<h2>Looks like you've found a page that doesn't exist.</h2>
	<h3>Click <a href="/">here</a> to go back to the home page.</h3>

	<p>Enjoy this random board while you are here :)</p>
	{#if loading}
		<p>Loading...</p>
	{:else}
		<BoardComponent {board} />
	{/if}
</div>

<style lang="scss">
	* {
		font-family: var(--font-text);
	}

	div {
		display: flex;
		flex-direction: column;
		padding: 1rem;
		gap: 1rem;
	}
</style>
