<template>
	<div @dragenter.prevent="onDragEnter" @dragleave="onDragLeave" @dragover.prevent="onDragOver" @drop="onDrop"
		class="h-9 p-1 rounded-sm bg-plastic-800 border-y border-t-plastic-900 border-b-plastic-600">
		<div ref="el" class="relative h-full" :class="isDraggingContent ? 'pointer-events-none' : ''">
			<Keyframe v-for="entry in sequenceEntries" :name="entry.name" :selected="entry.selected"
				:start-time-millis="entry.startTimeMillis" :duration-millis="entry.durationMillis"
				:is-preview="entry.isPreview" :direction="direction" :index="entry.index" :key="entry.key"
				class="absolute h-full transition top-1/2 -translate-y-1/2" :style="entryStyle(entry)" />
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed, Ref, ref } from 'vue';
import { useAppStore } from '@/stores/app';
import { Direction, Sequence as SequenceDTO } from '@/api/dto';
import Keyframe from '@/components/timeline/Keyframe.vue';
import { dropFrameOnTimeline } from '@/api/document';

const app = useAppStore();

const props = defineProps<{
	sequence: SequenceDTO,
	direction: Direction
}>();

type SequenceEntry = {
	name: string,
	selected: boolean,
	startTimeMillis: number,
	durationMillis: number,
	isPreview: boolean,
	index: number,
	key: string,
};

const el: Ref<HTMLElement | null> = ref(null);
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
	return (app.currentDocument?.framesBeingDragged.length || 0) > 0;
});

const sequenceEntries = computed((): SequenceEntry[] => {
	const previewFrameDuration = 100;
	let currentTime = 0;
	let previewTime = null;
	let entries: SequenceEntry[] = [];

	if (!app.currentDocument) {
		return entries;
	}

	for (let [index, keyframe] of props.sequence.keyframes.entries()) {
		if (receivingDragAndDrop.value && !previewTime && index == insertionIndex.value) {
			previewTime = currentTime;
			currentTime += previewFrameDuration * app.currentDocument.framesBeingDragged.length;
		}
		entries.push({
			name: keyframe.name,
			selected: keyframe.selected,
			startTimeMillis: currentTime,
			durationMillis: keyframe.durationMillis,
			isPreview: false,
			index: index,
			key: keyframe.key,
		});
		currentTime += keyframe.durationMillis;
	}

	if (receivingDragAndDrop.value) {
		if (previewTime) {
			currentTime = previewTime;
		}
		for (let [index, frame] of app.currentDocument.framesBeingDragged.entries()) {
			entries.push({
				name: frame,
				selected: false,
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
		"transitionProperty": (entry.isPreview || app.currentDocument?.isDraggingKeyframeDuration) ? "none" : "width, left",
		"left": (zoom * entry.startTimeMillis) + "px",
		"width": (zoom * entry.durationMillis) + "px"
	};
}

function mouseEventToTime(event: MouseEvent) {
	if (!el.value) {
		return 0;
	}
	const pixelDelta = event.clientX - el.value.getBoundingClientRect().left;
	const time = pixelDelta / (app.currentDocument?.timelineZoom || 1);
	return time;
}

function onDragEnter() {
	receivingDragAndDrop.value = true;
}

function onDragLeave() {
	receivingDragAndDrop.value = false;
}

function onDragOver(event: DragEvent) {
	timeHovered.value = mouseEventToTime(event);
}

function onDrop() {
	dropFrameOnTimeline(props.direction, insertionIndex.value);
	receivingDragAndDrop.value = false;
}
</script>