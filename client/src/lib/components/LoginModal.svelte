<script lang="ts">
	import { modalStore } from '$lib/stores/modal.store';
	import axios from 'axios';
	import Button from './Button.svelte';
	import RegisterModal from './RegisterModal.svelte';
	import { PUBLIC_API_URL } from '$env/static/public';
	import { toastStore } from '$lib/stores/toast.store';
	import { userStore } from '$lib/stores/user.store';
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

	const login = async () => {
		try {
			const response = await axios.post(`${PUBLIC_API_URL}/auth/login`, {
				email: form.username.value,
				password: form.password.value
			});
			toastStore.addToast({
				title: 'Success',
				text: 'Logged in successfully',
				type: 'success'
			});
			userStore.set(response.data);
			modalStore.pop();
		} catch (e: any) {
			toastStore.addToast({
				title: 'Error',
				text: e?.response?.data || 'Something went wrong',
				type: 'error'
			});
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
		<Button action={register} text="Don't have an account? Register" type="secondary" />
		<Button action={login} text="Login" />
	</div>
</section>
