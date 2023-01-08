<template>
	<div class="absolute">
		<div :style="`transform: translate(${position[0]}px, ${position[1]}px)`">
			<div class="transition-transform"
				:style="`transform-origin: center left; transform: scale(${scale}, ${scale})`">
				<div class="transition-transform" :style="`transform: scale(${1 / zoom}, ${1 / zoom})`">
					<div class="absolute px-1 py-px font-semibold text-[10px]" :class="palette"
						:style="`max-width: ${zoom * size[0]}px`">
						<div class="text-ellipsis overflow-clip whitespace-nowrap">
							{{ text }}
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useStateStore } from "@/stores/state";

const props = defineProps<{
	color: "blue" | "pink",
	hovered: boolean,
	position: [number, number],
	size: [number, number],
	text: string,
}>();

const state = useStateStore();
const zoom = computed(() => state.currentDocument?.workbenchZoom || 1);

const scale = computed(() => {
	const w = zoom.value * props.size[0];
	const h = zoom.value * props.size[1];
	if (w < 32 || h < 32) {
		return 0;
	}
	if (w * h < 64 * 64) {
		return 0;
	}
	return 1;
});

const palette = computed(() => {
	if (props.color == "pink") {
		return [props.hovered ? "bg-pink-400" : "bg-pink-600", "text-pink-100"];
	}
	return [props.hovered ? "bg-blue-400" : "bg-blue-600", "text-blue-100"];
});
</script>
