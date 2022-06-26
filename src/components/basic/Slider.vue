<template>
	<DragArea inactive-cursor="cursor-pointer" active-cursor="cursor-pointer" :buttons="['left']"
		@drag-start="onDragStart" @drag-end="onDragEnd" @drag-update="onDragUpdate">
		<div ref="el" class="h-2 bg-plastic-900 rounded-md relative">
			<div class="h-full
				rounded-l-md bg-gradient-to-b from-blue-700 to-blue-600
				border-y border-t-blue-600 border-b-blue-900
				transition" :style="barStyle" />
			<div class="absolute top-0 bottom-0 left-0 right-2.5">
				<div class="w-full h-full relative">
					<div class="absolute top-1/2 right-0 w-3 h-3 -translate-y-1/2
					rounded-full bg-blue-200
					border-2 border-blue-600
					transition" :style="knobStyle" />
				</div>
			</div>
		</div>
	</DragArea>
</template>

<script setup lang="ts">
import { computed, Ref, ref, watch } from "vue";
import { debounceAnimation } from "@/utils/animation";
import DragArea, { DragAreaEvent } from "@/components/basic/DragArea.vue";

const props = defineProps<{
	value: number,
	dragging: boolean,
}>();

const emit = defineEmits(["update:value", "update:dragging"]);

const el: Ref<HTMLElement | null> = ref(null);
const animate = debounceAnimation(
	[() => props.dragging],
	() => !props.dragging,
);

const barStyle = computed(() => {
	return {
		transitionProperty: animate.value ? "width" : "none",
		width: `${100 * props.value}%`,
	}
});

const knobStyle = computed(() => {
	return {
		transitionProperty: animate.value ? "left" : "none",
		left: `${100 * props.value}%`,
	}
});

function onDragStart(event: DragAreaEvent) {
	emit("update:dragging", true);
	updateValue(event.mouseEvent);
}

function onDragEnd(event: DragAreaEvent) {
	emit("update:dragging", false);
}

function onDragUpdate(event: DragAreaEvent) {
	updateValue(event.mouseEvent);
}

function updateValue(event: MouseEvent) {
	if (!el.value) {
		return;
	}
	const sliderStartX = el.value.getBoundingClientRect().left;
	const newValue = (event.clientX - sliderStartX) / el.value.clientWidth;
	emit("update:value", newValue);
}
</script>