<script lang="ts">
	import { browser } from '$app/environment';
	import DOMPurify from 'dompurify';
	import type { PageServerData } from './$types';
	import moment from 'moment';
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
		<div><b>Orientation:</b> {data.orientation == 'landscape' ? 'Landscape' : 'Portrait'}</div>
		<div><b>Size:</b> {data.size} bytes</div>
		<div><b>Last Modified:</b> {moment(data.lastModified).format('MMMM Do YYYY, h:mm:ss a')}</div>
		<div><b>Signature:</b> {data.signature}</div>
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
</style>
