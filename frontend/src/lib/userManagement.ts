interface UserRegistrationData {
	username: string;
	email: string;
	password: string;
}

interface UserLoginData {
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
	console.log(response);
	if (!response.ok) {
		const err = await response.json().catch(() => ({}));
		throw new Error(`User registering request faild with error ${err}`);
	}
}
