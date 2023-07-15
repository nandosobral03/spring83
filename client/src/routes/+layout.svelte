<script lang="ts">
	import Header from '$lib/components/Header.svelte';
	import Modal from '$lib/components/Modal.svelte';
	import { boardCountStore } from '$lib/stores/board_count.store';
	import { loadingStore } from '$lib/stores/loading.store';
	import { modalStore } from '$lib/stores/modal.store';
	import { onMount } from 'svelte';
	import { toastStore } from '$lib/stores/toast.store';
	import type { LayoutServerData } from './$types';
	import ToastNotification from '$lib/components/ToastNotification.svelte';
	import { flip } from 'svelte/animate';
	import { userStore } from '$lib/stores/user.store';
	export let data: LayoutServerData;

	if (data.token) {
		try {
			userStore.set(data.token);
		} catch (err) {
			console.log(err);
		}
	}
	onMount(() => {
		import('@lottiefiles/lottie-player');
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
			<lottie-player
				src="https://assets3.lottiefiles.com/packages/lf20_H1NjXc5LBD.json"
				background="transparent"
				speed="1"
				style="width: 300px; height: 300px;"
				loop
				autoplay />
		</div>
	{/if}
	<div class="toast">
		{#each $toastStore as notification (notification.timestamp)}
			<div animate:flip={{ duration: 300 }}>
				<ToastNotification {...notification} />
			</div>
		{/each}
	</div>
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
	.loading {
		position: fixed;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
		background: rgba(0, 0, 0, 0.5);
		display: flex;
		justify-content: center;
		align-items: center;
		z-index: 10;
	}

	.toast {
		position: fixed;
		bottom: 0;
		right: 0;
		margin: 2rem;
		z-index: 11;
		display: flex;
		flex-direction: column-reverse;
		gap: 1rem;
		align-items: flex-start;
		height: 100vh;
	}
</style>
