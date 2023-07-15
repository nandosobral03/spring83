<script lang="ts">
	import { browser } from '$app/environment';
	import type { Board } from '$lib/models/board.model';
	import DOMPurify from 'dompurify';
	export let clickable = true;
	export let board: Board;
	let shadowHost: HTMLElement;
	let shadowRoot: ShadowRoot;
	$: {
		if (browser && shadowHost) {
			if (!shadowRoot) shadowRoot = shadowHost.attachShadow({ mode: 'open' });
			const clean = DOMPurify.sanitize(board.body, { FORBID_TAGS: ['img', 'video'] });
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
            ${clean}`; // Use template literals (backticks) to insert variables
		}
	}

	const details = () => {
		window.open(`/board/${board.key}`, '_blank');
	};
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	style={`border: 0px solid black !important; overflow: hidden !important; display: flex; box-sizing: border-box !important; margin: 0.25rem !important;
			position: relative !important;
	
	${
		board.orientation === 'Landscape'
			? 'max-width:500px !important; min-width: 500px !important; max-height: 350px !important; min-height: 350px !important'
			: 'max-height: 500px !important ; min-height: 500px !important; max-width: 350px !important; min-width: 350px !important'
	}`}
	class={board.orientation === 'Landscape' ? 'masonry-item wide' : 'masonry-item tall'}
	on:click={() => clickable && details()}
	on:keydown={(e) => clickable && e.key === 'Enter' && details()}>
	<article bind:this={shadowHost} style="min-width: 100%; min-height: 100%;" />
</div>

<style lang="scss">
	* {
		box-sizing: border-box;
	}
</style>
