<script lang="ts">
	import { boardCountStore } from '$lib/stores/board_count.store';
	import moment from 'moment';
	import Button from './Button.svelte';
	import { modalStore } from '$lib/stores/modal.store';
	import LoginModal from './LoginModal.svelte';
	import { page } from '$app/stores';
	import { userStore } from '$lib/stores/user.store';
	import { browser } from '$app/environment';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let orientation: 'portrait' | 'landscape';
	let width: number;
	$: {
		if (browser) {
			width = window.innerWidth;
			orientation = browser ? (screen.orientation.type.includes('landscape') ? 'landscape' : 'portrait') : 'landscape';
			if (width < 768) {
				orientation = 'portrait';
			}
		}
	}
</script>

<svelte:window bind:innerWidth={width} />
<header>
	<div class="nav_info">
		{#if orientation === 'landscape'}
			<nav>
				{#if $userStore}
					<a href="/following" class:active={$page.url.pathname === '/following'}>Following</a>
				{/if}
				<a href="/" class:active={$page.url.pathname === '/'}>Last Updated</a>
				<a href="/submit" class:active={$page.url.pathname === '/submit'}>Submit</a>
				<a href="/about" class:active={$page.url.pathname === '/about'}>About</a>
			</nav>

			<div class="current_info">
				<span>
					Currently Hosting: {$boardCountStore}
					{#if $boardCountStore > 0}
						{$boardCountStore > 1 ? 'boards' : 'board'}
					{/if}
				</span>
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
		{:else}
			<div class="button">
				<Button
					isIcon
					text="menu"
					action={() => {
						dispatch('toggleNav');
					}} />
			</div>
			<div class="current_info">
				<span>
					Currently Hosting: {$boardCountStore}
					{#if $boardCountStore > 0}
						{$boardCountStore > 1 ? 'boards' : 'board'}
					{/if}
				</span>
			</div>
		{/if}
	</div>
	<h1>
		<img src="/title.png" alt="Spring 83" />
	</h1>
</header>

<style lang="scss">
	* {
		box-sizing: border-box;
		font-family: var(--font-text);
	}

	header {
		width: 100%;
		padding: 1rem 1rem;
		position: sticky;
		font-size: 0.8rem;
	}
	h1 {
		display: flex;
		justify-content: center;
		border-top: 2px solid black;
		border-bottom: 2px solid black;
		text-align: center;
		padding: 1rem;
		font-size: clamp(72px, 5rem, 10vw);
		margin: 0.2rem;
		font-family: 'Crimson Text', serif;
		text-transform: uppercase;
		font-weight: 600;
		font-style: italic;
		line-height: 85% !important;
		user-select: none;
	}

	img {
		max-width: 80vw;
	}

	.nav_info {
		display: flex;
		justify-content: space-between;
		align-items: center;
		width: 100%;
		gap: 16px;
		nav {
			display: flex;
			gap: 16px;
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
		.current_info {
			gap: 16px;
			display: flex;
			align-items: flex-end;
			align-items: center;
		}
	}
</style>
