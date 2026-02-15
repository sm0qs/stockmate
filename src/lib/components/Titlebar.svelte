<script lang="ts">
	import { Minus, Square, X } from "@lucide/svelte";
	import { getCurrentWindow } from "@tauri-apps/api/window";

	const appWindow = getCurrentWindow();
	const btnBase = "flex w-10 items-center justify-center transition";

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
	<div role="presentation" onmousedown={handleTitlebarAction} class="flex flex-1 items-center">
		<img src="/app-icon.svg" alt="Stockmate Logo" class="h-5 px-2" />
		<span class="text-s font-semibold">Stockmate</span>
	</div>

	<div class="flex">
		<button onclick={() => appWindow.minimize()} class="{btnBase} hover:bg-accent-dark">
			<div class="translate-y-1">
				<Minus size={14} strokeWidth={3} />
			</div>
		</button>
		<button onclick={() => appWindow.toggleMaximize()} class="{btnBase} hover:bg-accent-dark">
			<Square size={14} />
		</button>
		<button onclick={() => appWindow.close()} class="{btnBase} hover:bg-red-500">
			<X size={16} />
		</button>
	</div>
</div>
