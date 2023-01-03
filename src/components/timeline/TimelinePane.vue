<template>
	<Pane>
		<div class="w-full px-4 py-2 flex items-center">
			<div class="flex-1 flex gap-2">
				<PerspectivePicker />
				<TooltipArea text="Toggle animation looping">
					<Toggle :toggled="!!app.currentAnimation?.isLooping" @toggled="setAnimationLooping"
						:icon="ArrowPathIcon" />
				</TooltipArea>
			</div>
			<div class="flex-1 flex justify-center">
				<PlaybackControls />
			</div>
			<div class="flex-1 flex gap-2 justify-end items-center">
				<TooltipArea text="Keyframe snapping">
					<Toggle :toggled="!!app.currentDocument?.snapKeyframeDurations" @toggled="setSnapKeyframeDurations"
						:icon="AdjustmentsHorizontalIcon" :canExpand="true">
						<template #expanded>
							<SnappingOptions />
						</template>
					</Toggle>
				</TooltipArea>
				<MagnifyingGlassIcon class="w-5 text-plastic-400" />
				<Slider class="w-28" v-model:value="zoomAmount" v-model:dragging="draggingScale" />
			</div>
		</div>
		<PaneInset class="flex-1 m-4 mt-0 ">
			<div class="flex items-stretch bg-plastic-700">
				<div class="flex flex-col">
					<div class="h-6 bg-plastic-600" />
					<div
						class="w-36 flex flex-col py-2 gap-1 text-plastic-400 text-xs uppercase font-semibold text-right">
						<div v-for="entry in sequenceEntries" @click="selectDirection(entry.direction)"
							class="h-10 ml-4 px-4 inline-flex items-center justify-end cursor-pointer" :class="entry.sequence == app.currentSequence ?
							'text-plastic-200 bg-plastic-800 rounded-l-md border-y border-t-plastic-900 border-b-plastic-600' : ''">
							{{ entry.direction }}
						</div>
					</div>
				</div>
				<div class="flex-grow relative min-h-[212px]">
					<div ref="scrollableElement" class="absolute min-w-full h-full overflow-clip flex flex-col"
						@wheel="onMouseWheel">
						<DragArea :buttons="['right']" activeCursor="cursor-move" @drag-update="updatePanning"
							@dragStart="onDragStart" @dragEnd="onDragEnd">
							<div class="absolute top-0 transition" :style="timelineStyle">
								<Ruler v-model:scrubbing="scrubbing" :animate="animateRuler" />
								<div class="flex flex-col py-2 gap-1">
									<Sequence v-for="entry in sequenceEntries" :sequence="entry.sequence"
										:direction="entry.direction" :animate="animateSequences" />
								</div>
							</div>
						</DragArea>
						<div class="absolute top-0 mx-2 h-full w-px bg-white transition pointer-events-none"
							:style="playheadStyle" />
					</div>
				</div>
			</div>
		</PaneInset>
	</Pane>
</template>

<script setup lang="ts">
import { computed, Ref, ref, watch } from "vue"
import { AdjustmentsHorizontalIcon, ArrowPathIcon, MagnifyingGlassIcon } from "@heroicons/vue/20/solid"
import { Direction, Sequence as SequenceDTO } from "@/api/dto"
import {
panTimeline,
	selectDirection, setAnimationLooping, setSnapKeyframeDurations, setTimelineOffset, setTimelineZoomAmount, zoomInTimeline, zoomInTimelineAround, zoomOutTimeline, zoomOutTimelineAround
} from "@/api/document"
import { useAppStore } from "@/stores/app"
import { debounceAnimation, isStable } from "@/utils/animation"
import DragArea, { DragAreaEvent } from "@/components/basic/DragArea.vue"
import Slider from "@/components/basic/Slider.vue"
import Pane from "@/components/basic/Pane.vue"
import PaneInset from "@/components/basic/PaneInset.vue"
import Toggle from "@/components/basic/Toggle.vue"
import TooltipArea from "@/components/basic/TooltipArea.vue"
import PerspectivePicker from "@/components/timeline/PerspectivePicker.vue"
import PlaybackControls from "@/components/timeline/PlaybackControls.vue"
import Ruler from "@/components/timeline/Ruler.vue"
import Sequence from "@/components/timeline/Sequence.vue"
import SnappingOptions from "@/components/timeline/SnappingOptions.vue"

const app = useAppStore();

const scrollableElement: Ref<HTMLElement | null> = ref(null);
const scrollableElementWidth = ref(0);
const scrubbing = ref(false);
const panning = ref(false);
const draggingScale = ref(false);
const isWindowSizeStable = isStable([scrollableElementWidth]);
const offset = computed(() => app.currentDocument?.timelineOffsetMillis || 0);
const isPlaying = computed(() => app.currentDocument?.timelineIsPlaying);
const zoomAmount = computed({
	get: () => app.currentDocument ? app.currentDocument?.timelineZoomAmount : 0.5,
	set: setTimelineZoomAmount,
});
const zoomFactor = computed(() => app.currentDocument?.timelineZoomFactor || 1);
const isZoomStable = isStable([zoomFactor]);

