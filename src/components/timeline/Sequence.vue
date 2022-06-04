<template>
	<div ref="el" @dragenter.prevent="onDragEnter" @dragleave="onDragLeave" @dragover.prevent="onDragOver"
		@drop="onDrop" class="h-10 p-1.5 px-2 bg-plastic-800 border-y border-t-plastic-900 border-b-plastic-600"
		:class="direction != app.currentDocument?.currentSequenceDirection ? 'rounded-md' : ''">
		<div ref="keyframesElement" class="relative h-full" :class="isDraggingContent ? 'pointer-events-none' : ''">
			<Keyframe v-for="entry in sequenceEntries" :name="entry.name" :selected="entry.selected"
				:dragged="entry.dragged" :start-time-millis="entry.startTimeMillis"
				:duration-millis="entry.durationMillis" :is-preview="entry.isPreview" :direction="direction"
				:index="entry.index" :key="entry.key" class="absolute h-full transition top-1/2 -translate-y-1/2"
				:style="entryStyle(entry)" />
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed, Ref, ref } from 'vue';
import { useAppStore } from '@/stores/app';
import { Direction, Sequence as SequenceDTO } from '@/api/dto';
import Keyframe from '@/components/timeline/Keyframe.vue';
import { dropFrameOnTimeline, dropKeyframeOnTimeline } from '@/api/document';

const app = useAppStore();

const props = defineProps<{
	sequence: SequenceDTO,
	direction: Direction
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
const receivingDragAndDrop = ref(false);
const timeHovered = ref(0);

const insertionIndex = computed(() => {
	for (let entry of sequenceEntries.value) {
		if (!entry.isPreview && (entry.startTimeMillis + entry.durationMillis / 2) >= timeHovered.value) {
			return entry.index;
		}
	}
	return props.sequence.keyframes.length;
});

const isDraggingContent = computed(() => {
	return (app.currentDocument?.framesBeingDragged.length || 0) != 0
		|| (app.currentDocument?.keyframesBeingDragged.length || 0) != 0;
});

const sequenceEntries = computed((): SequenceEntry[] => {
	let currentTime = 0;
	let previewTime = null;
	let entries: SequenceEntry[] = [];

	if (!app.currentDocument) {
		return entries;
	}

	const previewFrameDuration = 40 / app.currentDocument.timelineZoom;
	const numPreviewFrames = Math.max(app.currentDocument.framesBeingDragged.length, app.currentDocument.keyframesBeingDragged.length);

	for (let [index, keyframe] of props.sequence.keyframes.entries()) {
		if (receivingDragAndDrop.value && previewTime == null && index == insertionIndex.value) {
			previewTime = currentTime;
			currentTime += previewFrameDuration * numPreviewFrames;
		}
		const isBeingDragged = keyframe.selected && app.currentDocument.keyframesBeingDragged.length > 0;
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
	const zoom = app.currentDocument?.timelineZoom || 1;
	return {
		"transitionProperty": app.currentDocument?.isDraggingKeyframeDuration ? "none" : "width, left",
		"left": (zoom * entry.startTimeMillis) + "px",
		"width": (zoom * entry.durationMillis) + "px"
	};
}

function mouseEventToTime(event: MouseEvent) {
	if (!keyframesElement.value) {
		return 0;
	}
	const pixelDelta = event.clientX - keyframesElement.value.getBoundingClientRect().left;
	const time = pixelDelta / (app.currentDocument?.timelineZoom || 1);
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
	if ((app.currentDocument?.framesBeingDragged.length || 0) > 0) {
		dropFrameOnTimeline(props.direction, insertionIndex.value);
	} else if ((app.currentDocument?.keyframesBeingDragged.length || 0) > 0) {
		dropKeyframeOnTimeline(props.direction, insertionIndex.value);
	}
	receivingDragAndDrop.value = false;
}
</script>