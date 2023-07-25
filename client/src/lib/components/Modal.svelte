<script lang="ts">
	import { currentModalStore, modalStore } from '$lib/stores/modal.store';
</script>

{#if $modalStore.length > 0}
	<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
	<div class="backdrop" on:click={(e) => modalStore.pop()} on:keydown={(e) => e.key === 'Escape' && modalStore.pop()} role="dialog">
		<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
		<div class={`modal ${$currentModalStore?.size}`} on:click|stopPropagation on:keydown|stopPropagation role="dialog">
			<header>
				{$currentModalStore?.title}
			</header>
			<svelte:component this={$currentModalStore?.component} {...$currentModalStore?.props} />
		</div>
	</div>
{/if}

<style lang="scss">
	* {
		box-sizing: border-box;
	}
	.backdrop {
		position: fixed;
		top: 0;
		left: 0;
		z-index: 10;
		width: 100%;
		height: 100%;
		background-color: rgba(0, 0, 0, 0.75);
		display: flex;
		justify-content: center;
		align-items: center;
	}

	.modal {
		position: relative;
		z-index: 100;
		width: 90%;
		height: clamp(16rem, 70%, 40rem);
		max-width: 40rem;
		background-color: var(--background);
		border-radius: 3px;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.26);
		overflow: hidden;
		padding: 1.5rem;
		display: flex;
		flex-direction: column;
		header {
			display: flex;
			width: 100%;
			justify-content: space-between;
			font-size: 1.5rem;
			font-family: var(--font-header);
			text-transform: uppercase;
			font-weight: bold;
			border-bottom: 1px solid black;
			padding-bottom: 1rem;
			button {
				height: 2rem;
				display: grid;
				place-items: center;
				aspect-ratio: 1/1;
				padding: 0;
				border: none;
				cursor: pointer;
				background-color: transparent;
				border-radius: 8px;
				transition: background-color 0.3s ease;
				&:hover {
					background-color: rgba(0, 0, 0, 0.1);
				}
			}
		}
	}

	.sm {
		height: clamp(18rem, 70%, 22rem);
		max-width: 30rem;
	}

	.md {
		height: clamp(20rem, 70%, 30rem);
		max-width: 40rem;
	}

	.lg {
		height: clamp(22rem, 70%, 40rem);
		max-width: 50rem;
	}
</style>
