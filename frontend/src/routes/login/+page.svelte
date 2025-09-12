<script lang="ts">
	import { loginUser, type UserLoginData } from '$lib/userManagement.svelte';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import { toast } from 'svelte-sonner';

	let email = $state('');
	let password = $state('');

	async function validateAndRegisterUser() {
		console.log('Validating form and registering user');
		let data: UserLoginData = {
			email: email,
			password: password
		};
		try {
			await loginUser(data);
		} catch (err) {
			toast.error(`${err}`);
			return;
		}
		await goto(resolve('/'));
	}
</script>

<form
	class="fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4"
	onsubmit={validateAndRegisterUser}
>
	<legend class="fieldset-legend">Login</legend>

	<label class="label" for="email">Email</label>
	<input
		type="email"
		class="input"
		placeholder="Email"
		autocomplete="username"
		bind:value={email}
	/>

	<label class="label" for="password">Password</label>
	<input
		type="password"
		class="input"
		placeholder="Password"
		autocomplete="current-password"
		bind:value={password}
	/>

	<button type="submit" class="btn btn-neutral mt-4">Login</button>

	<div class="divider"></div>
	No account yet ?
	<a class="btn btn-link btn-outline" href={resolve('/register')}>Register here</a>
</form>
