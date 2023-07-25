<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();
	import { page } from '$app/stores';
	import { userStore } from '$lib/stores/user.store';
	import moment from 'moment';
	import Button from './Button.svelte';
	import { modalStore } from '$lib/stores/modal.store';
	import LoginModal from './LoginModal.svelte';
	import { swipe } from 'svelte-gestures';
	import { fly } from 'svelte/transition';
	let direction;

	function handler(event: any) {
		direction = event.detail.direction;
		if (direction === 'left') dispatch('close');
	}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore missing-declaration -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	class="wrapper"
	on:click={() => dispatch('close')}
	use:swipe={{ timeframe: 300, minSwipeDistance: 100, touchAction: 'pan-left' }}
	on:swipe={handler}>
	<div class="container" on:click|stopPropagation transition:fly={{ x: -500, duration: 350 }}>
		<nav>
			<a href="/" class:active={$page.url.pathname === '/'}>Last Updated</a>
			<a href="/submit" class:active={$page.url.pathname === '/submit'}>Submit</a>
			<a href="/about" class:active={$page.url.pathname === '/about'}>About</a>
		</nav>
		<div class="current_info">
			<span> {moment().format('dddd, MMMM Do YYYY')} </span>
			{#if $userStore}
				| <b>{$userStore?.username}</b>
				<div>
					<Button text="Logout" action={() => userStore.logout()} />
				</div>
			{:else}
				<div>
					<Button
						text="Login"
						action={() =>
							modalStore.add({
								title: 'Login',
								component: LoginModal,
								props: {},
								size: 'sm'
							})} />
				</div>
			{/if}
		</div>
	</div>
</div>

<style lang="scss">
	* {
		box-sizing: border-box;
		font-family: var(--font-text);
	}

	.wrapper {
		position: fixed;
		height: 100dvh;
		left: 0;
		width: 100%;
		background-color: #000000aa;
	}

	.container {
		height: 100%;
		width: clamp(300px, 90%, 500px);
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		gap: 16px;
		padding: 1rem;
		background-color: var(--background);
	}
	nav {
		display: flex;
		gap: 16px;
		flex-direction: column;
		a {
			text-decoration: none;
			color: black;
			&.active {
				font-weight: bold;
			}
			&:hover {
				filter: brightness(1.2);
				color: var(--accent) !important;
			}
		}
	}
</style>
