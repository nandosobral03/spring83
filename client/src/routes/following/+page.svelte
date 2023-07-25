<script lang="ts">
	import type { PageServerData } from './$types';
	import MasonryGrid from '$lib/components/MasonryGrid.svelte';
	export let data: PageServerData;
</script>

<div>
	<MasonryGrid elements={data.boards} />
</div>

<style lang="scss">
	* {
		box-sizing: border-box;
	}
	div {
		width: 90%;
		height: fit-content;
		display: flex;
		justify-content: center;
		align-items: center;
		padding: 30px 0px;
	}
</style>
