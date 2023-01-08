<template>
	<div class="p-2
		flex gap-4 items-center
		rounded-md bg-neutral-900
		border-2 border-neutral-800
		outline outline-4 outline-neutral-900
	">
		<FlatMultiSwitch :items="zoomItems" @activate="onZoomClicked" />
		<TooltipArea text="Toggle sprite darkening">
			<FlatToggle :icon="SunIcon" v-model="spriteDarkening" />
		</TooltipArea>
		<div class="flex items-center gap-1">
			<TooltipArea text="Toggle frame visibility">
				<FlatToggle :icon="PhotoIcon" v-model="drawSprite" color="orange" />
			</TooltipArea>
			<TooltipArea text="Toggle hitboxes visibility">
				<FlatToggle :icon="TagIcon" v-model="drawHitboxes" color="pink" />
			</TooltipArea>
			<TooltipArea text="Toggle origin visibility">
				<FlatToggle :icon="PlusIcon" v-model="drawOrigin" color="sky" />
			</TooltipArea>
		</div>
	</div>
</template>

<script setup lang="ts">
import { PhotoIcon, PlusIcon, SunIcon, TagIcon  } from "@heroicons/vue/20/solid";
import { computed, WritableComputedRef } from "vue";
import { disableSpriteDarkening, enableSpriteDarkening, hideHitboxes, hideOrigin, hideSprite, setWorkbenchZoomFactor, showHitboxes, showOrigin, showSprite } from "@/backend/api";
import { useStateStore } from "@/stores/state"
import FlatMultiSwitch, { FlatMultiSwitchItem } from "@/components/basic/FlatMultiSwitch.vue";
import FlatToggle from "@/components/basic/FlatToggle.vue"
import TooltipArea from "@/components/basic/TooltipArea.vue"

const state = useStateStore();

const zoomItems = computed(() => {
	const zoom = state.currentDocument?.workbenchZoom || 8;
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
	get: () => !state.currentDocument?.darkenSprites,
	set: (toggled) => {
		if (toggled) {
			disableSpriteDarkening();
		} else {
			enableSpriteDarkening();
		}
	},
});

const drawSprite: WritableComputedRef<boolean> = computed({
	get: () => !state.currentDocument?.hideSprite,
	set: (toggled) => {
		if (toggled) {
			showSprite();
		} else {
			hideSprite();
		}
	},
});

const drawHitboxes: WritableComputedRef<boolean> = computed({
	get: () => !state.currentDocument?.hideHitboxes,
	set: (toggled) => {
		if (toggled) {
			showHitboxes();
		} else {
			hideHitboxes();
		}
	},
});

const drawOrigin: WritableComputedRef<boolean> = computed({
	get: () => !state.currentDocument?.hideOrigin,
	set: (toggled) => {
		if (toggled) {
			showOrigin();
		} else {
			hideOrigin();
		}
	},
});
</script>
