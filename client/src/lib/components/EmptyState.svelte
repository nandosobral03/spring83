<script lang="ts">
</script>

<div>
	<img src="empty.png" alt="No Boards" />
	<p>No boards to display</p>
</div>

<style lang="scss">
	div {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		width: 100%;
		height: 100%;
		user-select: none;
	}

	img {
		width: clamp(300px, 50%, 500px);
	}
	p {
		text-align: center;
		font-family: var(--font-text);
		font-size: 1.5rem;
	}
</style>
