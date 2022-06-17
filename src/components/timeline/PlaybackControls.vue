<template>
	<div class="flex flex-row items-center
		cursor-pointer
		bg-plastic-900
		rounded-md
	">
		<Icon name="ChevronDoubleLeftIcon" @click="jumpToAnimationStart"
			class="h-9 p-1.5 px-5 text-plastic-300 hover:text-plastic-200" />
		<Icon name="ChevronLeftIcon" @click="jumpToPreviousFrame"
			class="h-9 p-1 text-plastic-300 hover:text-plastic-200" />
		<div v-if="!app.currentDocument?.timelineIsPlaying" @click="play" class="relative w-20">
			<Icon name="PlayIcon" class="absolute left-1/2 -translate-x-1/2 -translate-y-1/2 h-11
				text-green-100 bg-green-600
				ring-4 ring-plastic-900
				p-1.5 rounded-full
			" />
		</div>
		<div v-if="app.currentDocument?.timelineIsPlaying" @click="pause" class="relative w-20">
			<Icon name="PauseIcon" class="absolute left-1/2 -translate-x-1/2 -translate-y-1/2 h-11
				text-red-100 bg-red-600
				ring-4 ring-plastic-900
				p-1.5 rounded-full
			" />
		</div>
		<Icon name="ChevronRightIcon" @click="jumpToNextFrame"
			class="h-9 p-1 text-plastic-300 hover:text-plastic-200" />
		<Icon name="ChevronDoubleRightIcon" @click="jumpToAnimationEnd"
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