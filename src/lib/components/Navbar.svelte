<script lang="ts">
	import { page } from "$app/state";
	import { Cloud, Image, Info, Settings } from "@lucide/svelte";

	const topIcons = [
		{ href: "/", icon: Image },
		{ href: "/upload", icon: Cloud },
	];

	const bottomIcons = [
		{ href: "/settings", icon: Settings },
		{ href: "/info", icon: Info },
	];
</script>

<nav class="flex h-screen w-14 flex-col justify-between border-r border-accent py-4 text-light/50">
	<div class="flex flex-col items-center space-y-2">
		{#each topIcons as { href, icon }}
			<a
				{href}
				class="flex h-10 w-10 items-center justify-center rounded-lg transition
                {page.url.pathname === href ? 'bg-accent-dark text-dark' : 'hover:bg-accent/50'}"
			>
				<svelte:component this={icon} size={25} />
			</a>
		{/each}
	</div>
	<div class="flex flex-col items-center space-y-2">
		{#each bottomIcons as { href, icon }}
			<a
				{href}
				class="flex h-10 w-10 items-center justify-center rounded-lg transition
                {page.url.pathname === href ? 'bg-accent-dark text-dark' : 'hover:bg-accent/50'}"
			>
				<svelte:component this={icon} size={25} />
			</a>
		{/each}
	</div>
</nav>
