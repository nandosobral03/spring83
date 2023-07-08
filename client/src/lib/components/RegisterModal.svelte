<script lang="ts">
	import axios from 'axios';
	import Button from './Button.svelte';
	import { PUBLIC_API_URL } from '$env/static/public';
	import { toastStore } from '$lib/stores/toast.store';
	import { modalStore } from '$lib/stores/modal.store';

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

	const signUp = async () => {
		form.username.touched = true;
		form.password.touched = true;
		form.privateKey.touched = true;
		if (!form.username.value || !form.password.value || !form.privateKey.value) return;
		try {
			const response = await axios.post(`${PUBLIC_API_URL}/auth`, {
				email: form.username.value,
				password: form.password.value,
				private_key: form.privateKey.value
			});
			toastStore.addToast({
				title: 'Success',
				text: 'Account created successfully',
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
		<input
			type="text"
			placeholder="Username"
			bind:value={form.username.value}
			on:keydown={() => (form.username.touched = true)}
			style={`border: ${form.username.touched && !form.username.value ? '1px solid red' : '1px solid black'}`} />
		<input
			type="password"
			placeholder="Password"
			bind:value={form.password.value}
			on:keydown={() => (form.password.touched = true)}
			style={`border: ${form.password.touched && !form.password.value ? '1px solid red' : '1px solid black'}`} />
		<input
			type="password"
			placeholder="Private key"
			bind:value={form.privateKey.value}
			on:keydown={() => (form.privateKey.touched = true)}
			style={`border: ${form.privateKey.touched && !form.privateKey.value ? '1px solid red' : '1px solid black'}`} />
	</div>
	<div class="button-group">
		<Button action={signUp} text="Sign up" />
	</div>
</section>

<style lang="scss">
	section {
		padding: 1rem;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		height: 100%;
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
	}
</style>
