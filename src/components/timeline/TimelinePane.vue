<template>
	<Pane>
		<div class="w-full pl-4 p-2 pb-0 flex flex-row items-center space-x-2">
			<button v-if="app.currentDocument?.timelineIsPlaying" @click="pause"
				class="py-1 px-2 rounded-md uppercase text-xs text-gray-800 font-bold bg-gray-300 border-y border-t-gray-100 border-b-gray-900">
				<PauseIcon class="w-6" />
			</button>
			<button v-if="!app.currentDocument?.timelineIsPlaying" @click="play"
				class="py-1 px-2 rounded-md uppercase text-xs text-gray-800 font-bold bg-gray-300 border-y border-t-gray-100 border-b-gray-900">
				<PlayIcon class="w-6" />
			</button>
			<button @click="zoomInTimeline"
				class="py-1 px-2 rounded-md uppercase text-xs text-gray-800 font-bold bg-gray-300 border-y border-t-gray-100 border-b-gray-900">
				<ZoomInIcon class="w-6" />
			</button>
			<button @click="zoomOutTimeline"
				class="py-1 px-2 rounded-md uppercase text-xs text-gray-800 font-bold bg-gray-300 border-y border-t-gray-100 border-b-gray-900">
				<ZoomOutIcon class="w-6" />
			</button>
		</div>
		<PaneInset class="flex-1 m-4 mt-2 ">
			<div class="relative flex flex-row h-full bg-plastic-700">
				<div class="flex flex-col">
					<div class="h-6 bg-plastic-600" />
					<div
						class="w-36 flex-initial flex flex-col py-2 space-y-1 text-plastic-300 text-xs uppercase font-semibold text-right">
						<div v-for="sequence, direction in app.currentAnimation?.sequences"
							class="h-9 px-4 mx-2 inline-flex items-center justify-end">
							{{ direction }}
						</div>
					</div>
				</div>
				<div class="flex-1 relative overflow-x-scroll styled-scrollbars">
					<div class="min-w-full flex flex-col" :style="timelineStyle">
						<Ruler v-model:scrubbing="scrubbing" />
						<div class="flex flex-col py-2 space-y-1 ">
							<Sequence v-for="sequence in app.currentAnimation?.sequences" :sequence="sequence" />
						</div>
						<div class="absolute top-0 mx-1 h-full w-px bg-white transition" :style="playheadStyle" />
					</div>
				</div>
			</div>
		</PaneInset>
	</Pane>
</template>

<script setup lang="ts">
import { play, pause, zoomInTimeline, zoomOutTimeline } from '@/api/document'
import Pane from '@/components/basic/Pane.vue'
import PaneInset from '@/components/basic/PaneInset.vue'
import Ruler from '@/components/timeline/Ruler.vue'
import Sequence from '@/components/timeline/Sequence.vue'
import { useAppStore } from '@/stores/app'
import { PauseIcon, PlayIcon, ZoomInIcon, ZoomOutIcon } from '@heroicons/vue/solid'
import { computed, Ref, ref } from '@vue/reactivity'
import { watch } from 'vue'

const app = useAppStore();

const scrubbing = ref(false);

const timelineDuration = computed(() => {
	const zoom = app.currentDocument?.timelineZoom || 1;
	const bonusDuration = 500;
	const animationDuration = Math.max(...Object.values(app.currentAnimation?.sequences || {}).map(s => s.durationMillis || 0)) || 0;
	return zoom * (animationDuration + bonusDuration);
});

const timelineStyle = computed(() => {
	return {
		width: timelineDuration.value + "px"
	}
});

const transitionProperty: Ref<string> = ref("none");

watch([() => app.currentDocument?.timelineIsPlaying, scrubbing], ([isPlaying, isScrubbing]) => {
	if (isPlaying || isScrubbing) {
		transitionProperty.value = "none";
	} else {
		// Delay so the transition doesn't kick in as the animation ends and the playhead
		// still has to reach its final location.
		setTimeout(() => {
			if (!app.currentDocument?.timelineIsPlaying && !isScrubbing) {
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
		left: Math.floor(zoom * time) + "px",
	};
});

</script>
