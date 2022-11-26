<template>
	<div class="absolute">
		<div :style="`transform: translate(${position[0]}px, ${position[1]}px)`">
			<div class="transition-transform"
				:style="`transform-origin: center left; transform: scale(${scale}, ${scale})`">
				<div class="transition-transform" :style="`transform: scale(${1 / zoom}, ${1 / zoom})`">
					<div class="absolute text-xs px-1 py-0.5" :class="palette"
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
import { useAppStore } from "@/stores/app";

const props = defineProps<{
	color: "blue" | "pink",
	hovered: boolean,
	position: [number, number],
	size: [number, number],
	text: string,
}>();

const app = useAppStore();
const zoom = computed(() => app.currentDocument?.workbenchZoom || 1);

const scale = computed(() => {
	if (zoom.value * props.size[0] < 80) {
		return 0;
	}
	if (zoom.value * props.size[1] < 30) {
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
