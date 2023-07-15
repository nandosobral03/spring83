<script lang="ts">
	import { browser } from '$app/environment';
	import DOMPurify from 'dompurify';
	import type { PageServerData } from './$types';
	import moment from 'moment';
	import Button from '$lib/components/Button.svelte';
	import { userStore } from '$lib/stores/user.store';
	import { modalStore } from '$lib/stores/modal.store';
	import LoginModal from '$lib/components/LoginModal.svelte';
	import axios from 'axios';
	import { PUBLIC_API_URL } from '$env/static/public';
	import { toastStore } from '$lib/stores/toast.store';
	import { onMount } from 'svelte';
	import Prism from 'prismjs';
	export let data: PageServerData;
	let shadowHost: HTMLElement;
	let shadowRoot: ShadowRoot;

	onMount(() => {
		let result_element = document.querySelector('#highlighting-content')!;
		result_element.innerHTML = data.body.replace(new RegExp('&', 'g'), '&amp;').replace(new RegExp('<', 'g'), '&lt;'); /* Global RegExp */
		Prism.highlightElement(result_element);
	});

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

	let showCode = false;
</script>

<section style={`${data.orientation == 'landscape' ? 'flex-direction: column' : 'flex-direction: row'}`}>
	<div
		style={`border: 0px solid black !important; overflow: hidden !important; display: flex; box-sizing: border-box !important; margin: 0.25rem !important;
			position: relative !important;
            display: ${showCode ? 'none' : 'flex'} !important;
            ${
							data.orientation?.toLowerCase() === 'landscape'
								? 'max-width:500px !important; min-width: 500px !important; max-height: 350px !important; min-height: 350px !important'
								: 'max-height: 500px !important ; min-height: 500px !important; max-width: 350px !important; min-width: 350px !important'
						}`}>
		<article bind:this={shadowHost} style="min-width: 100%; min-height: 100%;" />
	</div>
	<div
		class="text-editor"
		style={`border: 0px solid black !important; overflow: hidden !important; display: flex; box-sizing: border-box !important; margin: 0.25rem !important;
		position: relative !important;
		display: ${showCode ? 'flex' : 'none'} !important;
		${
			data.orientation?.toLowerCase() === 'landscape'
				? 'max-width:500px !important; min-width: 500px !important; max-height: 350px !important; min-height: 350px !important'
				: 'max-height: 500px !important ; min-height: 500px !important; max-width: 350px !important; min-width: 350px !important'
		}`}>
		<pre id="highlighting" aria-hidden="true">
			<code class="language-html" id="highlighting-content" />
		</pre>
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
			{#if showCode}
				<Button text="View Board" action={() => (showCode = !showCode)} />
			{:else}
				<Button text="View Code" action={() => (showCode = !showCode)} />
			{/if}
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

	#editing,
	#highlighting {
		/* Both elements need the same text and space styling so they are directly on top of each other */
		margin: 10px;
		padding: 10px;
		border: 0;
		width: calc(100% - 32px);
		height: calc(100% - 32px);
		outline: none;
	}
	#editing,
	#highlighting,
	#highlighting * {
		/* Also add text styles to highlighing tokens */
		font-size: 10pt;
		font-family: monospace;
		line-height: 1.5;
		tab-size: 2;
	}

	#editing,
	#highlighting {
		/* In the same place */
		position: absolute;
		top: 0;
		left: 0;
	}

	/* Move the textarea in front of the result */

	#editing {
		z-index: 1;
	}
	#highlighting {
		z-index: 0;
	}

	/* Make textarea almost completely transparent */

	#editing {
		color: transparent;
		background: transparent;
		caret-color: white; /* Or choose your favourite color */
	}

	/* Can be scrolled */
	#editing,
	#highlighting {
		overflow: auto;
		white-space: nowrap; /* Allows textarea to scroll horizontally */
	}

	/* No resize on textarea */
	#editing {
		resize: none;
	}

	/* Paragraphs; First Image */
	* {
		font-family: 'Fira Code', monospace;
	}

	.text-editor {
		position: relative;
		flex-grow: 1;
		border: 1px solid var(--background);
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		background-color: var(--text);
	}

	code {
		max-width: 100%;
	}
</style>
