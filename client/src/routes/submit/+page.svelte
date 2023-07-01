<script lang="ts">
	import BoardComponent from '$lib/components/Board.svelte';
	import CreateBoard from '$lib/components/CreateBoard.svelte';
	import type { Board } from '$lib/models/board.model';
	import nacl from 'tweetnacl';
	import moment from 'moment';
	import { PUBLIC_API_URL } from '$env/static/public';
	import axios from 'axios';

	let board: Board = {
		body: '',
		timestamp: '',
		last_modified: '',
		signature: '',
		orientation: 'Portrait',
		public_key: ''
	};

	const decodeHexStringToByteArray = (hexString: string) => {
		var result = [];
		while (hexString.length >= 2) {
			result.push(parseInt(hexString.substring(0, 2), 16));
			hexString = hexString.substring(2, hexString.length);
		}
		return result;
	};

	function toHexString(byteArray: Uint8Array) {
		return Array.from(byteArray, function (byte) {
			return ('0' + (byte & 0xff).toString(16)).slice(-2);
		}).join('');
	}

	const signBoard = async (board: Board) => {
		const privateKeyString = '';
		const publicKeyString = '';
		const hexBytes = decodeHexStringToByteArray(privateKeyString + publicKeyString);
		const keypairBytes = new Uint8Array(hexBytes);
		let keypair = nacl.sign.keyPair.fromSecretKey(keypairBytes);
		const timestamp = `<time datetime="${moment().format('YYYY-MM-DDTHH:mm:ss')}Z"></time>`; 
		const newBody = `${timestamp}${board.body}`;
		const secretMessage = new TextEncoder().encode(newBody);
		const signature = nacl.sign.detached(secretMessage, keypair.secretKey);
		console.log(signature);
		const hexSignature = toHexString(signature);
		console.log(hexSignature.length);
		board.signature = hexSignature;

		const response = await axios.put(`${PUBLIC_API_URL}/${publicKeyString}`, newBody, {
			headers: {
				'Content-Type': 'text/html',
				'Spring-Signature': hexSignature
			},
			params: {
				orientation: board.orientation.toLowerCase()
			}
		});
		console.log(response);
	};
</script>

<div class="container">
	<div class="preview_container">
		<CreateBoard bind:board />
		<section class="preview">
			<BoardComponent bind:board />
		</section>
	</div>
	<button on:click={() => signBoard(board)}> Sign and Submit </button>
</div>

<style lang="scss">
	* {
		box-sizing: border-box;
	}

	button {
		width: 90%;
		height: 50px;
		border: 1px solid black;
	}

	.container {
		width: 100%;
		height: 100%;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		padding: 1rem;
		gap: 1rem;
	}

	.preview_container {
		width: 90%;
		height: 100%;
		display: flex;
		justify-content: center;
		gap: 10px;
	}

	section {
		width: 50%;
		height: 100%;
		border: 1px solid black;
	}

	.preview {
		display: flex;
		justify-content: center;
		align-items: center;
	}
</style>
