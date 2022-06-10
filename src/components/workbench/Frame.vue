<template>
	<div>
		<BoundingBox v-if="frameSize && drawBoundingBox" :origin="origin" :position="position" :size="frameSize"
			:colorClasses="backgroundColor" />
		<img ref="el" :src="convertFileSrc(keyframe.frame)" @load="onFrameLoaded"
			class="absolute pixelated transition-transform z-10" :class="frameClass" draggable="false"
			:style="frameStyle" />
		<BoundingBox v-if="frameSize && drawBoundingBox" :origin="origin" :position="position" :size="frameSize"
			class="z-20 fill-transparent" :colorClasses="outlineColor" />
		<DragArea v-if="frameSize && (isActiveFrame || keyframe.selected)" :buttons="['left', 'right']"
			@mouseenter="onMouseEnter" @mouseleave="onMouseLeave" active-cursor="cursor-move"
			inactive-cursor="cursor-move" @drag-start="startDrag" @drag-end="endDrag" @drag-update="updateDrag"
			class="absolute pointer-events-auto z-40" :style="frameStyle" />
	</div>
</template>

<script setup lang="ts">
import { convertFileSrc } from "@tauri-apps/api/tauri"
import { computed, ref, Ref } from "vue"
import { Direction, Keyframe } from "@/api/dto"
import { useAppStore } from "@/stores/app"
import { beginNudgeKeyframe, endNudgeKeyframe, pan, selectKeyframe, updateNudgeKeyframe } from "@/api/document"
import DragArea, { DragAreaEvent } from "@/components/basic/DragArea.vue"
import BoundingBox from "@/components/workbench/BoundingBox.vue"

const app = useAppStore();

const props = defineProps<{
	keyframe: Keyframe,
	origin: [number, number],
	direction: Direction,
	index: number,
}>();

const el: Ref<HTMLImageElement | null> = ref(null);
const hovered = ref(false);
const frameSize: Ref<[number, number] | null> = ref(null);

const isActiveFrame = computed(() => props.keyframe == app.currentKeyframe);
const drawBoundingBox = computed(() => !app.currentDocument?.timelineIsPlaying && (isActiveFrame.value || props.keyframe.selected));

const position = computed(() => [
	-Math.floor((frameSize.value?.[0] || 0) / 2) + props.keyframe.offset[0],
	- Math.floor((frameSize.value?.[1] || 0) / 2) + props.keyframe.offset[1]
] as [number, number]);

const frameClass = computed(() => {
	return [
		(frameSize.value && isActiveFrame.value) ? "opacity-100" : "opacity-0",
		...(app.currentDocument?.darkenSprites ? ["saturate-50", "brightness-50", "contrast-125"] : []),
	];
});

const frameStyle = computed(() => {
	const zoom = app.currentDocument?.workbenchZoom || 1;
	const size: [number, number] = frameSize.value || [0, 0];
	const left = props.origin[0] - Math.floor((frameSize.value?.[0] || 0) / 2) + props.keyframe.offset[0];
	const top = props.origin[1] - Math.floor((frameSize.value?.[1] || 0) / 2) + props.keyframe.offset[1];
	const transformOrigin = [props.origin[0] - left, props.origin[1] - top];
	return {
		left: `${left}px`,
		top: `${top}px`,
		width: `${size[0]}px`,
		height: `${size[1]}px`,
		transform: `scale(${zoom}, ${zoom})`,
		transformOrigin: `${transformOrigin[0]}px ${transformOrigin[1]}px`,
	};
});

const backgroundColor = computed(() => {
	if (hovered.value) {
		return props.keyframe.selected ? "fill-blue-600/20" : "fill-orange-400/0";
	} else {
		return props.keyframe.selected ? "fill-blue-600/20" : "fill-orange-600/0";
	}
});

const outlineColor = computed(() => {
	if (hovered.value) {
		return props.keyframe.selected ? "stroke-blue-400" : "stroke-orange-400";
	} else {
		return props.keyframe.selected ? "stroke-blue-600" : "stroke-orange-600";
	}
});

function onMouseEnter() {
	hovered.value = true;
}

function onMouseLeave() {
	hovered.value = false;
}

function onFrameLoaded() {
	if (!el.value) {
		return;
	}
	frameSize.value = [el.value.naturalWidth, el.value.naturalHeight];
}

function startDrag(event: DragAreaEvent) {
	if (event.button == "left") {
		beginNudgeKeyframe(props.direction, props.index);
	}
}

function endDrag(event: DragAreaEvent) {
	if (event.button == "left") {
		if (event.didMove) {
			endNudgeKeyframe();
		} else {
			selectKeyframe(props.direction, props.index, event.mouseEvent.shiftKey, event.mouseEvent.ctrlKey);
		}
	}
}

function updateDrag(event: DragAreaEvent) {
	if (event.button == "left") {
		const displacement: [number, number] = [
			event.mouseEvent.clientX - event.initialMouseEvent.clientX,
			event.mouseEvent.clientY - event.initialMouseEvent.clientY,
		];
		updateNudgeKeyframe(displacement, !event.mouseEvent.shiftKey);
	} else if (event.button == "right") {
		pan([event.mouseEvent.movementX, event.mouseEvent.movementY]);
	}
}

</script>