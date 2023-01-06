<template>
	<TooltipArea :text="frame.path">
		<div @click.stop="onFrameClicked" @contextmenu.stop.prevent="onOpenContextMenu" @dragstart="onDragStart"
			@dragend="onDragEnd" draggable="true">
			<Selectable v-if="compact" :left-icon="frame.missingOnDisk ? ExclamationTriangleIcon : PhotoIcon"
				:text="frame.name" :selected="frame.selected"
				:actions="[{ icon: XMarkIcon, callback: onDeleteClicked }]" />
			<Thumbnail v-else :path="frame.path" class="cursor-pointer outline-offset-2"
				:class="frame.selected ? 'outline outline-blue-600' : 'hover:outline outline-plastic-500'" />
		</div>
		<ContextMenu ref="contextMenu" :content="contextMenuEntries" />
	</TooltipArea>
</template>

<script setup lang="ts">
import { Ref, ref } from "vue"
import { PhotoIcon, XMarkIcon } from "@heroicons/vue/20/solid"
import { ExclamationTriangleIcon } from "@heroicons/vue/24/solid"
import { Frame as FrameDTO } from "@/api/dto"
import { useSpriteStore } from "@/stores/sprite"
import { revealInExplorer } from "@/api/app"
import { beginDragAndDropFrame, endDragAndDropFrame, selectFrame, deleteSelectedFrames, deleteFrame } from "@/api/document"
import ContextMenu from "@/components/basic/ContextMenu.vue"
import Selectable from "@/components/basic/Selectable.vue"
import TooltipArea from "@/components/basic/TooltipArea.vue"
import Thumbnail from "@/components/frames/Thumbnail.vue"

const sprite = useSpriteStore();

const props = defineProps<{
	frame: FrameDTO,
	compact: boolean,
}>();

defineExpose({
	getFrame: () => props.frame
});

const contextMenu: Ref<typeof ContextMenu | null> = ref(null);
const dragCursorElement: Ref<HTMLElement | null> = ref(null);

const contextMenuEntries = [
	{ name: "Delete", shortcut: "Del", action: deleteSelectedFrames },
	{ name: "Reveal in Explorer", action: () => revealInExplorer(props.frame.path) },
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
