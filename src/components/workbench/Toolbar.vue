<template>
	<div class="p-2 px-4
		flex flex-row space-x-10 items-center
		rounded-md bg-neutral-900
		border-2 border-neutral-800
		outline outline-4 outline-neutral-900
	">
		<FlatToggle icon="SunIcon" v-model="spriteDarkening" />
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
			<FlatToggle icon="PhotographIcon" v-model="drawFrames" color="orange" />
			<FlatToggle icon="TagIcon" v-model="drawHitboxes" color="pink" />
			<FlatToggle icon="PlusIcon" v-model="drawOrigin" color="sky" />
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed, ref, WritableComputedRef } from "vue";
import { disableSpriteDarkening, enableSpriteDarkening } from "@/api/document";
import { useAppStore } from "@/stores/app"
import Icon from "@/components/basic/Icon.vue"
import FlatToggle from "@/components/basic/FlatToggle.vue"

const app = useAppStore();

const spriteDarkening: WritableComputedRef<boolean> = computed({
	get: () => !app.currentDocument?.darkenSprites,
	set: (toggled) => {
		if (toggled) {
			disableSpriteDarkening();
		} else {
			enableSpriteDarkening();
		}
	},
});

const drawFrames = ref(true);
const drawHitboxes = ref(true);
const drawOrigin = ref(true);
</script>