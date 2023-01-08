<template>
	<div ref="el" @dragenter.prevent="onDragEnter" @dragleave="onDragLeave" @dragover.prevent="onDragOver"
		@drop="onDrop"
		class="flex items-center h-10 px-2 bg-plastic-800 border-y border-t-plastic-900 border-b-plastic-600"
		:class="direction != state.currentDocument?.currentSequenceDirection ? 'rounded-md' : ''">
		<div ref="keyframesElement" class="relative h-7" :style="sequenceWidth"
			:class="isDraggingContent ? 'pointer-events-none' : ''">
			<Keyframe v-for="entry in sequenceEntries" :name="entry.name" :selected="entry.selected"
				:dragged="entry.dragged" :start-time-millis="entry.startTimeMillis"
				:duration-millis="entry.durationMillis" :is-preview="entry.isPreview" :direction="direction"
				:index="entry.index" :key="entry.key" class="absolute h-full transition top-1/2 -translate-y-1/2"
				:style="entryStyle(entry)" />
		</div>
		<div class="flex-grow h-full" :class="isDraggingContent ? 'pointer-events-none' : ''" @click="onDeadZoneClicked"
			@dblclick="jumpToAnimationEnd" @contextmenu.stop.prevent="onOpenContextMenu" />
		<ContextMenu ref="contextMenu" :content="contextMenuEntries" />
	</div>
</template>

<script setup lang="ts">
import { computed, Ref, ref } from "vue"
import { dropFrameOnTimeline, dropKeyframeOnTimeline, jumpToAnimationEnd, paste, selectDirection } from "@/backend/api"
import { ClipboardManifest, Direction, Sequence as SequenceDTO } from "@/backend/dto"
import { useStateStore } from "@/stores/state"
import ContextMenu from "@/components/basic/ContextMenu.vue"
import Keyframe from "@/components/timeline/Keyframe.vue"

const state = useStateStore();

const props = defineProps<{
	sequence: SequenceDTO,
	direction: Direction,
	animate: boolean,
}>();

type SequenceEntry = {
	name: string,
	selected: boolean,
	dragged: boolean,
	startTimeMillis: number,
	durationMillis: number,
	isPreview: boolean,
	index: number,
	key: string,
};

const el: Ref<HTMLElement | null> = ref(null);
const keyframesElement: Ref<HTMLElement | null> = ref(null);
const contextMenu: Ref<typeof ContextMenu | null> = ref(null);
const receivingDragAndDrop = ref(false);
const timeHovered = ref(0);

const contextMenuEntries = computed(() => [
	{ name: "Paste", shortcut: "Ctrl+V", action: paste, disabled: state.clipboardManifest != ClipboardManifest.Keyframes },
]);

const insertionIndex = computed(() => {
	for (let entry of sequenceEntries.value) {
		if (!entry.isPreview && (entry.startTimeMillis + entry.durationMillis / 2) >= timeHovered.value) {
			return entry.index;
		}
	}
	return props.sequence.keyframes.length;
});

const isDraggingContent = computed(() => {
	return (state.currentDocument?.framesBeingDragged.length || 0) != 0
		|| (state.currentDocument?.keyframesBeingDragged.length || 0) != 0;
});

const sequenceEntries = computed((): SequenceEntry[] => {
	let currentTime = 0;
	let previewTime = null;
	let entries: SequenceEntry[] = [];

	if (!state.currentDocument) {
		return entries;
	}

	const previewFrameDuration = 40 / state.currentDocument.timelineZoomFactor;
	const numPreviewFrames = Math.max(state.currentDocument.framesBeingDragged.length, state.currentDocument.keyframesBeingDragged.length);

	for (let [index, keyframe] of props.sequence.keyframes.entries()) {
		if (receivingDragAndDrop.value && previewTime == null && index == insertionIndex.value) {
			previewTime = currentTime;
			currentTime += previewFrameDuration * numPreviewFrames;
		}
		const isBeingDragged = keyframe.selected && state.currentDocument.keyframesBeingDragged.length > 0;
		entries.push({
			name: keyframe.name,
			selected: keyframe.selected,
			dragged: isBeingDragged,
			startTimeMillis: currentTime,
			durationMillis: keyframe.durationMillis,
			isPreview: false,
			index: index,
			key: keyframe.key,
		});
		currentTime += keyframe.durationMillis;
	}

	if (receivingDragAndDrop.value) {
		if (previewTime != null) {
			currentTime = previewTime;
		}
		for (let index = 0; index < numPreviewFrames; index++) {
			entries.push({
				name: "",
				selected: false,
				dragged: false,
				startTimeMillis: currentTime,
				durationMillis: previewFrameDuration,
				isPreview: true,
				index: 0,
				key: "preview_" + index,
			});
			currentTime += previewFrameDuration;
		}
	}

	return entries;
});

function entryStyle(entry: SequenceEntry) {
	const zoom = state.currentDocument?.timelineZoomFactor || 1;
	return {
		transitionProperty: props.animate ? "width, left" : "none",
		left: `${zoom * entry.startTimeMillis}px`,
		width: `${zoom * entry.durationMillis}px`
	};
}

const sequenceWidth = computed(() => {
	const zoom = state.currentDocument?.timelineZoomFactor || 1;
	return {
		width: `${zoom * (props.sequence.durationMillis || 0)}px`
	};
});

function mouseEventToTime(event: MouseEvent) {
	if (!keyframesElement.value) {
		return 0;
	}
	const pixelDelta = event.clientX - keyframesElement.value.getBoundingClientRect().left;
	const time = pixelDelta / (state.currentDocument?.timelineZoomFactor || 1);
	return time;
}

function onDragEnter(event: DragEvent) {
	if (event.target != el.value) {
		return;
	}
	receivingDragAndDrop.value = true;
	timeHovered.value = mouseEventToTime(event);
}

function onDragLeave(event: DragEvent) {
	if (event.target != el.value) {
		return;
	}
	receivingDragAndDrop.value = false;
}

function onDragOver(event: DragEvent) {
	timeHovered.value = mouseEventToTime(event);
}

function onDrop() {
	if ((state.currentDocument?.framesBeingDragged.length || 0) > 0) {
		dropFrameOnTimeline(props.direction, insertionIndex.value);
	} else if ((state.currentDocument?.keyframesBeingDragged.length || 0) > 0) {
		dropKeyframeOnTimeline(props.direction, insertionIndex.value);
	}
	receivingDragAndDrop.value = false;
}

function onDeadZoneClicked() {
	selectDirection(props.direction);
}

function onOpenContextMenu(event: MouseEvent) {
	selectDirection(props.direction);
	if (contextMenu.value) {
		contextMenu.value.show(event);
	}
}
</script>
