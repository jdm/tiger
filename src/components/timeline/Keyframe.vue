<template>
	<div class="pr-1">
		<div ref="el" class="h-full min-w-0 relative rounded-md border-2 cursor-pointer" :class="dynamicClasses">
			<div @click="onKeyframeClicked" class="h-full px-2 flex items-center font-semibold text-[11px]">
				<div class="min-w-0 overflow-hidden whitespace-nowrap text-ellipsis">{{ name }}</div>
			</div>
		</div>
		<DragArea v-if="!isPreview" @drag-start="beginDurationDrag" @drag-update="updateDurationDrag"
			@drag-end="endDurationDrag" inactive-cursor="cursor-ew-resize" active-cursor="cursor-ew-resize"
			class="absolute top-0 right-[-4px] h-full w-[16px]" />
	</div>
</template>

<script setup lang="ts">
import { computed, Ref, ref } from 'vue';
import { Direction } from '@/api/dto'
import { useAppStore } from '@/stores/app';
import { updateDragKeyframeDuration, selectKeyframe, endDragKeyframeDuration, beginDragKeyframeDuration } from '@/api/document';
import DragArea, { DragAreaEvent } from '@/components/basic/DragArea.vue';

const app = useAppStore();

const props = defineProps<{
	name: string,
	selected: boolean,
	startTimeMillis: number,
	durationMillis: number,
	direction: Direction,
	index: number,
	isPreview: boolean,
}>();

const dynamicClasses = computed(() => {
	if (props.isPreview) {
		return ["bg-orange-600", "border-orange-600", "animate-pulse"];
	}
	if (props.selected) {
		return ["text-blue-100", "bg-zinc-900", "border-blue-600"];
	}
	if (props.direction == app.currentDocument?.currentSequenceDirection) {
		return ["text-orange-200", "bg-plastic-900", "border-orange-600"];
	}
	return ["text-plastic-500", "bg-plastic-900", "border-plastic-500"];
});

let el: Ref<HTMLElement | null> = ref(null);
let dragReferenceTime = 0;

function mouseEventToTime(event: MouseEvent) {
	if (!el.value) {
		return 0;
	}
	const pixelDelta = event.clientX - el.value.getBoundingClientRect().left;
	const durationDelta = pixelDelta / (app.currentDocument?.timelineZoom || 1);
	return props.startTimeMillis + durationDelta;
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
