<template>
	<BoundingBox :origin="origin" :position="hitbox.topLeft" :size="hitbox.size" :darken="true"
		:colorClasses="hitbox.selected ? 'stroke-blue-600 fill-blue-600/20' : 'stroke-pink-600 fill-pink-600/10'"
		class="z-30" />
	<DragArea :buttons="['left', 'right']" active-cursor="cursor-move" inactive-cursor="cursor-move"
		@drag-start="startDrag" @drag-end="endDrag" @drag-update="updateDrag" class="absolute pointer-events-auto z-50"
		:style="dragAreaStyle" />
</template>

<script setup lang="ts">
import { computed } from "vue"
import { Hitbox } from "@/api/dto"
import { beginNudgeHitbox, endNudgeHitbox, pan, selectHitbox, updateNudgeHitbox } from "@/api/document"
import { useAppStore } from "@/stores/app"
import DragArea, { DragAreaEvent } from "@/components/basic/DragArea.vue"
import BoundingBox from "@/components/workbench/BoundingBox.vue"

const app = useAppStore();

const props = defineProps<{
	hitbox: Hitbox,
	name: string,
	origin: [number, number],
}>();

const dragAreaStyle = computed(() => {
	const zoom = app.currentDocument?.workbenchZoom;
	const left = props.origin[0] + props.hitbox.topLeft[0];
	const top = props.origin[1] + props.hitbox.topLeft[1];
	const transformOrigin = [props.origin[0] - left, props.origin[1] - top];
	return {
		left: `${left}px`,
		top: `${top}px`,
		width: `${props.hitbox.size[0]}px`,
		height: `${props.hitbox.size[1]}px`,
		transform: `scale(${zoom}, ${zoom})`,
		transformOrigin: `${transformOrigin[0]}px ${transformOrigin[1]}px`,
	};
});

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
