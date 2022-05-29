<template>
	<div class="relative min-w-0">
		<div
			class="h-full flex items-center rounded-sm px-2 bg-amber-500 border-r border-amber-900 text-amber-900 text-xs font-semibold overflow-hidden">
			{{ keyframe.name }}
		</div>
		<DragArea @drag-start="beginDurationDrag" @drag-update="updateDurationDrag" inactive-cursor="cursor-ew-resize"
			active-cursor="cursor-ew-resize" class="absolute top-0 -right-[7px] z-10 h-full w-[15px]" />
	</div>
</template>

<script setup lang="ts">
import { Keyframe as KeyframeDTO } from '@/api/dto'
import { useAppStore } from '@/stores/app';
import DragArea, { DragAreaEvent } from '@/components/basic/DragArea.vue';

const app = useAppStore();

const props = defineProps<{ keyframe: KeyframeDTO }>();

let initialMousePosition = 0;

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