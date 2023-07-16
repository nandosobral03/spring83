<script lang="ts">
	import { modalStore } from '$lib/stores/modal.store';
	import axios from 'axios';
	import Button from './Button.svelte';
	import { PUBLIC_API_URL } from '$env/static/public';
	import { toastStore } from '$lib/stores/toast.store';
	import '$lib/modal.scss';
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
