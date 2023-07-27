<script lang="ts">
	import axios from 'axios';
	import Button from './Button.svelte';
	import { PUBLIC_API_URL } from '$env/static/public';
	import { toastStore } from '$lib/stores/toast.store';
	import { modalStore } from '$lib/stores/modal.store';
	import '$lib/modal.scss';
	let form = {
		username: {
			value: '',
			touched: false
		},
		password: {
			value: '',
			touched: false
		}
	};

	const signUp = async () => {
		form.username.touched = true;
		form.password.touched = true;
		if (!form.username.value || !form.password.value) {
			toastStore.addToast({
				title: 'Error',
				text: 'Please fill in all fields',
				type: 'error'
			});
			return;
		}
		try {
			const response = await axios.post(`${PUBLIC_API_URL}/auth`, {
				email: form.username.value,
				password: form.password.value
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
			on:keydown={(event) => {
				form.username.touched = true;
				if (event.key === 'Enter') login();
			}}
			style={`border-color: ${form.username.touched && !form.username.value ? 'red !important' : ' black !important'}`} />
		<input
			type="password"
			placeholder="Password"
			bind:value={form.password.value}
			on:keydown={(event) => {
				form.password.touched = true;
				if (event.key === 'Enter') login();
			}}
			style={`border-color: ${form.password.touched && !form.password.value ? 'red !important' : ' black !important'}`} />
	</div>
	<div class="button-group">
		<Button action={() => modalStore.pop()} text="Cancel" type="secondary" />
		<Button action={signUp} text="Sign up" />
	</div>
</section>
