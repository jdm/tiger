<template>
	<div class="absolute">
		<div v-for="handle in handles" :key="handle.axis" :style="`transform:
			translate(${position[0]}px, ${position[1]}px)
			translate(${handle.tx}px, ${handle.ty}px)
		`">
			<div class="transition-transform" :style="`transform: scale(${1 / zoom}, ${1 / zoom})`">
				<ResizeHandle :axis="handle.axis" @resize-start="onResizeStart" @resize-update="onResizeUpdate"
					@resize-end="onResizeEnd" @drag-start="onDragStart" @drag-update="onDragUpdate"
					@drag-end="onDragEnd" />
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { ResizeAxis } from "@/backend/dto";
import { useStateStore } from "@/stores/state"
import { DragAreaEvent } from "@/components/basic/DragArea.vue";
import ResizeHandle from "@/components/workbench/ResizeHandle.vue"

export type ResizeEvent = {
	axis: ResizeAxis,
	dragEvent: DragAreaEvent,
}

const props = defineProps<{
	position: [number, number],
	size: [number, number],
}>();

const emit =
	defineEmits<{
		(e: "resizeStart", event: ResizeEvent): void
		(e: "resizeEnd", event: ResizeEvent): void
		(e: "resizeUpdate", event: ResizeEvent): void
		(e: "dragStart", event: DragAreaEvent): void
		(e: "dragUpdate", event: DragAreaEvent): void
		(e: "dragEnd", event: DragAreaEvent): void
	}>();

const state = useStateStore();
const zoom = computed(() => state.currentDocument?.workbenchZoom || 1);

const handles = computed(() => [
	{ axis: ResizeAxis.NW, tx:0, ty:0 },
	{ axis: ResizeAxis.N, tx: props.size[0] / 2, ty: 0 },
	{ axis: ResizeAxis.NE,tx: props.size[0], ty: 0 },
	{ axis: ResizeAxis.E, tx: props.size[0], ty: props.size[1] / 2 },
	{ axis: ResizeAxis.SE, tx: props.size[0], ty: props.size[1] },
	{ axis: ResizeAxis.S, tx: props.size[0] / 2, ty: props.size[1] },
	{ axis: ResizeAxis.SW, tx: 0, ty: props.size[1] },
	{ axis: ResizeAxis.W, tx:0, ty: props.size[1] / 2 },
]);


function onResizeStart(event: ResizeEvent) {
	emit("resizeStart", event);
}

function onResizeUpdate(event: ResizeEvent) {
	emit("resizeUpdate", event);
}

function onResizeEnd(event: ResizeEvent) {
	emit("resizeEnd", event);
}

function onDragStart(event: DragAreaEvent) {
	emit("dragStart", event);
}

function onDragUpdate(event: DragAreaEvent) {
	emit("dragUpdate", event);
}

function onDragEnd(event: DragAreaEvent) {
	emit("dragEnd", event);
}
</script>