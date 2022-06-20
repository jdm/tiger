<template>
	<div class="p-2 px-4
		flex flex-row space-x-10 items-center
		rounded-md bg-plastic-900
		border-2 border-plastic-700
		outline outline-4 outline-plastic-900
	">
		<Toggle :toggled="!app.currentDocument?.darkenSprites" @click="toggleSpriteDarkening" icon="SunIcon" />
		<div class="flex flex-row items-center font-xs font-semibold">
			<div
				class="w-11 h-11 inline-flex items-center justify-center text-plastic-200 border-2 border-blue-600 rounded-md">
				1x</div>
			<div class="w-11 h-11 flex items-center justify-center text-plastic-400"><span>2x</span></div>
			<div class="w-11 h-11 flex items-center justify-center text-plastic-400 "><span>4x</span></div>
			<div class="w-11 h-11 flex items-center justify-center text-plastic-400 "><span>8x</span></div>
			<div class="w-11 h-11 flex items-center justify-center text-plastic-400 "><span>16x</span></div>
		</div>
		<div class="flex flex-row items-center space-x-1.5">
			<Icon name="PhotographIcon" class="w-9 h-9 text-blue-200 p-2 rounded-md bg-blue-600" />
			<Icon name="TagIcon" class="w-9 h-9 text-blue-200 p-2 rounded-md bg-blue-600" />
			<Icon name="PlusIcon" class="w-9 h-9 text-plastic-200 p-2 rounded-md bg-plastic-700" />
			<Icon name="LocationMarkerIcon" class="w-9 h-9 text-plastic-200 p-2 rounded-md bg-plastic-700" />
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