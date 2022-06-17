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
					<div class="h-2 w-28 bg-plastic-900 rounded-md relative">
						<div class="h-full w-2/3 rounded-l-md
						bg-gradient-to-b from-blue-700 to-blue-600
						border-y border-t-blue-600 border-b-blue-900
						" />
						<div class="absolute left-2/3 top-1/2 w-2.5 h-2.5 -translate-x-1/2 -translate-y-1/2 rounded-full bg-blue-200
							box-content border-2 border-blue-600
						" />
					</div>
					<!-- <Button @click="zoomOutTimeline" icon="ZoomOutIcon" /> -->
					<!-- <Button @click="zoomInTimeline" icon="ZoomInIcon" /> -->
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
						<Ruler v-model:scrubbing="scrubbing" />
						<div class="flex flex-col py-2 space-y-1">
							<Sequence v-for="sequence, direction in app.currentAnimation?.sequences"
								:sequence="sequence" :direction="direction" />
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
import { watch } from "vue"
import { computed, Ref, ref } from "@vue/reactivity"
import {
	zoomInTimeline, zoomOutTimeline,
	selectDirection, setAnimationLooping
} from "@/api/document"
import { useAppStore } from "@/stores/app"
import Button from "@/components/basic/Button.vue"
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
const scrollLeft = ref(0);

function onScroll() {
	scrollLeft.value = scrollableElement.value?.scrollLeft || 0;
}

const animationDuration = computed(() => {
	return Math.max(...Object.values(app.currentAnimation?.sequences || {}).map(s => s.durationMillis || 0)) || 0;
});

const timelineSize = computed(() => {
	const zoom = app.currentDocument?.timelineZoom || 1;
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

const transitionProperty: Ref<string> = ref("none");

watch([() => app.currentDocument?.timelineIsPlaying, scrubbing], ([isPlaying, isScrubbing]) => {
	if (isPlaying || isScrubbing) {
		transitionProperty.value = "none";
	} else {
		// Delay so the transition doesn't kick in as playback is ending
		// and the playhead still has to reach its final location.
		setTimeout(() => {
			if (!app.currentDocument?.timelineIsPlaying && !scrubbing.value) {
				transitionProperty.value = "left";
			}
		}, 300);
	}
});

const playheadStyle = computed(() => {
	const zoom = app.currentDocument?.timelineZoom || 1;
	const time = app.currentDocument?.timelineClockMillis || 0;
	return {
		transitionProperty: transitionProperty.value,
		left: `${Math.floor(zoom * time)}px`,
	};
});

</script>
