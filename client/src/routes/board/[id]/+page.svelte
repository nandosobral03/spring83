<script lang="ts">
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
	import BoardComponent from '$lib/components/Board.svelte';
	export let data: PageServerData;

	let mobile = false;
	let width: number;
	let scaleFactor: number = 1;
	const handleWidth = () => {
		if (width) {
			if (width < 650) {
				scaleFactor = width / 650;
				mobile = true;
			} else {
				scaleFactor = 1;
				mobile = false;
			}
		}
	};

	$: {
		width = width;
		handleWidth();
	}

	onMount(() => {
		let result_element = document.querySelector('#highlighting-content')!;
		result_element.innerHTML = data.body.replace(new RegExp('&', 'g'), '&amp;').replace(new RegExp('<', 'g'), '&lt;'); /* Global RegExp */
		Prism.highlightElement(result_element);
		handleWidth();
	});

	const login = () => {
		modalStore.add({
			title: 'Login',
			component: LoginModal,
			props: {},
			size: 'sm'
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
			toastStore.addToast({ type: 'success', title: 'Board unfollowed', text: `You are no longer following ${data.id}` });
			data.following = false;
		} catch (e) {
			toastStore.addToast({ type: 'error', title: 'Error', text: `Error following ${data.id} try again later` });
		} finally {
		}
	};

	let showCode = false;
</script>

<svelte:window bind:innerWidth={width} />
<section style={`${data.orientation == 'landscape' || mobile ? 'flex-direction: column' : 'flex-direction: row'}`}>
	<div
		style={`transform: scale(${data.orientation == 'landscape' ? scaleFactor : 1}); 
		transform-origin: top left;
	${data.orientation === 'landscape' ? `width: ${500 * scaleFactor}px; height: ${350 * scaleFactor}px;` : `width: ${350}px; height: ${500}px;`};
	display: ${showCode ? 'none' : 'flex'} !important;
	border: 1px solid var(--text);

	`}>
		<BoardComponent
			board={{
				body: data.body,
				orientation: data.orientation?.toLocaleLowerCase() === 'landscape' ? 'Landscape' : 'Portrait',
				timestamp: '',
				last_modified: data.lastModified || '',
				signature: '',
				key: data.id
			}} />
	</div>
	<div
		class="text-editor"
		style={`
			border: 1px solid var(--text);
			${
				data.orientation === 'landscape'
					? `
			width: ${500 * scaleFactor}px; 
			max-width: ${500 * scaleFactor}px;

			height: ${350 * scaleFactor}px;
			max-height: ${350 * scaleFactor}px;`
					: `
			width: ${350}px; 
			max-width: ${350}px;
			height: ${500}px;
			max-height: ${500}px;`
			};
		display: ${showCode ? 'flex' : 'none'} !important;`}>
		<pre id="highlighting" aria-hidden="true">
			<code class="language-html" id="highlighting-content" />
		</pre>
	</div>

	<div
		class="info"
		style={` ${
			mobile
				? ''
				: data.orientation?.toLowerCase() === 'landscape'
				? `max-width:${500}px !important; min-width: ${500}px !important; max-height: ${350}px !important; min-height: ${350}px !important`
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
	@media (max-width: 500px) {
		section {
			flex-direction: column !important;
		}
		.info {
			height: fit-content !important;
			margin: 1rem;
		}
	}

	section {
		box-sizing: border-box;
		height: fit-content;
		width: 100%;
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 20px;
		padding-bottom: 1rem;
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
