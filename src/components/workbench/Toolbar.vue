<template>
	<div class="p-2 px-4
		flex flex-row space-x-10 items-center
		rounded-md bg-neutral-900
		border-2 border-neutral-800
		outline outline-4 outline-neutral-900
	">
		<!-- <Toggle :toggled="!app.currentDocument?.darkenSprites" @click="toggleSpriteDarkening" icon="SunIcon" /> -->
		<Icon name="SunIcon" class="w-9 h-9 text-plastic-400 p-2 rounded-md bg-plastic-700" />
		<div class="flex flex-row items-center font-xs font-semibold">
			<div
				class="w-10 h-9 inline-flex items-center justify-center text-plastic-200 border-2 border-blue-600 rounded-md">
				1x</div>
			<div class="w-10 h-9 flex items-center justify-center text-plastic-400"><span>2x</span></div>
			<div class="w-10 h-9 flex items-center justify-center text-plastic-400 "><span>4x</span></div>
			<div class="w-10 h-9 flex items-center justify-center text-plastic-400 "><span>8x</span></div>
			<div class="w-10 h-9 flex items-center justify-center text-plastic-400 "><span>16x</span></div>
		</div>
		<div class="flex flex-row items-center space-x-1.5">
			<Icon name="PhotographIcon" class="w-9 h-9 text-orange-200 p-2 rounded-md bg-orange-700" />
			<Icon name="TagIcon" class="w-9 h-9 text-pink-200 p-2 rounded-md bg-pink-700" />
			<Icon name="PlusIcon" class="w-9 h-9 text-sky-200 p-2 rounded-md bg-sky-600" />
		</div>
	</div>
</template>

<script setup lang="ts">
import { disableSpriteDarkening, enableSpriteDarkening } from "@/api/document";
import { useAppStore } from "@/stores/app"
import Icon from "@/components/basic/Icon.vue"
import Toggle from "@/components/basic/Toggle.vue"

const app = useAppStore();

function toggleSpriteDarkening() {
	if (app.currentDocument?.darkenSprites) {
		disableSpriteDarkening();
	} else {
		enableSpriteDarkening();
	}
}
</script>