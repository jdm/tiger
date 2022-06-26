<template>
	<div class="flex flex-row items-center my-2
		cursor-pointer
		bg-plastic-900 rounded-md
		outline outline-offset-2 outline-plastic-900
	">
		<Icon name="RewindIcon" @click="jumpToAnimationStart"
			class="h-9 p-1.5 px-5 text-plastic-300 hover:text-plastic-200" />
		<Icon name="ChevronLeftIcon" @click="jumpToPreviousFrame"
			class="h-9 p-1 text-plastic-300 hover:text-plastic-200" />
		<div v-if="!app.currentDocument?.timelineIsPlaying" @click="play" class="relative w-20">
			<Icon name="PlayIcon" class="absolute left-1/2 -translate-x-1/2 -translate-y-1/2 h-8
				text-green-100 bg-green-600
				box-content border-4 border-plastic-900
				outline outline-offset-3 outline-plastic-700
				p-1.5 rounded-full
			" />
		</div>
		<div v-if="app.currentDocument?.timelineIsPlaying" @click="pause" class="relative w-20">
			<Icon name="PauseIcon" class="absolute left-1/2 -translate-x-1/2 -translate-y-1/2 h-8
				text-red-100 bg-red-600
				box-content border-4 border-plastic-900
				outline outline-offset-3 outline-plastic-700
				p-1.5 rounded-full
			" />
		</div>
		<Icon name="ChevronRightIcon" @click="jumpToNextFrame"
			class="h-9 p-1 text-plastic-300 hover:text-plastic-200" />
		<Icon name="FastForwardIcon" @click="jumpToAnimationEnd"
			class="h-9 p-1.5 px-5 text-plastic-300 hover:text-plastic-200" />
	</div>
</template>

<script setup lang="ts">
import {
	jumpToAnimationStart, jumpToAnimationEnd,
	jumpToNextFrame, jumpToPreviousFrame,
	play, pause,
} from "@/api/document"
import { useAppStore } from "@/stores/app"
import Icon from "@/components/basic/Icon.vue"

const app = useAppStore();
</script>