export interface UserRegistrationData {
	username: string;
	email: string;
	password: string;
}

export interface UserLoginData {
	email: string;
	password: string;
}

export interface UserInfosData {
	username: string;
	email: string;
}

export interface UserInfosState {
	data: null | UserInfosData;
}

export let userInfos: UserInfosState = $state({ data: null });

export async function registerUser(data: UserRegistrationData) {
	const response = await fetch('/api/identity/register', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(data)
	});
	if (!response.ok) {
		switch (response.status) {
			case 500:
				throw new Error(`Internal server error`);
			default:
				throw new Error(`User registering failed: ${await response.text()}`);
		}
	}
	userInfos.data = await getUserInfos();
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
	userInfos.data = await getUserInfos();
}

export async function logoutUser() {
	const response = await fetch('/api/identity/logout', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		}
	});
	if (!response.ok) {
		throw new Error(`User logout failed: ${await response.text()}`);
	}
	userInfos.data = null;
}

export async function getUserInfos(): Promise<UserInfosData | null> {
	const response = await fetch('/api/identity/info', {
		method: 'GET',
		headers: {
			'Content-Type': 'application/json'
		}
	});
	if (!response.ok) {
		return null;
	}
	return await response.json();
}
