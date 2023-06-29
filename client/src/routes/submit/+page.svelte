<script lang="ts">
	import { browser } from '$app/environment';
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
        if(browser && shadowHost){
            if(!shadowRoot) shadowRoot = shadowHost.attachShadow({ mode: 'open' });
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
        }
    }

</script>

<div>
	<section class="form">
		<select bind:value={board.orientation}>
			<option>Portrait</option>
			<option>Landscape</option>
		</select>
		<textarea bind:value={board.body} on:input={() => board = board} />
	</section>

	<section class="preview">
		<article
			bind:this={shadowHost}
			style={board.orientation === 'Landscape'
				? 'max-width:500px; min-width: 500px; max-height: 350px; min-height: 350px'
				: 'max-height: 500px ; min-height: 500px; max-width: 350px; min-width: 350px'}
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

    article {
		border: 1px solid black;
		overflow: hidden;
        display: flex;
		margin: 0.25rem;
    }
</style>
