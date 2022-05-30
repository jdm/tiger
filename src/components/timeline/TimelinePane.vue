<template>
	<Pane>
		<div class="w-full pl-4 p-2 pb-0 flex flex-row items-center space-x-2">
			<div
				class="space-x-2 justify-evenly flex flex-row rounded-md items-center cursor-pointer bg-plastic-800 border-2 border-plastic-900 text-plastic-500">
				<Icon name="ArrowsExpandIcon" class="rotate-45 w-9 p-2" />
				<Icon name="SwitchHorizontalIcon"
					class="w-9 p-2 rounded-md text-plastic-200 border-y border-t-amber-600 border-b-amber-900 bg-gradient-to-b from-amber-800 to-amber-600" />
				<Icon name="SwitchVerticalIcon" class="w-9 p-2" />
				<Icon name="ArrowsExpandIcon" class="w-9 p-2" />
				<Icon name="SunIcon" class="w-9 p-2" />
				<Icon name="CubeIcon" :outline="true" class="w-9 p-2" />
			</div>
			<Button @click="zoomInTimeline" icon="RefreshIcon" />
			<Separator :vertical="true" class="h-full px-2 py-1" />
			<Button @click="zoomInTimeline" icon="ChevronLeftIcon" />
			<Button v-if="app.currentDocument?.timelineIsPlaying" @click="pause" icon="PauseIcon" :danger="true" />
			<Button v-if="!app.currentDocument?.timelineIsPlaying" @click="play" icon="PlayIcon" :positive="true" />
			<Button @click="zoomInTimeline" icon="ChevronRightIcon" />
			<Separator :vertical="true" class="h-full px-2 py-1" />
			<Button @click="zoomInTimeline" icon="ZoomInIcon" />
			<Button @click="zoomOutTimeline" icon="ZoomOutIcon" />
		</div>
		<PaneInset class="flex-1 m-4 mt-2 ">
			<div class="relative flex flex-row h-full bg-plastic-700">
				<div class="flex flex-col">
					<div class="h-6 bg-plastic-600" />
					<div
						class="w-36 flex flex-col py-2 space-y-1 text-plastic-300 text-xs uppercase font-semibold text-right">
						<div v-for="sequence, direction in app.currentAnimation?.sequences"
							class="h-9 px-4 mx-2 inline-flex items-center justify-end">
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
							<div v-for="n in Array(8 - Object.keys(app.currentAnimation?.sequences || []).length)"
								class="h-9" />
						</div>
						<div class="absolute top-0 mx-1 h-full w-px bg-white transition" :style="playheadStyle" />
					</div>
				</div>
			</div>
		</PaneInset>
	</Pane>
</template>

<script setup lang="ts">
import { watch } from 'vue'
import { computed, Ref, ref } from '@vue/reactivity'
import { play, pause, zoomInTimeline, zoomOutTimeline } from '@/api/document'
import { useAppStore } from '@/stores/app'
import Button from '@/components/basic/Button.vue'
import Icon from '@/components/basic/Icon.vue'
import Pane from '@/components/basic/Pane.vue'
import PaneInset from '@/components/basic/PaneInset.vue'
import Separator from '@/components/basic/Separator.vue'
import Ruler from '@/components/timeline/Ruler.vue'
import Sequence from '@/components/timeline/Sequence.vue'

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
		width: timelineSize.value + "px"
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
		left: Math.floor(zoom * time) + "px",
	};
});

</script>
