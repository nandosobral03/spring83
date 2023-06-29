<script lang="ts">
	import type { Board } from '$lib/models/board.model';
	import DOMPurify from 'dompurify';
	import { onMount } from 'svelte';
	export let board: Board;
	let shadowHost: HTMLElement;

	onMount(() => {
		const shadowRoot = shadowHost.attachShadow({ mode: 'open' });
		const clean = DOMPurify.sanitize(board.body, {FORBID_TAGS: ['img', 'video'], ALLOWED_TAGS: ["time", "style"]});

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
	});

	
</script>

<article
	bind:this={shadowHost}
	style={`border: 1px solid black; overflow: hidden !important; display: flex; box-sizing: border-box !important; margin: 0.25rem !important;
	
		${board.orientation === 'Landscape'
		? 'max-width:500px !important; min-width: 500px !important; max-height: 350px !important; min-height: 350px !important'
		: 'max-height: 500px !important ; min-height: 500px !important; max-width: 350px !important; min-width: 350px !important'}
	`
	}
    class={board.orientation === 'Landscape' ? 'masonry-item wide' : 'masonry-item tall'}
/>

<style lang="scss">
	* {
		box-sizing: border-box;
	}



</style>
