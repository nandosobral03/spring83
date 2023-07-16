<script lang="ts">
	import { boardCountStore } from '$lib/stores/board_count.store';
	import moment from 'moment';
	import Button from './Button.svelte';
	import { modalStore } from '$lib/stores/modal.store';
	import LoginModal from './LoginModal.svelte';
	import { page } from '$app/stores';
	import { userStore } from '$lib/stores/user.store';
</script>

<header>
	<div class="nav_info">
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
	</div>
	<h1>
		Spring <span> '83 </span>
	</h1>
</header>

<style lang="scss">
	* {
		box-sizing: border-box;
		font-family: var(--font-header);
	}

	header {
		width: 100%;
		padding: 1rem 2rem;
		position: sticky;
	}
	h1 {
		user-select: none;
		border-top: 2px solid black;
		border-bottom: 2px solid black;
		text-align: center;
		padding: 1rem;
		font-size: 6rem;
		margin: 0.2rem;
		font-family: 'Crimson Text', serif;
		text-transform: uppercase;
		line-height: 85% !important;
		span {
			font-family: var(--font-header);
		}
	}

	.nav_info {
		display: flex;
		justify-content: space-between;
		align-items: flex-end;
		width: 100%;
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
