<template>
	<div data-tauri-drag-region @dblclick="toggleMaximize"
		class="flex flex-row justify-between bg-plastic-800 shadow-md shadow-black/15">
		<div class="self-stretch">
			<slot name="left" />
		</div>
		<div class="flex flex-row px-5">
			<slot name="right" />
			<div class="flex-1 flex flex-row text-plastic-300">
				<button @click="appWindow.minimize" class="h-full p-2 px-6 hover:bg-plastic-600">➖</button>
				<button v-if="isMaximized" @click="appWindow.unmaximize"
					class="h-full p-2 px-6 hover:bg-plastic-600">🚸</button>
				<button v-if="!isMaximized" @click="appWindow.maximize"
					class="h-full p-2 px-6 hover:bg-plastic-600">⬜</button>
				<XIcon @click="appWindow.close" class="h-full p-2 px-6 hover:bg-red-700 hover:text-plastic-100" />
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window';
import { XIcon } from '@heroicons/vue/solid'
import { onMounted, ref } from 'vue';

const isMaximized = ref(false);

appWindow.listen('tauri://resize', updateIsMaximized);
onMounted(updateIsMaximized);

async function updateIsMaximized() {
	isMaximized.value = await appWindow.isMaximized();
}

async function toggleMaximize() {
	if (isMaximized.value) {
		appWindow.unmaximize();
	} else {
		appWindow.maximize();
	}
}
</script>