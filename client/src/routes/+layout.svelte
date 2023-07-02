<script lang="ts">
	import Header from '$lib/components/Header.svelte';
	import Modal from '$lib/components/Modal.svelte';
	import {modalStore} from '$lib/stores/modal.store';
	import type { LayoutServerData } from './$types';
	export let data : LayoutServerData
</script>

<main>
	<Header count={data.board_count} />
	<slot />
	{#if $modalStore != null}
		<Modal/>
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
