<script lang="ts">
	export let action: () => void;
	export let text: string;
	export let isIcon: boolean = false;
	export let type: 'primary' | 'secondary' | 'danger' | 'success' = 'primary';
	export let style: string = '';
</script>

<button on:click={action} class={type} {style}>
	<span class={isIcon ? 'material-symbols-outlined' : ''}> {text} </span>
</button>

<style lang="scss">
	button {
		width: 100%;
		height: 100%;
		padding: 0.5rem;
		border: none;
		outline: none;
		font-weight: bold;
		display: flex;
		justify-content: center;
		align-items: center;
		border-radius: 3px;
		cursor: pointer;
		background: var(--text);
		color: var(--background);
		&:hover {
			filter: brightness(1.6);
			span {
				filter: brightness(0.6);
			}
		}
	}

	.secondary {
		background: transparent;
		color: var(--text);
		border: 1px solid var(--text);
		&:hover {
			background: var(--text);
			color: var(--background);
		}
	}
</style>
