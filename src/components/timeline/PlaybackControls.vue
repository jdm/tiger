<template>
	<div class="flex items-center my-2
		cursor-pointer
		bg-plastic-900 rounded-md
		outline outline-offset-2 outline-plastic-900
	">
		<BackwardIcon @click="jumpToAnimationStart" class="h-9 p-1.5 px-5 text-plastic-300 hover:text-plastic-200" />
		<ChevronLeftIcon @click="jumpToPreviousFrame" class="h-9 p-1 text-plastic-300 hover:text-plastic-200" />
		<div v-if="!app.currentDocument?.timelineIsPlaying" @click="play" class="relative w-20">
			<PlayCircleIcon class="absolute left-1/2 -translate-x-1/2 -translate-y-1/2 h-8
				text-green-100 bg-green-600
				box-content border-4 border-plastic-900
				outline outline-offset-3 outline-plastic-700
				p-1.5 rounded-full
			" />
		</div>
		<div v-if="app.currentDocument?.timelineIsPlaying" @click="pause" class="relative w-20">
			<PauseCircleIcon class="absolute left-1/2 -translate-x-1/2 -translate-y-1/2 h-8
				text-red-100 bg-red-600
				box-content border-4 border-plastic-900
				outline outline-offset-3 outline-plastic-700
				p-1.5 rounded-full
			" />
		</div>
		<ChevronRightIcon @click="jumpToNextFrame" class="h-9 p-1 text-plastic-300 hover:text-plastic-200" />
		<ForwardIcon @click="jumpToAnimationEnd" class="h-9 p-1.5 px-5 text-plastic-300 hover:text-plastic-200" />
	</div>
</template>

<script setup lang="ts">
import {
	jumpToAnimationStart, jumpToAnimationEnd,
	jumpToNextFrame, jumpToPreviousFrame,
	play, pause,
} from "@/api/document"
import { useAppStore } from "@/stores/app"
import { ChevronLeftIcon, ChevronRightIcon } from "@heroicons/vue/20/solid";
import { BackwardIcon, ForwardIcon, PauseCircleIcon, PlayCircleIcon } from "@heroicons/vue/24/solid";

const app = useAppStore();
</script>