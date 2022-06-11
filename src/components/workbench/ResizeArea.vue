<template>
	<div>
		<div class="h-full relative">
			<div v-for="handle in handles" class="absolute" :class="handle.class">
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

const app = useAppStore();
const zoom = computed(() => app.currentDocument?.workbenchZoom || 1);

const handles = [
	{ axis: ResizeAxis.NW, class: "top-0 left-0 -translate-x-1/2 -translate-y-1/2" },
	{ axis: ResizeAxis.N, class: "top-0 left-1/2 -translate-x-1/2 -translate-y-1/2" },
	{ axis: ResizeAxis.NE, class: "top-0 right-0 translate-x-1/2 -translate-y-1/2" },
	{ axis: ResizeAxis.E, class: "top-1/2 right-0 translate-x-1/2 -translate-y-1/2" },
	{ axis: ResizeAxis.SE, class: "bottom-0 right-0 translate-x-1/2 translate-y-1/2" },
	{ axis: ResizeAxis.S, class: "bottom-0 left-1/2 -translate-x-1/2 translate-y-1/2" },
	{ axis: ResizeAxis.SW, class: "bottom-0 left-0 -translate-x-1/2 translate-y-1/2" },
	{ axis: ResizeAxis.W, class: "top-1/2 left-0 -translate-x-1/2 -translate-y-1/2" },
];

const emit =
	defineEmits<{
		(e: "resizeStart", event: ResizeEvent): void
		(e: "resizeEnd", event: ResizeEvent): void
		(e: "resizeUpdate", event: ResizeEvent): void
		(e: "dragStart", event: DragAreaEvent): void
		(e: "dragUpdate", event: DragAreaEvent): void
		(e: "dragEnd", event: DragAreaEvent): void
	}>();

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