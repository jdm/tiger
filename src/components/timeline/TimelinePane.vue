<template>
	<Pane>
		<div class="w-full px-4 p-2 pb-0 flex flex-row items-center">
			<div class="flex-1 flex flex-row space-x-2">
				<PerspectivePicker />
				<Toggle :toggled="app.currentAnimation?.isLooping || false" icon="RefreshIcon"
					@toggled="setAnimationLooping" />
			</div>
			<div class="flex-1 flex flex-row justify-center">
				<PlaybackControls />
			</div>
			<div class="flex-1 flex flex-row justify-end items-center">
				<div class="flex flex-row items-center space-x-3">
					<Zoom class="h-6 w-5 text-plastic-400" />
					<Slider class="w-28" v-model:value="zoomAmount" v-model:dragging="draggingScale" />
				</div>
			</div>
		</div>
		<PaneInset class="flex-1 m-4 mt-2 ">
			<div class="relative flex flex-row h-full bg-plastic-700">
				<div class="flex flex-col">
					<div class="h-6 bg-plastic-600" />
					<div
						class="w-36 flex flex-col py-2 space-y-1 text-plastic-400 text-xs uppercase font-semibold text-right">
						<div v-for="sequence, direction in app.currentAnimation?.sequences"
							@click="selectDirection(direction)"
							class="h-10 ml-4 px-4 inline-flex items-center justify-end cursor-pointer" :class="sequence == app.currentSequence ?
							'text-plastic-200 bg-plastic-800 rounded-l-md border-y border-t-plastic-900 border-b-plastic-600' : ''">
							{{ direction }}
						</div>
					</div>
				</div>
				<div ref="scrollableElement" @scroll="onScroll"
					class="flex-1 relative overflow-x-scroll styled-scrollbars">
					<div class="min-w-full flex flex-col" :style="timelineStyle">
						<Ruler v-model:scrubbing="scrubbing" :animate="animateRuler" />
						<div class="flex flex-col py-2 space-y-1">
							<Sequence v-for="sequence, direction in app.currentAnimation?.sequences"
								:sequence="sequence" :direction="direction" :animate="animateSequences" />
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
import {
	selectDirection, setAnimationLooping, setTimelineZoomAmount
} from "@/api/document"
import { useAppStore } from "@/stores/app"
import { debounceAnimation } from "@/utils/animation"
import Slider from "@/components/basic/Slider.vue"
import Zoom from "@/components/basic/icons/Zoom.vue"
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
