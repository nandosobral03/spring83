<script lang="ts">
	import type { Board } from '$lib/models/board.model';
	import Prism from 'prismjs';
	import { createEventDispatcher } from 'svelte';
	export let board: Board;
	import moment from 'moment';

	let timestamp = `<time datetime="${moment().format('YYYY-MM-DDTHH:mm:ss')}Z"></time>`;
	let textArea: HTMLTextAreaElement;
	const dispatch = createEventDispatcher();
	function update(text: string) {
		let result_element = document.querySelector('#highlighting-content')!;
		// Handle final newlines (see article)
		if (text[text.length - 1] == '\n') {
			text += ' ';
		}
		// Update code
		result_element.innerHTML = text.replace(new RegExp('&', 'g'), '&amp;').replace(new RegExp('<', 'g'), '&lt;'); /* Global RegExp */
		// Syntax Highlight
		Prism.highlightElement(result_element);
	}

	function sync_scroll(element: HTMLTextAreaElement) {
		/* Scroll result to scroll coords of event - sync with textarea */
		let result_element = document.querySelector('#highlighting')!;
		// Get and set x and y
		result_element.scrollTop = element.scrollTop;
		result_element.scrollLeft = element.scrollLeft;
	}

	function check_tab(element: HTMLTextAreaElement, event: any) {
		let code = element.value;
		if (event.key == 'Tab') {
			/* Tab key pressed */
			event.preventDefault(); // stop normal
			let before_tab = code.slice(0, element.selectionStart); // text before tab
			let after_tab = code.slice(element.selectionEnd, element.value.length); // text after tab
			let cursor_pos = element.selectionStart + 1; // where cursor moves after tab - moving forward by 1 char to after tab
			element.value = before_tab + '\t' + after_tab; // add tab char
			// move cursor
			element.selectionStart = cursor_pos;
			element.selectionEnd = cursor_pos;
			update(element.value); // Update text to include indent
		}
	}
</script>

<section>
	<select
		name="orientation"
		id="orientation"
		bind:value={board.orientation}
		on:change={() => {
			update(textArea.value);
			sync_scroll(textArea);
		}}>
		<option value="Portrait">Portrait</option>
		<option value="Landscape">Landscape</option>
	</select>
	<div class="text-editor">
		<textarea
			placeholder="Board's html goes here..."
			id="editing"
			spellcheck="false"
			on:input={() => {
				update(textArea.value);
				sync_scroll(textArea);
			}}
			on:scroll={() => sync_scroll(textArea)}
			on:keydown={(e) => check_tab(textArea, e)}
			bind:this={textArea}
			bind:value={board.body} />
		<pre id="highlighting" aria-hidden="true">
			<code class="language-html" id="highlighting-content" />
		</pre>
		<button class="info-button" on:click={() => dispatch('toggle_info')}>
			<span class="material-symbols-outlined"> info_i </span>
		</button>
		<span class="length"> {new Blob([board.body]).size + new Blob([timestamp]).size + 1} bytes </span>
	</div>
</section>

<style lang="scss">
	* {
		box-sizing: border-box;
		::-webkit-scrollbar {
			display: none;
		}
	}
	select {
		width: 100%;

		padding: 1rem;
		gap: 1rem;
		background-color: var(--text);
		color: var(--background);
		option {
			background-color: var(--text);
			color: var(--background);
		}
	}

	section {
		min-height: 70vh;
		width: 100%;
		border: 1px solid black;
		display: flex;
		flex-direction: column;
		padding: 1rem;
		gap: 1rem;
		background-color: var(--text);
		border-radius: 3px;
		flex-grow: 1;
	}
	code {
		display: block;
		flex-direction: column;
	}

	.text-editor {
		position: relative;
		flex-grow: 1;
		border: 1px solid var(--background);
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
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

	button {
		position: absolute;
		top: 10px;
		right: 10px;
		background-color: white;
		opacity: 10%;
		border-radius: 50%;
		z-index: 1;
		height: 25px;
		width: 25px;
		display: flex;
		justify-content: center;
		align-items: center;
		span {
			font-size: 1rem;
			font-family: 'Material Symbols Outlined' !important;
			user-select: none;
		}
		&:hover {
			opacity: 60%;
		}
	}

	.length {
		position: absolute;
		bottom: 10px;
		right: 10px;
		color: white;
		z-index: 1;
		font-size: 1rem;
	}
</style>
