<template>
	<div class="flex items-center my-2
		cursor-pointer
		bg-plastic-900 rounded-md
		outline outline-offset-2 outline-plastic-900
	">
		<BackwardIcon @click="jumpToAnimationStart"
			class="h-9 p-1.5 px-5 text-plastic-300 hover:text-plastic-200 active:scale-90 transition duration-100" />
		<ChevronLeftIcon @click="jumpToPreviousFrame"
			class="h-9 p-1 text-plastic-300 hover:text-plastic-200 active:scale-90 transition duration-100" />
		<div class="relative w-20 h-9 active:scale-95 transition duration-100" @click="togglePlayback"
			@mouseenter="onMouseEnterTogglePlayback" @mouseleave="onMouseLeaveTogglePlayback">
			<div v-if="!isPlaying">
				<PlayCircleIcon class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 h-8
				bg-green-600 text-green-200 hover:text-green-100
					box-content border-4 border-plastic-900
					outline outline-offset-3 outline-plastic-700
					p-1.5 rounded-full" />
			</div>
			<div v-if="isPlaying">
				<PauseCircleIcon class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 h-8
					bg-red-600 text-red-200 hover:text-red-100
					box-content border-4 border-plastic-900
					outline outline-offset-3 outline-plastic-700
					p-1.5 rounded-full" />
			</div>
			<div class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 w-8 h-8 blur-md mix-blend-screen pointer-events-none"
				v-if="togglePlaybackHovered" :class="isPlaying ? 'bg-rose-500/50' : 'bg-teal-500/50'" />
		</div>
		<ChevronRightIcon @click="jumpToNextFrame"
			class="h-9 p-1 text-plastic-300 hover:text-plastic-200 active:scale-90 transition duration-100" />
		<ForwardIcon @click="jumpToAnimationEnd"
			class="h-9 p-1.5 px-5 text-plastic-300 hover:text-plastic-200 active:scale-90 transition duration-100" />
	</div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue"
import {
	jumpToAnimationStart, jumpToAnimationEnd,
	jumpToNextFrame, jumpToPreviousFrame,
	play, pause,
} from "@/backend/api"
import { useStateStore } from "@/stores/state"
import { ChevronLeftIcon, ChevronRightIcon } from "@heroicons/vue/20/solid";
import { BackwardIcon, ForwardIcon, PauseCircleIcon, PlayCircleIcon } from "@heroicons/vue/24/solid";

const state = useStateStore();

const togglePlaybackHovered = ref(false);
const isPlaying = computed(() => !!state.currentDocument?.timelineIsPlaying);

function onMouseEnterTogglePlayback() {
	togglePlaybackHovered.value = true;
}

function onMouseLeaveTogglePlayback() {
	togglePlaybackHovered.value = false;
}

function togglePlayback() {
	if (isPlaying.value) {
		pause();
	} else {
		play();
	}
}

</script>
