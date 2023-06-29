<script lang="ts">
	import Header from '$lib/components/Header.svelte';
</script>

<main>
	<Header />
		<slot />
</main>

<style lang="scss">
	main {
		display: flex;
		flex-direction: column;
		align-items: center;
		height: 100%;
		width: 100%;
	}
	* {
		box-sizing: border-box;
	}
</style>
