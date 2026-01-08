<script lang="ts">
	import { Github } from "@lucide/svelte";
	import { getVersion } from "@tauri-apps/api/app";
	import { openUrl } from "@tauri-apps/plugin-opener";
	import { onMount } from "svelte";

	let version = "";
	onMount(async () => {
		version = await getVersion();
	});

	async function openGithub() {
		await openUrl("https://github.com/sm0qs/stockmate");
	}
</script>

<div class="rounded-xl border border-accent p-5">
	<p>
		Stockmate is a desktop application designed to assist photographers in streamlining the
		process of uploading photos to multiple stock services and social media platforms. It
		focuses on efficient management of keywords, tags, and titles to optimize the submission
		workflow.
	</p>
	<button
		class="mt-2.5 flex cursor-pointer items-center gap-2 rounded
		       bg-accent px-4 py-2
		       font-medium text-on-accent transition hover:bg-accent-dark hover:text-light
		       "
		on:click={() => openGithub()}
	>
		<Github />
		View on GitHub
	</button>

	<p class="mt-2 text-sm">
		Version: {version}
	</p>
</div>
