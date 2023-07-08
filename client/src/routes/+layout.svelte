<script lang="ts">
	import Header from '$lib/components/Header.svelte';
	import Modal from '$lib/components/Modal.svelte';
	import { boardCountStore } from '$lib/stores/board_count.store';
	import { loadingStore } from '$lib/stores/loading.store';
	import { modalStore } from '$lib/stores/modal.store';
	import { onMount } from 'svelte';
	import type { LayoutServerData } from './$types';
	export let data: LayoutServerData;

	onMount(() => {
		boardCountStore.set(data.board_count);
	});
</script>

<main>
	<Header />
	<slot />
	{#if $modalStore != null}
		<Modal />
	{/if}
	{#if $loadingStore}
		<div class="loading">
			<div class="spinner" />
		</div>
	{/if}
</main>

<style lang="scss">
	main {
		display: flex;
		flex-direction: column;
		align-items: center;
		height: 100%;
		width: 100%;
	}
	* {
		box-sizing: border-box;
	}
</style>
