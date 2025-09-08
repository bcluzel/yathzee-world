<script lang="ts">
	import '$lib/userManagement';

	let username = $state('');
	let email = $state('');
	let password = $state('');
	let passwordValidation = $state('');
	let passwordsEquals = $derived(password == passwordValidation);

	async function validateAndRegisterUser() {
		console.log('Validating form and registering user');
		if (password !== passwordValidation) {
			alert('Passwords do not match');
			return;
		}
		let data: UserRegistrationData = {
			username: username,
			email: email,
			password: password
		};
		try {
			registerUser(data);
		} catch (err) {
			alert('Unexpected error from backend!');
			console.error(err);
		}
	}
</script>

<form
	class="fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4"
	onsubmit={validateAndRegisterUser}
>
	<legend class="fieldset-legend">Register</legend>
	<label class="label" for="username"> Username</label>
	<input
		type="text"
		class="input validator"
		required
		pattern="[A-Za-z][A-Za-z0-9\-]*"
		minlength="3"
		maxlength="30"
		title="Only letters, numbers or dash"
		bind:value={username}
	/>
	<p class="validator-hint hidden">
		Must be 3 to 30 characters
		<br />containing only letters, numbers or dash
	</p>
	<label class="label" for="email">Email</label>
	<input class="input validator" type="email" bind:value={email} />
	<div class="validator-hint hidden">Enter valid email address</div>
	<label class="label" for="password">Password</label>
	<input
		type="password"
		class="input validator"
		required
		minlength="8"
		pattern={'(?=.*\\d)(?=.*[a-z])(?=.*[A-Z]).{8,}'}
		title="Must be more than 8 characters, including number, lowercase letter, uppercase letter"
		bind:value={password}
	/>
	<p class="validator-hint hidden">
		Must be more than 8 characters, including
		<br />At least one number
		<br />At least one lowercase letter
		<br />At least one uppercase letter
	</p>
	<label class="label" for="password-validation">Validate Password</label>
	<input
		type="password"
		id="password-validation"
		class="input"
		required
		bind:value={passwordValidation}
	/>
	{#if !passwordsEquals}
		<p class="validator-hint">Passwords do not match</p>
	{/if}
	<button type="submit" class="btn btn-neutral mt-4">Register</button>
</form>
