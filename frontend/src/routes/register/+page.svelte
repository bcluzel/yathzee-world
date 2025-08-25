<fieldset class="fieldset bg-base-200 border-base-300 rounded-box w-xs border p-4">
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
	/>
	<p class="validator-hint hidden">
		Must be 3 to 30 characters
		<br />containing only letters, numbers or dash
	</p>
	<label class="label" for="email">Email</label>
	<input class="input validator" type="email" />
	<div class="validator-hint hidden">Enter valid email address</div>
	<label class="label" for="password">Password</label>
	<input
		type="password"
		class="input validator"
		required
		minlength="8"
		pattern={'(?=.*\\d)(?=.*[a-z])(?=.*[A-Z]).{8,}'}
		title="Must be more than 8 characters, including number, lowercase letter, uppercase letter"
	/>
	<p class="validator-hint hidden">
		Must be more than 8 characters, including
		<br />At least one number
		<br />At least one lowercase letter
		<br />At least one uppercase letter
	</p>
	<label class="label" for="password-validation">Validate Password</label>
	<input type="password" id="password-validation" class="input validator" required />
	<p class="validator-hint hidden">Passwords do not match</p>
	<button class="btn btn-neutral mt-4">Register</button>
</fieldset>
