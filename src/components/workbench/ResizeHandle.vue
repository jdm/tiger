<template>
	<div class="absolute -translate-x-1/2 -translate-y-1/2">
		<svg viewBox="0 0 100 100" width="30" height="30">
			<circle cx="50" cy="50" r="24" class="fill-white stroke-blue-600" stroke-width="5" />
			<circle cx="50" cy="50" r="16" class="fill-blue-600" />
		</svg>
		<DragArea :buttons="['left', 'right']" class="absolute pointer-events-auto inset-0" @drag-start="onDragStart"
			@drag-update="onDragUpdate" @drag-end="onDragEnd" :active-cursor="cursor" :inactive-cursor="cursor" />
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { ResizeAxis } from "@/backend/dto";
import DragArea, { Cursor, DragAreaEvent } from "@/components/basic/DragArea.vue";
import { ResizeEvent } from "@/components/workbench/ResizeArea.vue";

const props = defineProps<{
	axis: ResizeAxis,
}>();

const emit =
	defineEmits<{
		(e: "resizeStart", event: ResizeEvent): void
		(e: "resizeEnd", event: ResizeEvent): void
		(e: "resizeUpdate", event: ResizeEvent): void
		(e: "dragStart", event: DragAreaEvent): void
		(e: "dragEnd", event: DragAreaEvent): void
		(e: "dragUpdate", event: DragAreaEvent): void
	}>();

const cursor = computed((): Cursor => {
	switch (props.axis) {
		case ResizeAxis.NW: return "cursor-nwse-resize";
		case ResizeAxis.N: return "cursor-ns-resize";
		case ResizeAxis.NE: return "cursor-nesw-resize";
		case ResizeAxis.E: return "cursor-ew-resize";
		case ResizeAxis.SE: return "cursor-nwse-resize";
		case ResizeAxis.S: return "cursor-ns-resize";
		case ResizeAxis.SW: return "cursor-nesw-resize";
		case ResizeAxis.W: return "cursor-ew-resize";
		default: return "cursor-ew-resize";
	}
});

function onDragStart(event: DragAreaEvent) {
	if (event.button == "left") {
		emit("resizeStart", { axis: props.axis, dragEvent: event });
	} else {
		emit("dragStart", event);
	}
}

function onDragUpdate(event: DragAreaEvent) {
	if (event.button == "left") {
		emit("resizeUpdate", { axis: props.axis, dragEvent: event });
	} else {
		emit("dragUpdate", event);
	}
}

function onDragEnd(event: DragAreaEvent) {
	if (event.button == "left") {
		emit("resizeEnd", { axis: props.axis, dragEvent: event });
	} else {
		emit("dragEnd", event);
	}
}

</script>