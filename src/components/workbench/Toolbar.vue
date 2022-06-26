<template>
	<div class="p-2
		flex flex-row space-x-4 items-center
		rounded-md bg-neutral-900
		border-2 border-neutral-800
		outline outline-4 outline-neutral-900
	">
		<FlatMultiSwitch :items="zoomItems" @activate="onZoomClicked" />
		<FlatToggle icon="SunIcon" v-model="spriteDarkening" />
		<div class="flex flex-row items-center space-x-1">
			<FlatToggle icon="PhotographIcon" v-model="drawSprite" color="orange" />
			<FlatToggle icon="TagIcon" v-model="drawHitboxes" color="pink" />
			<FlatToggle icon="PlusIcon" v-model="drawOrigin" color="sky" />
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed, ref, WritableComputedRef } from "vue";
import { disableSpriteDarkening, enableSpriteDarkening, hideHitboxes, hideOrigin, hideSprite, setWorkbenchZoomFactor, showHitboxes, showOrigin, showSprite } from "@/api/document";
import { useAppStore } from "@/stores/app"
import FlatMultiSwitch, { FlatMultiSwitchItem } from "@/components/basic/FlatMultiSwitch.vue";
import FlatToggle from "@/components/basic/FlatToggle.vue"

const app = useAppStore();

const zoomItems = computed(() => {
	const zoom = app.currentDocument?.workbenchZoom || 1;
	return [
		{ text: "1x", active: zoom == 1, zoomFactor: 1 },
		{ text: "2x", active: zoom == 2, zoomFactor: 2 },
		{ text: "4x", active: zoom == 4, zoomFactor: 4 },
		{ text: "8x", active: zoom == 8, zoomFactor: 8 },
		{ text: "16x", active: zoom == 16, zoomFactor: 16 },
		{ text: "32x", active: zoom == 32, zoomFactor: 32 },
	];
});

function onZoomClicked(item: FlatMultiSwitchItem) {
	const zoomItem = zoomItems.value.find((i) => item == i);
	if (zoomItem) {
		setWorkbenchZoomFactor(zoomItem.zoomFactor);
	}
}

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

const drawSprite: WritableComputedRef<boolean> = computed({
	get: () => !app.currentDocument?.hideSprite,
	set: (toggled) => {
		if (toggled) {
			showSprite();
		} else {
			hideSprite();
		}
	},
});

const drawHitboxes: WritableComputedRef<boolean> = computed({
	get: () => !app.currentDocument?.hideHitboxes,
	set: (toggled) => {
		if (toggled) {
			showHitboxes();
		} else {
			hideHitboxes();
		}
	},
});


const drawOrigin: WritableComputedRef<boolean> = computed({
	get: () => !app.currentDocument?.hideOrigin,
	set: (toggled) => {
		if (toggled) {
			showOrigin();
		} else {
			hideOrigin();
		}
	},
});
</script>