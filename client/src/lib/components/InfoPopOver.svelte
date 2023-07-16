<script lang="ts">
</script>

<div class="pop-over">
	<h1>What's allowed?</h1>
	<p>
		A board can contain be any valid HTML document. But the client will not execute any scripts or load any external resources, this includes images,
		stylesheets, fonts, etc. It will however still present anchor tags as links.
	</p>
	<h1>How do I style my board?</h1>
	<p>
		You can use inline styles or the style tag to style your board. You can also use the 'style' attribute on any element to style it. You can use the <code>
			:host
		</code> selector to style the board itself. For compatibility reasons, the style tag should be the last element in the board.
	</p>
	<h1>What's the maximum size of a board?</h1>
	<p>
		The maximum size of a board is 2217 bytes as stated on the <a href="https://github.com/robinsloan/spring-83/blob/main/draft-20220629.md">
			protocol specification</a
		>. You may notice the size starts at 46 bytes. This is because the client adds the current timestamp to the board before signing it. To be safe,
		we recommend keeping the size of your board under 2200 bytes.
	</p>
</div>

<style lang="scss">
	.pop-over {
		position: absolute;
		background-color: var(--background);
		opacity: 100%;
		width: 95%;
		height: 95%;
		z-index: 1;
		padding: 1rem;
		box-sizing: border-box;
		border-radius: 3px;
	}
	h1 {
		font-family: var(--font-header);
	}
	p {
		font-family: var(--font-text);
	}
</style>
