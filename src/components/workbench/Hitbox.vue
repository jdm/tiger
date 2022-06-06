<template>
	<div>
		<!-- Selection indicator -->
		<!-- This intentionally uses SVG and not a plain div. We must use transform:scale() to size
			elements (for perf reasons and also because width/height animates one frame behind).
			However, transform:scale() also applies to borders, with no way of scaling back to 1px borders
			when zoomed in.
		 -->
		<svg class="absolute transition-transform z-20" :style="selectionStyle"
			:viewBox="`0 0 ${hitbox.size[0] + 1} ${hitbox.size[1] + 1}`">
			<rect :x="1" :y="1" :width="hitbox.size[0]" :height="hitbox.size[1]" shape-rendering="crispEdges"
				:stroke-width="1 / zoom"
				:class="hitbox.selected ? 'stroke-blue-600 fill-blue-600/20' : 'stroke-pink-600 fill-pink-600/20'"
				class="transition" style="transitionProperty: stroke-width" />
			<line :x1="1" :y1="1" :x2="1 + hitbox.size[0]" :y2="1 + hitbox.size[1]" :stroke-width="1 / zoom"
				:class="hitbox.selected ? 'stroke-blue-600' : 'stroke-pink-600'" class="transition"
				style="transitionProperty: stroke-width" />
			<line :x1="1 + hitbox.size[0]" :y1="1" :x2="1" :y2="1 + hitbox.size[1]" :stroke-width="1 / zoom"
				:class="hitbox.selected ? 'stroke-blue-600' : 'stroke-pink-600'" class="transition"
				style="transitionProperty: stroke-width" />
		</svg>
		<DragArea :buttons="['left', 'right']" active-cursor="cursor-move" inactive-cursor="cursor-move"
			@drag-start="startDrag" @drag-end="endDrag" @drag-update="updateDrag"
			class="absolute pointer-events-auto z-30" :style="frameStyle" />
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { Hitbox } from "@/api/dto"
import { useAppStore } from "@/stores/app"
import DragArea, { DragAreaEvent } from "@/components/basic/DragArea.vue"
import { beginNudgeHitbox, endNudgeHitbox, pan, selectHitbox, updateNudgeHitbox } from "@/api/document"

const app = useAppStore();

const props = defineProps<{
	hitbox: Hitbox,
	name: string,
	origin: [number, number],
}>();


const zoom = computed(() => app.currentDocument?.workbenchZoom || 1);
const left = computed(() => props.origin[0] + props.hitbox.topLeft[0]);
const top = computed(() => props.origin[1] + props.hitbox.topLeft[1]);

const frameStyle = computed(() => {
	const transformOrigin = [props.origin[0] - left.value, props.origin[1] - top.value];
	return {
		left: `${left.value}px`,
		top: `${top.value}px`,
		width: `${props.hitbox.size[0]}px`,
		height: `${props.hitbox.size[1]}px`,
		transform: `scale(${zoom.value}, ${zoom.value})`,
		transformOrigin: `${transformOrigin[0]}px ${transformOrigin[1]}px`,
	};
});


const nameStyle = computed(() => {
	return {
		left: `${props.origin[0]}px`,
		top: `${props.origin[1]}px`,
		width: `${zoom.value * props.hitbox.size[0]}px`,
		"max-height": `${zoom.value * props.hitbox.size[1]}px`,
		transform: `translate(
			${zoom.value * props.hitbox.topLeft[0]}px,
			${zoom.value * props.hitbox.topLeft[1]}px
		)`
	};
});

const selectionStyle = computed(() => {
	const transformOrigin = [props.origin[0] - left.value + 1, props.origin[1] - top.value + 1];
	return {
		left: `${left.value - 1}px`,
		top: `${top.value - 1}px`,
		width: `${props.hitbox.size[0] + 1}px`,
		height: `${props.hitbox.size[1] + 1}px`,
		transform: `scale(${zoom.value}, ${zoom.value})`,
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
