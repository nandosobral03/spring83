<script lang="ts">
	import { PUBLIC_API_URL } from '$env/static/public';
</script>

<section>
	<p>
		This is a client for the Spring '83 protocol. The Spring '83 protocol, creted and specified by Robin Solan in their
		<a href="https://www.robinsloan.com/lab/specifying-spring-83/" target="_blank">blog post</a>. The Spring '83 protocol was born out of a desire to
		improve upon existing platforms like Twitter, RSS, and email, which often fall short in providing a satisfactory user experience. The creator of
		the protocol wanted a tool that would allow users to follow people who are interesting to them in a way that's straightforward and consistent,
		regardless of the frequency or medium of their content sharing.
	</p>
	<p>
		This client serves as an interface for a server running this protocol. It allows users to create and follow "boards" - HTML fragments mostly
		comprised of text, limited in size and cannot execute JavaScript or load external resources. This ensures that the content is safe, lightweight,
		and quick to load, while still allowing for a rich and diverse range of expression.
	</p>
	<p>
		The server itself is running on <a href={PUBLIC_API_URL}>{PUBLIC_API_URL}</a> and can be used on it's own, or with this client. The source code
		for the server can be found on
		<a href="https://github.com/nandosobral03/spring83" target="_blank">github</a>.
	</p>
	<p>
		To post a board one only needs to create a keypair, and sign and send both the board and the signature to the server. Boards are cryptographically
		signed by the creator, and are verified by the client. This ensures that the content is authentic and has not been tampered with.
	</p>
</section>

<style lang="scss">
	section {
		width: 80%;
		display: flex;
		justify-content: flex-start;
		flex-direction: column;
		font-size: 1.2rem;
		gap: 0.4rem;
		font-family: var(--font-header) !important;
		a {
			color: var(--accent);
			text-decoration: underline;
			&:hover {
				filter: brightness(1.2);
			}
		}
	}
</style>
