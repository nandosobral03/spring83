<script lang="ts">
	import { modalStore } from '$lib/stores/modal.store';
	import axios from 'axios';
	import Button from './Button.svelte';
	import { PUBLIC_API_URL } from '$env/static/public';
	import { toastStore } from '$lib/stores/toast.store';

	let form = {
		username: {
			value: '',
			touched: false
		},
		password: {
			value: '',
			touched: false
		},

		privateKey: {
			value: '',
			touched: false
		}
	};

	const assignKeys = async () => {
		form.username.touched = true;
		form.password.touched = true;
		form.privateKey.touched = true;

		try {
			await axios.put(`${PUBLIC_API_URL}/auth/keys`, {
				email: form.username.value,
				password: form.password.value,
				private_key: form.privateKey.value
			});

			toastStore.addToast({
				title: 'Success',
				text: 'Keys linked successfully',
				type: 'success'
			});
			modalStore.pop();
		} catch (e: any) {
			toastStore.addToast({
				title: 'Error',
				text: e?.response?.data || 'Something went wrong',
				type: 'error'
			});
		}
	};
</script>

<section>
	<div class="input-group">
		<input type="text" placeholder="Username" bind:value={form.username.value} />
		<input type="password" placeholder="Password" bind:value={form.password.value} />
		<input type="password" placeholder="Private key" bind:value={form.privateKey.value} />
	</div>
	<div class="button-group">
		<Button action={() => modalStore.pop()} text="Cancel" type="secondary" />
		<Button action={assignKeys} text="Link keypair" />
	</div>
</section>

<style lang="scss">
	section {
		height: 100%;
		padding: 0rem 1rem;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		.input-group {
			display: flex;
			flex-direction: column;
			gap: 1rem;
			input {
				padding: 1rem;
				border: none;
				outline: none;
				border-radius: 3px;
				font-size: 1rem;
				background-color: transparent;
				border: 1px solid black;
			}
		}
		.button-group {
			display: flex;
			gap: 1rem;
			height: 10%;
		}
		margin: {
			top: 1rem;
			bottom: 1rem;
		}
	}
</style>
