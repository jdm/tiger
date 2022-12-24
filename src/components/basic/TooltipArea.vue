<template>
	<div>
		<div @mouseenter="onMouseEnter" @mousemove="onMouseMove" @mouseleave="onMouseLeave" @mousedown="onMouseDown">
			<slot />
		</div>
		<FloatingWidget :open="showTooltip" :position="displayPosition">
			<Tooltip :text="text" />
		</FloatingWidget>
	</div>
</template>

<script setup lang="ts">
import { computed, onUnmounted, ref } from "vue";
import FloatingWidget from "@/components/basic/FloatingWidget.vue"
import Tooltip from "@/components/basic/Tooltip.vue"

defineProps<{
	text: string
}>();

const showTooltip = ref(false);
const position = ref([0, 0] as [number, number]);
const displayPosition = computed((): [number, number] => [position.value[0], 20 + position.value[1]]);
let timerHandle: number | undefined;

onUnmounted(cleanup);

function onMouseEnter(event: MouseEvent) {
	position.value = [event.clientX, event.clientY];
	timerHandle = window.setTimeout(() => {
		showTooltip.value = true;
	}, 500);
}

function onMouseMove(event: MouseEvent) {
	if (!showTooltip.value) {
		position.value = [event.clientX, event.clientY];
	}
}

function onMouseLeave() {
	cleanup();
}

function onMouseDown() {
	cleanup();
}

function cleanup() {
	showTooltip.value = false;
	window.clearTimeout(timerHandle);
}

</script>
