<script lang="ts">
	import type { Board } from '$lib/models/board.model';
	import DOMPurify from 'dompurify';
	import { onMount } from 'svelte';
	export let board: Board;
	let shadowHost: HTMLElement;

	onMount(() => {
		const shadowRoot = shadowHost.attachShadow({ mode: 'open' });
		const clean = DOMPurify.sanitize(board.body, {FORBID_TAGS: ['img', 'video']});

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
	style={board.orientation === 'Landscape'
		? 'max-width:500px; min-width: 500px; max-height: 350px; min-height: 350px'
		: 'max-height: 500px ; min-height: 500px; max-width: 350px; min-width: 350px'}
    class={board.orientation === 'Landscape' ? 'masonry-item wide' : 'masonry-item tall'}
/>

<style lang="scss">
	* {
		box-sizing: border-box;
	}

	article {
		border: 1px solid black;
		overflow: hidden;
        display: flex;
		margin: 0.25rem;
    }

</style>
