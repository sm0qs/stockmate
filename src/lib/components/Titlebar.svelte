<script lang="ts">
	import { Minus, Square, Copy, X } from "@lucide/svelte";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { onMount } from "svelte";

	const appWindow = getCurrentWindow();
	let isMaximized = $state(false);
	const btnClass =
		"flex w-12 items-center justify-center text-light/50 hover:text-light transition";

	async function updateWindowState() {
		isMaximized = await appWindow.isMaximized();
	}

	onMount(() => {
		updateWindowState();

		const unlisten = appWindow.onResized(updateWindowState);

		return () => {
			unlisten.then((f) => f());
		};
	});

	function handleTitlebarAction(e: MouseEvent) {
		if (e.buttons === 1) {
			if (e.detail === 2) {
				appWindow.toggleMaximize();
			} else {
				appWindow.startDragging();
			}
		}
	}
</script>

<div class="flex h-9 border-b border-accent">
	<div
		role="presentation"
		onmousedown={handleTitlebarAction}
		class="flex flex-1 items-center [-webkit-app-region:drag] [app-region:drag]"
	>
		<img src="/app-icon.svg" alt="Stockmate Logo" class="h-5 px-2" />
		<span class="text-s font-semibold">Stockmate</span>
	</div>

	<div class="flex">
		<button onclick={() => appWindow.minimize()} class="{btnClass} hover:bg-accent-dark">
			<div class="translate-y-1">
				<Minus size={14} strokeWidth={3} />
			</div>
		</button>
		<button onclick={() => appWindow.toggleMaximize()} class="{btnClass} hover:bg-accent-dark">
			{#if isMaximized}
				<Copy size={14} class="scale-x-[-1] rotate-180" />
			{:else}
				<Square size={14} />
			{/if}
		</button>
		<button onclick={() => appWindow.close()} class="{btnClass} hover:bg-red-500">
			<X size={16} />
		</button>
	</div>
</div>
