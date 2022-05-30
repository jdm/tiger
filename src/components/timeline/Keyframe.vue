<template>
	<div ref="el" class="relative min-w-0 rounded-md mx-[2px] cursor-pointer" :class="dynamicClasses">
		<div @click="onKeyframeClicked" class="h-full flex items-center font-semibold text-xs">
			<div class="min-w-0 px-2 overflow-hidden whitespace-nowrap text-ellipsis">{{ keyframe.name }}</div>
		</div>
		<DragArea @drag-start="beginDurationDrag" @drag-update="updateDurationDrag" @drag-end="endDurationDrag"
			inactive-cursor="cursor-ew-resize" active-cursor="cursor-ew-resize"
			class="absolute top-0 -right-[1px] translate-x-1/2 z-10 h-full w-[16px]" />
	</div>
</template>

<script setup lang="ts">
import { computed, Ref, ref } from 'vue';
import { Direction, Keyframe as KeyframeDTO } from '@/api/dto'
import { useAppStore } from '@/stores/app';
import { updateDragKeyframeDuration, selectKeyframe, endDragKeyframeDuration, beginDragKeyframeDuration } from '@/api/document';
import DragArea, { DragAreaEvent } from '@/components/basic/DragArea.vue';

const app = useAppStore();

const props = defineProps<{
	keyframe: KeyframeDTO,
	direction: Direction,
	index: number
}>();

const dynamicClasses = computed(() => {
	return [
		...props.keyframe.selected ?
			["text-blue-100", "bg-slate-900", "border-2", "border-blue-600", "outline", "outline-plastic-800"]
			: ["text-orange-200", "bg-plastic-900", "border-2", "border-orange-600", "outline", "outline-plastic-800"],
	];
});

let el: Ref<HTMLElement | null> = ref(null);
let dragReferenceTime = 0;

function mouseEventToTime(event: MouseEvent) {
	if (!el.value) {
		return 0;
	}
	const pixelDelta = event.clientX - el.value.getBoundingClientRect().left;
	const durationDelta = pixelDelta / (app.currentDocument?.timelineZoom || 1);
	console.log(props.keyframe.startTimeMillis + durationDelta);
	return props.keyframe.startTimeMillis + durationDelta;
}

function onKeyframeClicked(event: MouseEvent) {
	selectKeyframe(props.direction, props.index, event.shiftKey, event.ctrlKey);
}

function beginDurationDrag(e: DragAreaEvent) {
	beginDragKeyframeDuration(props.direction, props.index);
	dragReferenceTime = mouseEventToTime(e.mouseEvent);
}

function updateDurationDrag(e: DragAreaEvent) {
	const deltaMillis = mouseEventToTime(e.mouseEvent) - dragReferenceTime;
	updateDragKeyframeDuration(Math.round(deltaMillis));
}

function endDurationDrag(e: DragAreaEvent) {
	endDragKeyframeDuration();
}
</script>
