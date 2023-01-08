<template>
	<div class="pr-1" @contextmenu.prevent="onOpenContextMenu">
		<div ref="el" @dragstart="onDragStart" @dragend="onDragEnd" draggable="true"
			class="h-full min-w-0 relative rounded-md border-2 cursor-pointer" :class="dynamicClasses">
			<div @click="onKeyframeClicked" class="h-full px-2 flex items-center font-semibold text-[11px]">
				<div class="min-w-0 overflow-hidden whitespace-nowrap text-ellipsis">{{ name }}</div>
			</div>
		</div>
		<DragArea v-if="!isPreview" @drag-start="beginDurationDrag" @drag-update="updateDurationDrag"
			@drag-end="endDurationDrag" inactive-cursor="cursor-ew-resize" active-cursor="cursor-ew-resize"
			class="absolute top-0 right-[-4px] h-full w-[16px]" />
		<ContextMenu ref="contextMenu" :content="contextMenuEntries" />
	</div>
</template>

<script setup lang="ts">
import { computed, Ref, ref } from "vue"
import { Direction } from "@/api/dto"
import { useStateStore } from "@/stores/state"
import { beginDragAndDropKeyframe, updateDragKeyframeDuration, selectKeyframe, endDragKeyframeDuration, beginDragKeyframeDuration, endDragAndDropKeyframe, deleteSelectedKeyframes, copy, cut } from "@/api/document"
import ContextMenu from "@/components/basic/ContextMenu.vue"
import DragArea, { DragAreaEvent } from "@/components/basic/DragArea.vue"

const state = useStateStore();

const props = defineProps<{
	name: string,
	selected: boolean,
	dragged: boolean,
	startTimeMillis: number,
	durationMillis: number,
	direction: Direction,
	index: number,
	isPreview: boolean,
}>();

const contextMenu: Ref<typeof ContextMenu | null> = ref(null);

const contextMenuEntries = [
	{ name: "Cut", shortcut: "Ctrl+X", action: cut },
	{ name: "Copy", shortcut: "Ctrl+C", action: copy },
	{},
	{ name: "Delete", shortcut: "Del", action: deleteSelectedKeyframes },
];

const dynamicClasses = computed(() => {
	if (props.isPreview) {
		return ["bg-blue-600", "border-blue-600", "animate-pulse"];
	}
	if (props.selected) {
		return [
			"text-blue-100", "bg-zinc-900", "border-blue-600",
			...(props.dragged ? ["border-dotted", "animate-pulse"] : [])
		];
	}
	if (props.direction == state.currentDocument?.currentSequenceDirection) {
		return ["text-orange-200", "bg-plastic-900", "border-orange-600"];
	}
	return ["text-plastic-500", "bg-plastic-900", "border-plastic-500"];
});

const el: Ref<HTMLElement | null> = ref(null);
const dragCursorElement: Ref<HTMLElement | null> = ref(null);
let durationDragReferenceTime = 0;

function mouseEventToTime(event: MouseEvent) {
	if (!el.value) {
		return 0;
	}
	const pixelDelta = event.clientX - el.value.getBoundingClientRect().left;
	const durationDelta = pixelDelta / (state.currentDocument?.timelineZoomFactor || 1);
	return props.startTimeMillis + durationDelta;
}

function onKeyframeClicked(event: MouseEvent) {
	selectKeyframe(props.direction, props.index, event.shiftKey, event.ctrlKey);
}

function onOpenContextMenu(event: MouseEvent) {
	if (contextMenu.value) {
		if (!props.selected) {
			selectKeyframe(props.direction, props.index, false, false);
		}
		contextMenu.value.show(event);
	}
}

function onDragStart(event: DragEvent) {
	if (event.dataTransfer) {
		const previewElement = document.createElement("div");
		document.body.appendChild(previewElement);
		previewElement.style.position = "absolute";
		previewElement.style.top = "-1000px";
		previewElement.classList.add("opacity-0");
		previewElement.innerText = "N/A";
		dragCursorElement.value = previewElement;
		event.dataTransfer.setDragImage(previewElement, 0, 0);
	}
	beginDragAndDropKeyframe(props.direction, props.index);
}

function onDragEnd() {
	endDragAndDropKeyframe();
	if (dragCursorElement.value) {
		document.body.removeChild(dragCursorElement.value);
		dragCursorElement.value = null;
	}
}

function beginDurationDrag(e: DragAreaEvent) {
	beginDragKeyframeDuration(props.direction, props.index);
	durationDragReferenceTime = mouseEventToTime(e.mouseEvent);
}

function updateDurationDrag(e: DragAreaEvent) {
	const deltaMillis = mouseEventToTime(e.mouseEvent) - durationDragReferenceTime;
	updateDragKeyframeDuration(Math.round(deltaMillis));
}

function endDurationDrag(e: DragAreaEvent) {
	endDragKeyframeDuration();
}
</script>
