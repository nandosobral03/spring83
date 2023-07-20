<script lang="ts">
	import { modalStore } from '$lib/stores/modal.store';
	import RegisterModal from './RegisterModal.svelte';
	import Button from './Button.svelte';
	import { PUBLIC_API_URL } from '$env/static/public';
	import moment from 'moment';
	import nacl from 'tweetnacl';
	import type { Board } from '$lib/models/board.model';
	import axios from 'axios';
	import { loadingStore } from '$lib/stores/loading.store';
	import { toastStore } from '$lib/stores/toast.store';
	import { refreshBoardCount } from '$lib/stores/board_count.store';
	import AssignKeys from './AssignKeys.svelte';
	import '$lib/modal.scss';

	export let board: Board;

	let tab: 'keys' | 'login' = 'login';

	const loginForm = {
		username: {
			value: '',
			touched: false
		},
		password: {
			value: '',
			touched: false
		}
	};

	const keysForm = {
		publicKey: {
			value: '',
			touched: false
		},
		privateKey: {
			value: '',
			touched: false
		}
	};

	const publish = async () => {
		try {
			loadingStore.set(true);
			if (tab === 'login') {
				loginForm.username.touched = true;
				loginForm.password.touched = true;
				if (!loginForm.username.value || !loginForm.password.value) {
					toastStore.addToast({
						title: 'Error',
						text: 'Please fill in all fields',
						type: 'error'
					});
					return;
				}

				const response = await axios.post(`${PUBLIC_API_URL}/auth/keys`, {
					email: loginForm.username.value,
					password: loginForm.password.value
				});
				const { private_key, public_key } = response.data;
				await signBoard(board, `${private_key}${public_key}`);
			} else {
				keysForm.publicKey.touched = true;
				keysForm.privateKey.touched = true;
				if (!keysForm.publicKey.value || !keysForm.privateKey.value) {
					toastStore.addToast({
						title: 'Error',
						text: 'Please fill in all fields',
						type: 'error'
					});
					return;
				}

				const fullKey = `${keysForm.privateKey.value}${keysForm.publicKey.value}`;
				await signBoard(board, fullKey);
			}
			modalStore.pop();
			refreshBoardCount();
		} catch (e: any) {
			toastStore.addToast({
				title: 'Error',
				text: e?.response?.data || 'Something went wrong',
				type: 'error'
			});
		} finally {
			loadingStore.set(false);
		}
	};

	const register = () => {
		modalStore.add({
			title: 'Register',
			component: RegisterModal,
			props: {},
			size: 'sm'
		});
	};

	const assignKeys = () => {
		modalStore.add({
			title: 'Associate keypair',
			component: AssignKeys,
			props: {},
			size: 'md'
		});
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

	const signBoard = async (board: Board, fullKey: string) => {
		let publicKeyString = fullKey.substring(64);
		const hexBytes = decodeHexStringToByteArray(fullKey);
		const keypairBytes = new Uint8Array(hexBytes);
		let keypair = nacl.sign.keyPair.fromSecretKey(keypairBytes);
		const timestamp = `<time datetime="${moment().format('YYYY-MM-DDTHH:mm:ss')}Z"></time>`;
		const newBody = board.body.trim().length !== 0 ? `${timestamp}\n${board.body}` : `${timestamp}${board.body}`;
		const secretMessage = new TextEncoder().encode(newBody);
		const signature = nacl.sign.detached(secretMessage, keypair.secretKey);
		const hexSignature = toHexString(signature);
		board.signature = hexSignature;

		await axios.put(`${PUBLIC_API_URL}/${publicKeyString}`, newBody, {
			headers: {
				'Content-Type': 'text/html',
				'Spring-Signature': hexSignature
			},
			params: {
				orientation: board.orientation.toLowerCase()
			}
		});

		toastStore.addToast({
			title: 'Success',
			text: 'Board published successfully',
			type: 'success'
		});
	};
</script>

<div style="margin-top: 1rem;" class="container">
	<nav>
		<div>
			<Button
				action={() => {
					tab = 'login';
					keysForm.publicKey.touched = false;
					keysForm.privateKey.touched = false;
				}}
				text="login"
				isIcon={true} />
		</div>
		<div>
			<Button
				action={() => {
					tab = 'keys';
					loginForm.username.touched = false;
					loginForm.password.touched = false;
				}}
				text="key"
				isIcon={true} />
		</div>
	</nav>
	<section>
		{#if tab === 'login'}
			<div class="input-group">
				<input
					type="text"
					placeholder="Username"
					bind:value={loginForm.username.value}
					style={`border-color: ${loginForm.username.touched && !loginForm.username.value ? 'red !important' : ''}`} />
				<input
					type="password"
					placeholder="Password"
					bind:value={loginForm.password.value}
					style={`border-color: ${loginForm.password.touched && !loginForm.password.value ? 'red !important' : ''}`} />
			</div>
			<div class="button-group">
				<Button action={register} text="Register" type="secondary" />
				<Button action={assignKeys} text="Want us to handle your keys?" type="secondary" />
				<Button action={publish} text="Publish" />
			</div>
		{:else if tab === 'keys'}
			<div class="input-group">
				<input
					type="text"
					placeholder="Public key"
					bind:value={keysForm.publicKey.value}
					style={`border-color: ${keysForm.publicKey.touched && !keysForm.publicKey.value ? 'red !important' : ''}`} />
				<input
					type="password"
					placeholder="Private key"
					bind:value={keysForm.privateKey.value}
					style={`border-color: ${keysForm.privateKey.touched && !keysForm.privateKey.value ? 'red !important' : ''}`} />
			</div>
			<div class="button-group">
				<Button
					action={() => {
						window.open('/generate', '_blank');
					}}
					text="Don't have keys? Generate them!"
					type="secondary" />
				<Button action={publish} text="Publish" />
			</div>
		{/if}
	</section>
</div>

<style lang="scss">
	* {
		box-sizing: border-box;
	}

	.container {
		width: 100%;
		height: 100%;
		display: flex;
		nav {
			padding-right: 1rem;
			display: flex;
			flex-direction: column;
			border-right: 1px solid black;
			width: 20%;
			gap: 1rem;
		}
	}

	section {
		flex-grow: 1;
	}
</style>
