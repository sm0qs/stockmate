<script lang="ts">
	import { goto } from "$app/navigation";
	import { page } from "$app/state";
	import { Cloud, Image, Info, Settings } from "@lucide/svelte";

	const btnClass = (href: string) =>
		`flex h-10 w-10 items-center justify-center rounded-lg transition ${
			page.url.pathname === href ? "bg-accent text-on-accent" : "hover:bg-accent-dark"
		}`;
</script>

<nav class="flex h-full w-14 flex-col justify-between border-r border-accent py-4 text-light/50">
	<div class="flex flex-col items-center space-y-2">
		<button type="button" onclick={() => goto("/")} class={btnClass("/")}>
			<Image size={24} />
		</button>

		<button type="button" onclick={() => goto("/upload")} class={btnClass("/upload")}>
			<Cloud size={24} />
		</button>
	</div>
	<div class="flex flex-col items-center space-y-2">
		<button type="button" onclick={() => goto("/settings")} class={btnClass("/settings")}>
			<Settings size={24} />
		</button>

		<button type="button" onclick={() => goto("/info")} class={btnClass("/info")}>
			<Info size={24} />
		</button>
	</div>
</nav>
