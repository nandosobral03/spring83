<script lang="ts">
	import Button from '$lib/components/Button.svelte';
	import * as ed from '@noble/ed25519';
	import moment from 'moment';

	let startGenerating: string | undefined = undefined;
	let done: string | undefined = undefined;
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
	const KEY_REGEX = /83e(0[1-9]|1[0-2])23$/;

	const findKeypair = async () => {
		startGenerating = moment().format('MMMM Do YYYY, h:mm:ss a');
		let secretBytes = new Uint8Array();
		let publicBytes = new Uint8Array();
		let secretKey: String = '';
		let publicKey: String = '';

		let t = performance.now();
		let iterations = 0;

		while (!publicKey?.match(KEY_REGEX)) {
			secretBytes = ed.utils.randomPrivateKey();
			publicBytes = await ed.getPublicKeyAsync(secretBytes);
			publicKey = ed.etc.bytesToHex(publicBytes);
			iterations++;
			if (iterations % 10000 == 0) {
				console.log(`${iterations} iterations!`);
			}
		}

		secretKey = ed.etc.bytesToHex(secretBytes);
		done = moment().format('MMMM Do YYYY, h:mm:ss a');
		saveKeypairFile(secretKey, publicKey);
		console.log(`Generated valid keypair in ${(performance.now() - t) / 1000.0}s`);
		const coin = new Audio('/bell.mp3');
		coin.play();
	};

	let saveKeypairFile = function (secretKey: String, publicKey: String) {
		const keyString = `${secretKey}${publicKey}`;
		const memo = publicKey.slice(0, 12);
		const timestamp = new Date().toISOString().slice(0, 10);
		const downloader = document.createElement('a');
		const file = new Blob([keyString], { type: 'text/plain;charset=utf-8' });
		downloader.href = URL.createObjectURL(file);
		downloader.download = `spring-83-keypair-${timestamp}-${memo}.txt`;
		downloader.click();
	};
</script>

<div>
	<p>
		Generating keypairs is a computationally intensive process 10-30 minutes on average. This page will generate a keypair and save it to a file. Once
		you click the button, it will take a few minutes to generate a keypair You can leave the tab open and come back to it later, you'll be notified
		when it's done!
	</p>
	<p>
		Want a faster way to get a keypair? Check out other generators you can download and run locally for faster results(About 5 minutes on average)
	</p>
	<ul>
		<li><a href="https://github.com/nandosobral03/spring83-keygen"> Download the generator from github </a></li>
	</ul>
	<div class="button">
		<Button action={findKeypair} text="Generate" />
	</div>
	{#if startGenerating}
		<p>Started generating at {startGenerating}</p>
		{#if done}
			<p>Done at {done}</p>
		{/if}
	{/if}
</div>

<style lang="scss">
	div {
		width: 80%;
		display: flex;
		flex-direction: column;
	}
	a {
		color: var(--text);
		text-decoration: underline;
		&:hover {
			filter: brightness(1.2);
		}
	}
	.button {
		margin-top: 25px;
		width: 300px;
		height: 50px;
		align-self: center;
	}
</style>
