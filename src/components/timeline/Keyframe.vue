<template>
	<div class="relative min-w-0 px-2 rounded-sm border-x border-x-plastic-800" :class="dynamicClasses">
		<div @click="onKeyframeClicked" class="h-full flex items-center font-semibold text-xs overflow-hidden">
			{{ keyframe.name }}
		</div>
		<DragArea @drag-start="beginDurationDrag" @drag-update="updateDurationDrag" inactive-cursor="cursor-ew-resize"
			active-cursor="cursor-ew-resize" class="absolute top-0 -right-[7px] z-10 h-full w-[15px]" />
	</div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { Direction, Keyframe as KeyframeDTO } from '@/api/dto'
import { useAppStore } from '@/stores/app';
import { selectKeyframe } from '@/api/document';
import DragArea, { DragAreaEvent } from '@/components/basic/DragArea.vue';

const app = useAppStore();

const props = defineProps<{
	keyframe: KeyframeDTO,
	direction: Direction,
	index: number
}>();

let initialMousePosition = 0;

const dynamicClasses = computed(() => {
	return [
		...props.keyframe.selected ? ["text-rose-900", "bg-rose-200",] : ["text-rose-200", "bg-rose-600",],
	];
});

function onKeyframeClicked(event: MouseEvent) {
	selectKeyframe(props.direction, props.index, event.shiftKey, event.ctrlKey);
}

function beginDurationDrag(e: DragAreaEvent) {
	initialMousePosition = e.mouseEvent.clientX;
	// TODO select keyframe if not already selected
}

function updateDurationDrag(e: DragAreaEvent) {
	const pixelDelta = e.mouseEvent.clientX - initialMousePosition;
	const durationDelta = pixelDelta / (app.currentDocument?.timelineZoom || 1);
	// TODO adjust duration on all selected keyframes
	console.log(durationDelta);
}
</script>