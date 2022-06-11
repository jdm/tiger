<template>
	<BoundingBox :position="hitbox.topLeft" :size="hitbox.size" :darken="true" :colorClasses="boundingBoxClass"
		class="z-30" />
	<DragArea v-if="!app.currentDocument?.timelineIsPlaying" :buttons="['left', 'right']" active-cursor="cursor-move"
		:inactive-cursor="hitbox.selected ? 'cursor-move' : 'cursor-pointer'" @mouseenter="onMouseEnter"
		@mouseleave="onMouseLeave" @drag-start="startDrag" @drag-end="endDrag" @drag-update="updateDrag"
		class="absolute pointer-events-auto z-50" :style="dragAreaStyle" />
</template>

<script setup lang="ts">
import { computed, ref } from "vue"
import { Hitbox } from "@/api/dto"
import { beginNudgeHitbox, endNudgeHitbox, pan, selectHitbox, updateNudgeHitbox } from "@/api/document"
import { useAppStore } from "@/stores/app"
import DragArea, { DragAreaEvent } from "@/components/basic/DragArea.vue"
import BoundingBox from "@/components/workbench/BoundingBox.vue"

const app = useAppStore();

const props = defineProps<{
	hitbox: Hitbox,
	name: string,
}>();

const hovered = ref(false);

const dragAreaStyle = computed(() => {
	return {
		transform: `translate(${props.hitbox.topLeft[0]}px, ${props.hitbox.topLeft[1]}px)`,
		width: `${props.hitbox.size[0]}px`,
		height: `${props.hitbox.size[1]}px`,
	};
});

const boundingBoxClass = computed(() => {
	return [
		...(hovered.value && props.hitbox.selected ? ["stroke-blue-400", "fill-blue-600/20"] : []),
		...(!hovered.value && props.hitbox.selected ? ["stroke-blue-600", "fill-blue-600/20"] : []),
		...(hovered.value && !props.hitbox.selected ? ["stroke-pink-400", "fill-pink-600/10"] : []),
		...(!hovered.value && !props.hitbox.selected ? ["stroke-pink-600", "fill-pink-600/10"] : []),
	];
});

function onMouseEnter() {
	hovered.value = true;
}

function onMouseLeave() {
	hovered.value = false;
}

function startDrag(event: DragAreaEvent) {
	if (event.button == "left") {
		beginNudgeHitbox(props.name);
	}
}

function endDrag(event: DragAreaEvent) {
	if (event.button == "left") {
		if (event.didMove) {
			endNudgeHitbox();
		} else {
			selectHitbox(props.name, event.mouseEvent.shiftKey, event.mouseEvent.ctrlKey);
		}
	}
}

function updateDrag(event: DragAreaEvent) {
	if (event.button == "left") {
		const displacement: [number, number] = [
			event.mouseEvent.clientX - event.initialMouseEvent.clientX,
			event.mouseEvent.clientY - event.initialMouseEvent.clientY,
		];
		updateNudgeHitbox(displacement, !event.mouseEvent.shiftKey);
	} else if (event.button == "right") {
		pan([event.mouseEvent.movementX, event.mouseEvent.movementY]);
	}
}
</script>
