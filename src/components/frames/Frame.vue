<template>
	<div @click="(event) => onFrameClicked(event)" @contextmenu.prevent="onOpenContextMenu" @dragstart="onDragStart"
		@dragend="onDragEnd" draggable="true">
		<Selectable v-if="compact" left-icon="PhotoIcon" :text="frame.name" :selected="frame.selected"
			:actions="[{ icon: 'XMarkIcon', callback: onDeleteClicked }]" />
		<div v-else
			class="aspect-square checkerboard flex place-content-center rounded-sm cursor-pointer overflow-hidden outline-offset-2"
			:class="frame.selected ? 'outline outline-blue-600' : 'hover:outline outline-plastic-500'">
			<img :src="sprite.getURL(frame.path)" class="pixelated object-none" />
		</div>
		<ContextMenu ref="contextMenu" :content="contextMenuEntries" />
	</div>
</template>

<script setup lang="ts">
import { Ref, ref } from "vue"
import { Frame as FrameDTO } from "@/api/dto"
import { useSpriteStore } from "@/stores/sprite"
import { beginDragAndDropFrame, endDragAndDropFrame, selectFrame, deleteSelectedFrames, deleteFrame } from "@/api/document"
import ContextMenu from "@/components/basic/ContextMenu.vue"
import Selectable from "@/components/basic/Selectable.vue"

const sprite = useSpriteStore();

const props = defineProps<{
	frame: FrameDTO,
	compact: boolean,
}>();

const contextMenu: Ref<typeof ContextMenu | null> = ref(null);
const dragCursorElement: Ref<HTMLElement | null> = ref(null);

const contextMenuEntries = [
	{ name: "Delete", shortcut: "Del", action: deleteSelectedFrames },
];

function onOpenContextMenu(event: MouseEvent) {
	if (contextMenu.value) {
		if (!props.frame.selected) {
			selectFrame(props.frame.path, event.shiftKey, event.ctrlKey);
		}
		contextMenu.value.show(event);
	}
}

function onDragStart(event: DragEvent) {
	if (event.dataTransfer) {
		const previewElement = document.createElement("img");
		document.body.appendChild(previewElement);
		previewElement.style.position = "absolute";
		previewElement.style.top = "-1000px";
		previewElement.classList.add("opacity-0");
		previewElement.src = sprite.getURL(props.frame.path);
		dragCursorElement.value = previewElement;
		event.dataTransfer.setDragImage(previewElement, 0, 0);
	}
	beginDragAndDropFrame(props.frame.path);
}

function onDragEnd() {
	endDragAndDropFrame();
	if (dragCursorElement.value) {
		document.body.removeChild(dragCursorElement.value);
		dragCursorElement.value = null;
	}
}

function onDeleteClicked() {
	deleteFrame(props.frame.path);
}

function onFrameClicked(event: MouseEvent) {
	selectFrame(props.frame.path, event.shiftKey, event.ctrlKey)
}
</script>


<style>
.checkerboard {
	background-size: 16px 16px;
	background-image:
		linear-gradient(45deg, theme("colors.plastic.700") 25%, transparent 25%, transparent 75%, theme("colors.plastic.700") 75%, theme("colors.plastic.700") 100%),
		linear-gradient(45deg, theme("colors.plastic.700") 25%, theme("colors.plastic.600") 25%, theme("colors.plastic.600") 75%, theme("colors.plastic.700") 75%, theme("colors.plastic.700") 100%);
	background-position:
		0px 0px,
		8px 8px;
}
</style>