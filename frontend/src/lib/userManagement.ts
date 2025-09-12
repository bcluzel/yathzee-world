export interface UserRegistrationData {
	username: string;
	email: string;
	password: string;
}

export interface UserLoginData {
	email: string;
	password: string;
}

export async function registerUser(data: UserRegistrationData) {
	const response = await fetch('/api/identity/register', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(data)
	});
	if (!response.ok) {
		throw new Error(`User login failed: ${await response.text()}`);
	}
}

export async function loginUser(data: UserLoginData) {
	const response = await fetch('/api/identity/login', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(data)
	});
	if (!response.ok) {
		throw new Error(`User login failed: ${await response.text()}`);
	}
}
