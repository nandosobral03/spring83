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
		if (!form.username.value || !form.password.value) return;
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
			on:keydown={() => (form.username.touched = true)}
			style={`border: ${form.username.touched && !form.username.value ? '1px solid red' : '1px solid black'}`} />
		<input
			type="password"
			placeholder="Password"
			bind:value={form.password.value}
			on:keydown={() => (form.password.touched = true)}
			style={`border: ${form.password.touched && !form.password.value ? '1px solid red' : '1px solid black'}`} />
	</div>
	<div class="button-group">
		<Button action={() => modalStore.pop()} text="Cancel" type="secondary" />
		<Button action={signUp} text="Sign up" />
	</div>
</section>
