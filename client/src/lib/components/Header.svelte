<script lang="ts">
	import moment from 'moment';
	export let count: number;
</script>

<header>
	<div class="nav_info">
		<nav>
			<a href="/" class="active"> Last Updated </a>
			<a href="/submit">Submit</a>
			<a href="/about">About</a>
		</nav>
		<div class="current_info">
			<span>
				Currently Hosting: {count}
				{#if count > 0}
					{count > 1 ? 'boards' : 'board'}
				{/if}
			</span>
			<div class="current_time">
				<b> The Internet </b>
				<span> {moment().format('dddd, MMMM Do YYYY')} </span>
			</div>
		</div>
	</div>
	<h1>
		Spring <span> '83 </span>
	</h1>
</header>

<style lang="scss">
	* {
		box-sizing: border-box;
		font-family: 'EB Garamond', serif;
	}

	header {
		width: 100%;
		padding: 1rem 2rem;
		position: sticky;
	}
	h1 {
		user-select: none;
		border-top: 2px solid black;
		border-bottom: 2px solid black;
		text-align: center;
		padding: 1rem;
		font-size: 6rem;
		margin: 0.2rem;
		font-family: 'IM Fell Great Primer SC', serif;
		line-height: 85% !important;
		span {
			font-family: 'EB Garamond', serif;
		}
	}

	.nav_info {
		display: flex;
		justify-content: space-between;
		align-items: flex-end;
		width: 100%;
		nav {
			display: flex;
			gap: 16px;
			a {
				text-decoration: none;
				color: black;
				&.active {
					font-weight: bold;
				}
			}
		}
		.current_info {
			gap: 16px;
			display: flex;
			align-items: flex-end;
			.current_time {
				b {
					font-weight: 600;
				}
				display: flex;
				flex-direction: column;
			}
		}
	}
</style>
