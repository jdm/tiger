<template>
	<div ref="titleBarElement" data-tauri-drag-region @dblclick="handleDoubleClick"
		class="h-11 flex justify-between bg-plastic-800 shadow-lg shadow-black/15">
		<div class="self-stretch">
			<slot name="left" />
		</div>
		<div class="flex space-x-4">
			<slot name="right" />
			<div class="flex-1 flex items-stretch">
				<button @click="appWindow.minimize" class="w-12 py-2 px-4 hover:bg-plastic-500">
					<div class="w-2.5 h-0 mx-auto border-t border-zinc-400" />
				</button>
				<button v-if="isMaximized" @click="appWindow.unmaximize" class="w-12 p-2 px-4 hover:bg-plastic-500">
					<div class="relative mx-auto w-2.5 h-2.5">
						<div class="absolute w-2 h-2 right-0 top-0 border border-zinc-400 " />
						<div class="absolute w-2 h-2 left-0 bottom-0 border border-zinc-400 bg-plastic-800" />
					</div>
				</button>
				<button v-if="!isMaximized" @click="appWindow.maximize" class="w-12 px-4 hover:bg-plastic-500">
					<div class="w-2.5 h-2.5 mx-auto border border-zinc-400" />
				</button>
				<button @click="requestExit" class="w-12 px-4 hover:bg-red-600" @mouseenter="xHovered = true"
					@mouseleave="xHovered = false">
					<div class="relative mx-auto w-3.5 h-3.5">
						<div class="absolute w-full h-px top-1/2 rotate-45" :class="xColor" />
						<div class="absolute w-full h-px top-1/2 -rotate-45" :class="xColor" />
					</div>
				</button>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { appWindow } from "@tauri-apps/api/window"
import { computed, onMounted, Ref, ref } from "vue"
import { requestExit } from "@/api/app"

const isMaximized = ref(false);
const titleBarElement: Ref<HTMLElement | null> = ref(null);

appWindow.listen("tauri://resize", updateIsMaximized);
onMounted(updateIsMaximized);

const xHovered = ref(false);
const xColor = computed(() => xHovered.value ? "bg-zinc-100" : "bg-zinc-400");

async function updateIsMaximized() {
	isMaximized.value = await appWindow.isMaximized();
}

async function handleDoubleClick(event: MouseEvent) {
	if (event.target != titleBarElement.value) {
		return;
	}
	if (isMaximized.value) {
		appWindow.unmaximize();
	} else {
		appWindow.maximize();
	}
}
</script>