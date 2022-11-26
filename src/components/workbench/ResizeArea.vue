<template>
	<div class="relative">
		<div v-for="handle in handles" class="absolute" :class="handle.class">
			<div :style="`transform: translate(${handle.tx || 0}px, ${handle.ty || 0}px)`">
				<ResizeHandle :axis="handle.axis" @resize-start="onResizeStart" @resize-update="onResizeUpdate"
					@resize-end="onResizeEnd" @drag-start="onDragStart" @drag-update="onDragUpdate"
					@drag-end="onDragEnd" class="transition-transform"
					:style="`transform: scale(${1 / zoom}, ${1 / zoom})`" />
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { ResizeAxis } from "@/api/dto";
import { useAppStore } from "@/stores/app"
import { DragAreaEvent } from "@/components/basic/DragArea.vue";
import ResizeHandle from "@/components/workbench/ResizeHandle.vue"

export type ResizeEvent = {
	axis: ResizeAxis,
	dragEvent: DragAreaEvent,
}

const props = defineProps<{
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

const app = useAppStore();
const zoom = computed(() => app.currentDocument?.workbenchZoom || 1);

const handles = computed(() => [
	{ axis: ResizeAxis.NW, class: "top-0 left-0 -translate-x-1/2 -translate-y-1/2" },
	{ axis: ResizeAxis.N, class: "top-0 left-0 -translate-x-1/2 -translate-y-1/2", tx: props.size[0] / 2 },
	{ axis: ResizeAxis.NE, class: "top-0 right-0 translate-x-1/2 -translate-y-1/2" },
	{ axis: ResizeAxis.E, class: "top-0 right-0 translate-x-1/2 -translate-y-1/2", ty: props.size[1] / 2 },
	{ axis: ResizeAxis.SE, class: "bottom-0 right-0 translate-x-1/2 translate-y-1/2" },
	{ axis: ResizeAxis.S, class: "bottom-0 left-0 -translate-x-1/2 translate-y-1/2", tx: props.size[0] / 2 },
	{ axis: ResizeAxis.SW, class: "bottom-0 left-0 -translate-x-1/2 translate-y-1/2" },
	{ axis: ResizeAxis.W, class: "top-0 left-0 -translate-x-1/2 -translate-y-1/2", ty: props.size[1] / 2 },
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