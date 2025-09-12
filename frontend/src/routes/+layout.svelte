<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	let { children } = $props();
	import { Toaster } from 'svelte-sonner';
	import { resolve } from '$app/paths';
	import { userInfos, logoutUser } from '$lib/userManagement.svelte';
</script>

<Toaster richColors theme="system" />

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="navbar bg-base-100 shadow-sm">
	<div class="navbar-start">
		<a class="btn btn-ghost btn-circle" href={resolve('/')} aria-label="Home btn">
			<svg
				fill="currentColor"
				class="h-6 w-6"
				stroke="currentColor"
				viewBox="-1.27 0 30.066 30.066"
				id="_01_-_Home_Button"
				data-name="01 - Home Button"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					id="Home_Button"
					data-name="Home Button"
					d="M29.759,9.912a3,3,0,0,0-1.543-2.623L17.457,1.312a3,3,0,0,0-2.914,0L3.784,7.289A3,3,0,0,0,2.241,9.912V28a3,3,0,0,0,3,3h6.5a1,1,0,0,0,1-1V24.912a1,1,0,0,1,1-1h4.51a1,1,0,0,1,1,1V30a1,1,0,0,0,1,1h6.5a3,3,0,0,0,3-3V9.912Zm-2,0V28a1,1,0,0,1-1,1h-5.5V24.912a3,3,0,0,0-3-3h-4.51a3,3,0,0,0-3,3V29h-5.5a1,1,0,0,1-1-1V9.912a1,1,0,0,1,.514-.875L15.514,3.06a1,1,0,0,1,.972,0L27.245,9.037a1,1,0,0,1,.514.875Z"
					transform="translate(-2.241 -0.934)"
					fill-rule="evenodd"
				></path>
			</svg>
		</a>
	</div>
	<div class="navbar-center">
		<a class="btn btn-ghost text-xl" href={resolve('/')}>Yathzee World</a>
	</div>
	<div class="navbar-end">
		<!-- Notification button -->
		<!-- <button class="btn btn-ghost btn-circle">
			<div class="indicator">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					class="h-5 w-5"
					fill="none"
					viewBox="0 0 24 24"
					stroke="currentColor"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"
					/>
				</svg>
				<span class="badge badge-xs badge-primary indicator-item"></span>
			</div>
		</button> -->
		{#if userInfos.data}
			<details class="dropdown">
				<summary class="btn m-1">{userInfos.data.username}</summary>
				<ul class="menu dropdown-content bg-base-100 rounded-box z-1 w-52 p-2 shadow-sm">
					<li>
						<button
							onclick={() => {
								logoutUser();
							}}>Logout</button
						>
					</li>
				</ul>
			</details>
		{:else}
			<a class="btn" href={resolve('/login')}>Login</a>
		{/if}
	</div>
</div>
<div class="relative flex flex-wrap items-center justify-center gap-2 bg-top p-4 xl:py-10">
	{@render children?.()}
</div>
