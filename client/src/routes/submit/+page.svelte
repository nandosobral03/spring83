<script lang="ts">
	import { browser } from '$app/environment';
	import CreateBoard from '$lib/components/CreateBoard.svelte';
	import type { Board } from '$lib/models/board.model';
	import DOMPurify from 'dompurify';

	let shadowHost: HTMLElement;
	let shadowRoot: ShadowRoot;
	let board: Board = {
		body: '',
		timestamp: '',
		last_modified: '',
		signature: '',
		orientation: 'Portrait',
		public_key: ''
	};

	$: {
		if (browser && shadowHost) {
			if (!shadowRoot) shadowRoot = shadowHost.attachShadow({ mode: 'open' });
			const clean = DOMPurify.sanitize(board.body, { FORBID_TAGS: ['img', 'video'] });

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
            ${clean}`; // Use template literals (backticks) to insert variables
			console.log(shadowRoot.innerHTML);
		}
	}
</script>

<div>
	<CreateBoard bind:board />

	<section class="preview">
		<article
			bind:this={shadowHost}
			style={`border: 1px solid black; overflow: hidden !important; display: flex; box-sizing: border-box !important; margin: 0.25rem !important;
		
			${
				board.orientation === 'Landscape'
					? 'max-width:500px !important; min-width: 500px !important; max-height: 350px !important; min-height: 350px !important'
					: 'max-height: 500px !important ; min-height: 500px !important; max-width: 350px !important; min-width: 350px !important'
			}
		`}
		/>
	</section>
</div>

<style lang="scss">
	* {
		box-sizing: border-box;
	}
	div {
		width: 90%;
		height: 100%;
		display: flex;
		justify-content: center;
		gap: 10px;
		padding: 1rem;
	}

	section {
		width: 50%;
		height: 100%;
		border: 1px solid black;
	}

	.preview {
		display: flex;
		justify-content: center;
		align-items: center;
	}
</style>

