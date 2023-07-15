<script lang="ts">
	import { browser } from '$app/environment';
	import DOMPurify from 'dompurify';
	import type { PageServerData } from './$types';
	import moment from 'moment';
	import Button from '$lib/components/Button.svelte';
	import { userStore } from '$lib/stores/user.store';
	import { modalStore } from '$lib/stores/modal.store';
	import LoginModal from '$lib/components/LoginModal.svelte';
	import { loadingStore } from '$lib/stores/loading.store';
	import axios from 'axios';
	import { PUBLIC_API_URL } from '$env/static/public';
	import { toastStore } from '$lib/stores/toast.store';
	export let data: PageServerData;
	let shadowHost: HTMLElement;
	let shadowRoot: ShadowRoot;
	$: {
		if (browser && shadowHost) {
			if (!shadowRoot) shadowRoot = shadowHost.attachShadow({ mode: 'open' });
			const clean = DOMPurify.sanitize(data.body, { FORBID_TAGS: ['img', 'video'] });
			DOMPurify.addHook('afterSanitizeAttributes', function (node) {
				if (node.nodeName.toLowerCase() === 'a') {
					node.setAttribute('target', '_blank');
					node.setAttribute('rel', 'noopener noreferrer');
				}
			});

			shadowRoot.innerHTML = `
            <style>
                :host {
                    position: relative;
                    background-color: #d9d9d9;
                    box-sizing: border-box;
                    padding: 2rem;
                }
                time { display: none; }
                p, h1, h2, h3, h4, h5 { margin: 0 0 2rem 0; }
            </style>
            ${clean}`;
		}
	}

	const login = () => {
		modalStore.add({
			title: 'Login',
			component: LoginModal,
			props: {}
		});
	};

	const follow = async () => {
		try {
			await axios.put(
				`${PUBLIC_API_URL}/boards/following/${data.id}`,
				{},
				{
					headers: {
						Authorization: `${$userStore?.token}`
					}
				}
			);
			toastStore.addToast({ type: 'success', title: 'Board followed', text: `You are now following ${data.id}` });
			data.following = true;
		} catch (e) {
			toastStore.addToast({ type: 'error', title: 'Error', text: `Error following ${data.id} try again later` });
		} finally {
		}
	};

	const unfollow = async () => {
		try {
			await axios.delete(`${PUBLIC_API_URL}/boards/following/${data.id}`, {
				headers: {
					Authorization: `${$userStore?.token}`
				}
			});
			toastStore.addToast({ type: 'success', title: 'Board followed', text: `You are now following ${data.id}` });
			data.following = false;
		} catch (e) {
			toastStore.addToast({ type: 'error', title: 'Error', text: `Error following ${data.id} try again later` });
		} finally {
		}
	};
</script>

<section style={`${data.orientation == 'landscape' ? 'flex-direction: column' : 'flex-direction: row'}`}>
	<div
		style={`border: 0px solid black !important; overflow: hidden !important; display: flex; box-sizing: border-box !important; margin: 0.25rem !important;
			position: relative !important;
            
            ${
							data.orientation?.toLowerCase() === 'landscape'
								? 'max-width:500px !important; min-width: 500px !important; max-height: 350px !important; min-height: 350px !important'
								: 'max-height: 500px !important ; min-height: 500px !important; max-width: 350px !important; min-width: 350px !important'
						}`}>
		<article bind:this={shadowHost} style="min-width: 100%; min-height: 100%;" />
	</div>
	<div
		class="info"
		style={` ${
			data.orientation?.toLowerCase() === 'landscape'
				? 'max-width:500px !important; min-width: 500px !important; max-height: 350px !important; min-height: 350px !important'
				: 'max-height: 500px !important ; min-height: 500px !important; max-width: 350px !important; min-width: 350px !important'
		}`}>
		<div><b>Key:</b> {data.id}</div>
		<div><b>Orientation:</b> {data.orientation == 'landscape' ? 'Landscape' : 'Portrait'}</div>
		<div><b>Size:</b> {data.size} bytes</div>
		<div><b>Last Modified:</b> {moment(data.lastModified).format('MMMM Do YYYY, h:mm:ss a')}</div>
		<div><b>Signature:</b> {data.signature}</div>
		<div class="buttons">
			{#if $userStore}
				{#if data.following}
					<Button text="Unfollow" action={unfollow} />
				{:else}
					<Button text="Follow" action={follow} />
				{/if}
			{:else}
				<Button text="Follow" action={login} />
			{/if}
		</div>
	</div>
</section>

<style lang="scss">
	section {
		box-sizing: border-box;
		height: 100%;
		width: 100%;
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 20px;
	}

	.info {
		font-size: 1rem;
		box-sizing: border-box;
		padding: 2rem;
		display: flex;
		gap: 1rem;
		flex-direction: column;
		border: 1px solid var(--text);
		div {
			max-width: 500px;
			word-break: break-all;
		}
	}

	.buttons {
		margin-top: auto;
		display: flex;
		gap: 1rem;
	}
</style>
