<script lang="ts">
	import { flip } from 'svelte/animate';
	import { fly } from 'svelte/transition';

	export let type: 'success' | 'error' | 'warning' | 'info' = 'info';
	export let title: string = 'Title';
	export let text: string = 'Text';
	export let timestamp: number = Date.now();

	let colors = {
		success: '#4caf50',
		error: '#f44336',
		warning: '#ff9800',
		info: '#2196f3'
	};
</script>

<div style={`border-left: 5px solid ${colors[type]}`} transition:fly={{ x: 200, duration: 300 }}>
	<span class="title">{title}</span>
	<span class="text">{text}</span>
	<span class="timestamp">{new Date(timestamp).toLocaleTimeString()}</span>
</div>

<style lang="scss">
	div {
		background-color: var(--background);
		border-top-right-radius: 3px;
		border-bottom-right-radius: 3px;
		padding: 1rem;
		display: flex;
		flex-direction: column;
		width: 300px;
		gap: 0.5rem;
		font-family: 'Montserrat', sans-serif;
		.title {
			font-size: 1rem;
			font-weight: 600;
			text-transform: uppercase;
			text-decoration: underline;
		}
		.text {
			font-size: 1rem;
			font-weight: 400;
		}
		.timestamp {
			display: flex;
			width: 100%;
			justify-content: flex-end;
			font-size: 0.8rem;
			font-weight: 300;
			margin-top: 0.5rem;
		}
	}
</style>
