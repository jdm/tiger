<template>
	<div>
		<BoundingBox v-if="frameSize && drawBoundingBox" :position="position" :size="frameSize"
			:colorClasses="backgroundColor" />
		<img ref="imageElement" :src="sprite.getURL(keyframe.frame)" @load="onImageLoaded" @error="onImageError"
			class="absolute pixelated z-10" :class="frameClass" draggable="false" :style="frameStyle" />
		<BoundingBox v-if="frameSize && drawBoundingBox" :position="position" :size="frameSize"
			class="z-20 fill-transparent" :colorClasses="outlineColor" />
		<DragArea v-if="canInteract" :buttons="['left', 'right']" @mouseenter="onMouseEnter" @mouseleave="onMouseLeave"
			active-cursor="cursor-move" :inactive-cursor="keyframe.selected ? 'cursor-move' : 'cursor-pointer'"
			@drag-start="startDrag" @drag-end="endDrag" @drag-update="updateDrag"
			class="absolute pointer-events-auto z-40" :style="frameStyle" />
	</div>
</template>

<script setup lang="ts">
import { computed, CSSProperties, ref, Ref } from "vue"
import { Direction, Keyframe } from "@/api/dto"
import { useAppStore } from "@/stores/app"
import { beginNudgeKeyframe, endNudgeKeyframe, pan, selectKeyframe, updateNudgeKeyframe } from "@/api/document"
import DragArea, { DragAreaEvent } from "@/components/basic/DragArea.vue"
import BoundingBox from "@/components/workbench/BoundingBox.vue"
import { useSpriteStore } from "@/stores/sprite"

const app = useAppStore();
const sprite = useSpriteStore();

const props = defineProps<{
	keyframe: Keyframe,
	direction: Direction,
	index: number,
}>();

const imageElement: Ref<HTMLImageElement | null> = ref(null);
const hovered = ref(false);
const hasImage = ref(false);
const frameSize: Ref<[number, number] | null> = ref(null);

const isActiveFrame = computed(() => props.keyframe == app.currentKeyframe);
const canInteract = computed(() => !app.currentDocument?.timelineIsPlaying && (isActiveFrame.value || props.keyframe.selected) && frameSize.value);
const drawBoundingBox = computed(() => !app.currentDocument?.timelineIsPlaying && (isActiveFrame.value || props.keyframe.selected));

const position = computed(() => [
	-Math.floor((frameSize.value?.[0] || 0) / 2) + props.keyframe.offset[0],
	- Math.floor((frameSize.value?.[1] || 0) / 2) + props.keyframe.offset[1]
] as [number, number]);

const frameClass = computed(() => {
	return [
		(hasImage.value && isActiveFrame.value) ? "opacity-100" : "opacity-0",
		...(app.currentDocument?.darkenSprites ? ["saturate-50", "brightness-50", "contrast-125"] : []),
	];
});

const frameStyle = computed(() => {
	const size: [number, number] = frameSize.value || [0, 0];
	const left = -Math.floor((frameSize.value?.[0] || 0) / 2) + props.keyframe.offset[0];
	const top = -Math.floor((frameSize.value?.[1] || 0) / 2) + props.keyframe.offset[1];
	return {
		transform: `translate(${left}px, ${top}px)`,
		width: `${size[0]}px`,
		height: `${size[1]}px`,
		"backface-visibility": "hidden", // Fixes blurriness during zoom animations
	} as CSSProperties;
});

const showHover = computed(() => {
	return hovered.value
		&& (app.currentDocument?.hitboxesBeingNudged || []).length == 0
		&& (app.currentDocument?.hitboxesBeingResized || []).length == 0
		;
});

const backgroundColor = computed(() => {
	if (showHover.value) {
		return props.keyframe.selected ? "fill-blue-600/20" : "fill-orange-400/0";
	} else {
		return props.keyframe.selected ? "fill-blue-600/20" : "fill-orange-600/0";
	}
});

const outlineColor = computed(() => {
	if (showHover.value) {
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

function onImageLoaded() {
	if (imageElement.value) {
		frameSize.value = [imageElement.value.naturalWidth, imageElement.value.naturalHeight];
	} else {
		frameSize.value = null;
	}
	hasImage.value = (imageElement.value?.naturalWidth || 0) > 0;
}

function onImageError() {
	frameSize.value = null;
	hasImage.value = false;
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