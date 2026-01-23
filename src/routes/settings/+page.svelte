<script lang="ts">
	import {
		choosePath,
		cleanPath,
		deletePath,
		openPath,
		savePath,
		savedPaths,
		hasSavedPaths,
		editPath,
	} from "$lib/utils/path-handler";
	import { FolderOpen, Plus, SquarePen, Trash } from "@lucide/svelte";

	let path_input: string = "";

	async function handleSave() {
		await savePath(path_input);
		if ($savedPaths.length > 0) {
			path_input = "";
		}
	}
</script>

<h1 class="mb-5 text-5xl font-semibold">Settings</h1>
<div class="rounded-xl border border-accent p-5">
	<div class="mb-5">
		<h2 class="text-xl font-semibold">Photo directories:</h2>
		<p>Manage directories where the application will look for photos.</p>
	</div>
	<div class="flex items-center gap-2">
		<input
			type="text"
			disabled={$hasSavedPaths}
			placeholder="Enter a full directory path..."
			class="rounded border-2 border-accent p-2 outline-none"
			class:opacity-50={$hasSavedPaths}
			class:cursor-not-allowed={$hasSavedPaths}
			bind:value={path_input}
			on:keydown={(e) => e.key === "Enter" && handleSave()}
		/>

		<button
			type="button"
			disabled={$hasSavedPaths}
			class="flex items-center gap-2 rounded border-2 border-accent bg-accent px-4 py-2 text-on-accent transition hover:border-accent-dark hover:bg-accent-dark hover:text-light"
			class:opacity-50={$hasSavedPaths}
			class:cursor-not-allowed={$hasSavedPaths}
			class:cursor-pointer={!$hasSavedPaths}
			on:click={handleSave}
		>
			<Plus />
			Add
		</button>

		<button
			type="button"
			disabled={$hasSavedPaths}
			class="flex items-center gap-2 rounded border-2 border-accent px-4 py-2 transition hover:bg-accent hover:text-on-accent"
			class:opacity-50={$hasSavedPaths}
			class:cursor-not-allowed={$hasSavedPaths}
			class:cursor-pointer={!$hasSavedPaths}
			on:click={choosePath}
		>
			<FolderOpen />
			Choose
		</button>
	</div>

	{#if $hasSavedPaths}
		<p class="mt-3 text-sm text-yellow-600">
			For now, only one photo directory can be added at a time. Please delete the existing one
			to add a new directory.
		</p>
	{/if}
	<div class="mt-5 space-y-2">
		{#if $hasSavedPaths}
			{#each $savedPaths as p}
				<div
					class="flex items-center justify-between rounded border-2 border-accent px-4 py-2"
				>
					<button
						type="button"
						class="text-left break-all select-all hover:underline"
						on:click={() => openPath(p)}
					>
						{cleanPath(p)}
					</button>
					<div class="flex gap-2">
						<button
							type="button"
							class="flex h-8 w-8 cursor-pointer items-center justify-center rounded-lg bg-yellow-600 hover:bg-yellow-800"
							on:click={() => editPath(p)}
						>
							<SquarePen size={20} />
						</button>
						<button
							type="button"
							class="flex h-8 w-8 cursor-pointer items-center justify-center rounded-lg bg-red-500 hover:bg-red-800"
							on:click={() => deletePath(p)}
						>
							<Trash size={20} />
						</button>
					</div>
				</div>
			{/each}
		{:else}
			<div class="rounded border-2 border-dashed border-gray-500 p-8 text-gray-500">
				<p class="font-semibold">No photo directories have been added yet.</p>
				<p>Use the input field or the "Choose" button to add a directory.</p>
			</div>
		{/if}
	</div>
</div>
