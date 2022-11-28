<template>
	<Pane>
		<div class="w-full px-4 py-2 flex items-center">
			<div class="flex-1 flex space-x-2">
				<PerspectivePicker />
				<Toggle :toggled="!!app.currentAnimation?.isLooping" @toggled="setAnimationLooping"
					:icon="ArrowPathIcon" />
			</div>
			<div class="flex-1 flex justify-center">
				<PlaybackControls />
			</div>
			<div class="flex-1 flex space-x-4 justify-end items-center">
				<Toggle :toggled="!!app.currentDocument?.snapKeyframeDurations" @toggled="setSnapKeyframeDurations"
					:icon="AdjustmentsHorizontalIcon" />
				<div class="flex items-center space-x-2">
					<MagnifyingGlassIcon class="w-5 text-plastic-400" />
					<Slider class="w-28" v-model:value="zoomAmount" v-model:dragging="draggingScale" />
				</div>
			</div>
		</div>
		<PaneInset class="flex-1 m-4 mt-0 ">
			<div class="relative flex h-full bg-plastic-700">
				<div class="flex flex-col">
					<div class="h-6 bg-plastic-600" />
					<div
						class="w-36 flex flex-col py-2 space-y-1 text-plastic-400 text-xs uppercase font-semibold text-right">
						<div v-for="entry in sequenceEntries" @click="selectDirection(entry.direction)"
							class="h-10 ml-4 px-4 inline-flex items-center justify-end cursor-pointer" :class="entry.sequence == app.currentSequence ?
							'text-plastic-200 bg-plastic-800 rounded-l-md border-y border-t-plastic-900 border-b-plastic-600' : ''">
							{{ entry.direction }}
						</div>
					</div>
				</div>
				<div ref="scrollableElement" @scroll="onScroll"
					class="flex-1 relative overflow-x-scroll styled-scrollbars">
					<div class="min-w-full flex flex-col" :style="timelineStyle">
						<Ruler v-model:scrubbing="scrubbing" :animate="animateRuler" />
						<div class="flex flex-col py-2 space-y-1">
							<Sequence v-for="entry in sequenceEntries" :sequence="entry.sequence"
								:direction="entry.direction" :animate="animateSequences" />
							<div v-for="_ in Math.max(0, (4 - Object.keys(app.currentAnimation?.sequences || []).length))"
								class="h-10" />
						</div>
						<div class="absolute top-0 mx-2 h-full w-px bg-white transition pointer-events-none"
							:style="playheadStyle" />
					</div>
				</div>
			</div>
		</PaneInset>
	</Pane>
</template>

<script setup lang="ts">
import { computed, Ref, ref } from "@vue/reactivity"
import { AdjustmentsHorizontalIcon, ArrowPathIcon,ChevronDownIcon, MagnifyingGlassIcon } from "@heroicons/vue/20/solid"
import { Direction, Sequence as SequenceDTO } from "@/api/dto"
import {
	selectDirection, setAnimationLooping, setSnapKeyframeDurations, setTimelineZoomAmount
} from "@/api/document"
import { useAppStore } from "@/stores/app"
import { debounceAnimation } from "@/utils/animation"
import Slider from "@/components/basic/Slider.vue"
import Pane from "@/components/basic/Pane.vue"
import PaneInset from "@/components/basic/PaneInset.vue"
import Toggle from "@/components/basic/Toggle.vue"
import PerspectivePicker from "@/components/timeline/PerspectivePicker.vue"
import PlaybackControls from "@/components/timeline/PlaybackControls.vue"
import Ruler from "@/components/timeline/Ruler.vue"
import Sequence from "@/components/timeline/Sequence.vue"

const app = useAppStore();

const scrollableElement: Ref<HTMLElement | null> = ref(null);
const scrubbing = ref(false);
const draggingScale = ref(false);
const scrollLeft = ref(0);

const zoomAmount = computed({
	get: () => app.currentDocument ? app.currentDocument?.timelineZoomAmount : 0.5,
	set: setTimelineZoomAmount,
});

function onScroll() {
	scrollLeft.value = scrollableElement.value?.scrollLeft || 0;
}

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

const timelineSize = computed(() => {
	const zoom = app.currentDocument?.timelineZoomFactor || 1;
	const visibleSize = scrollLeft.value + (scrollableElement.value?.clientWidth || 0);
	const visibleDuration = visibleSize / zoom;
	const bonusDuration = 500 / zoom;
	return zoom * Math.max(visibleDuration, animationDuration.value + bonusDuration);
});

const timelineStyle = computed(() => {
	return {
		width: `${timelineSize.value}px`
	}
});

const animateRuler = debounceAnimation(
	[draggingScale],
	() => !draggingScale.value
);

const animateSequences = debounceAnimation(
	[() => app.currentDocument?.isDraggingKeyframeDuration, draggingScale],
	() => !app.currentDocument?.isDraggingKeyframeDuration && !draggingScale.value
);

const animatePlayhead = debounceAnimation(
	[() => app.currentDocument?.timelineIsPlaying, scrubbing, draggingScale],
	() => !app.currentDocument?.timelineIsPlaying && !scrubbing.value && !draggingScale.value
);

const playheadStyle = computed(() => {
	const zoom = app.currentDocument?.timelineZoomFactor || 1;
	const time = app.currentDocument?.timelineClockMillis || 0;
	return {
		transitionProperty: animatePlayhead.value ? "left" : "none",
		left: `${Math.floor(zoom * time)}px`,
	};
});

</script>