type SequenceEntry = {
	direction: Direction,
	sequence: SequenceDTO,
};

const sequenceEntries = computed(() => {
	if (!app.currentAnimation) {
		return null;
	}
	const orderedDirections = Object.values(Direction);
	const entries = Object.entries(app.currentAnimation?.sequences).map(([direction, sequence]): SequenceEntry => {
		return { direction: direction as Direction, sequence: sequence }
	});
	entries.sort((entryA, entryB) =>
		orderedDirections.indexOf(entryA.direction) - orderedDirections.indexOf(entryB.direction)
	);
	return entries;
});

const animationDuration = computed(() => {
	return Math.max(...Object.values(app.currentAnimation?.sequences || {}).map(s => s.durationMillis || 0)) || 0;
});

const resizeObserver = new ResizeObserver(entries => {
	for (let entry of entries) {
		if (entry.target === scrollableElement.value) {
			scrollableElementWidth.value = entry.contentRect.width;
		}
	}
});

watch(scrollableElement, (newScrollable, oldScrollable) => {
	if (oldScrollable) {
		resizeObserver.unobserve(oldScrollable);
	}
	if (newScrollable) {
		resizeObserver.observe(newScrollable);
	}
});

const timelineSize = computed(() => {
	const visibleDuration = offset.value + scrollableElementWidth.value / zoomFactor.value;
	const bonusDuration = 500 / zoomFactor.value;
	return zoomFactor.value * Math.max(visibleDuration, animationDuration.value + bonusDuration);
});

const scrollingCanAnimate = debounceAnimation(
	[draggingScale, panning, isPlaying],
	() => !draggingScale.value && !panning.value && !isPlaying.value,
	50
);
const animateScrolling = computed(() => isWindowSizeStable.value && (scrollingCanAnimate.value || !isZoomStable.value));

const timelineStyle = computed(() => {
	return {
		width: `${timelineSize.value}px`,
		left: `-${Math.round(offset.value * zoomFactor.value)}px`,
		transitionProperty: animateScrolling.value ? "left, width" : "none",
	}
});

const animateRuler = debounceAnimation(
	[draggingScale],
	() => !draggingScale.value,
	50
);

const animateSequences = debounceAnimation(
	[() => app.currentDocument?.isDraggingKeyframeDuration, draggingScale],
	() => !app.currentDocument?.isDraggingKeyframeDuration && !draggingScale.value,
	50
);

const animatePlayhead = debounceAnimation(
	[ isPlaying
	, () => app.currentDocument?.isDraggingKeyframeDuration
	, scrubbing
	, panning
	, draggingScale
	],
	() => !isPlaying.value
	&& !app.currentDocument?.isDraggingKeyframeDuration
	&& !scrubbing.value
	&& !panning.value
	&& !draggingScale.value,
	50
);

const playheadStyle = computed(() => {
	const time = app.currentDocument?.timelineClockMillis || 0;
	return {
		transitionProperty: animatePlayhead.value ? "left" : "none",
		left: `${Math.round(zoomFactor.value * (time - offset.value))}px`,
	};
});

watch(() => app.currentDocument?.timelineClockMillis || 0, (clock) => {
	if (!scrollableElement.value) {
		return;
	}
	const boundingBox = scrollableElement.value.getBoundingClientRect();
	const minVisible = offset.value;
	const maxVisible = offset.value + (boundingBox.right - boundingBox.left) / zoomFactor.value;
	if (clock < minVisible || clock > maxVisible) {
		const padding = isPlaying.value ? 0 : -100 / zoomFactor.value;
		setTimelineOffset(clock + padding);
	}
});

function onMouseWheel(event: WheelEvent) {
	if (event.ctrlKey) {
		if (!scrollableElement.value) {
			return;
		}
		if (isPlaying.value) {
			if (event.deltaY < 0) {
				zoomInTimeline();
			} else {
				zoomOutTimeline();
			}
		} else {
			const boundingBox = scrollableElement.value.getBoundingClientRect();
			// 8px offset is for the padding between start of scrollable element
			// and beginning of timeline content
			const cursorTime = (event.clientX - 8 - boundingBox.left) / zoomFactor.value + offset.value;
			if (event.deltaY < 0) {
				zoomInTimelineAround(cursorTime);
			} else {
				zoomOutTimelineAround(cursorTime);
			}
		}
	} else {
		panTimeline(-event.deltaY);
	}
}

function onDragStart(event: DragAreaEvent) {
	panning.value = true;
}

function onDragEnd(event: DragAreaEvent) {
	panning.value = false;
}

function updatePanning(event: DragAreaEvent) {
	panTimeline(event.mouseEvent.movementX);
}

</script>
