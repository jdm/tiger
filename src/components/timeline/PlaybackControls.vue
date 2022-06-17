<template>
	<div class="rounded-md border-2 border-plastic-900">
		<div class="flex flex-row items-center
		cursor-pointer
		divide-x divide-zinc-500
		bg-zinc-400
		border-y border-b-zinc-700 border-t-zinc-300 border-x-zinc-300
		rounded-md
	">
			<Icon name="ChevronDoubleLeftIcon" class="h-8 p-1.5 px-4 text-plastic-600" />
			<Icon name="ChevronLeftIcon" @click="jumpToPreviousFrame" class="h-8 p-1 px-3 text-plastic-600" />
			<div v-if="!app.currentDocument?.timelineIsPlaying" @click="play" class="relative w-11">
				<Icon name="PlayIcon"
					class="absolute -translate-y-1/2 h-11 ring-4 ring-plastic-700 p-1.5 rounded-full" />
				<Icon name="PlayIcon" class="absolute -translate-y-1/2 h-11 text-green-100
				bg-green-600
				border-2 border-green-500
				ring-2 ring-zinc-900
				p-1.5 rounded-full
			" />
			</div>
			<div v-if="app.currentDocument?.timelineIsPlaying" @click="pause" class="relative w-11">
				<Icon name="PauseIcon"
					class="absolute -translate-y-1/2 h-11 ring-4 ring-plastic-700 p-1.5 rounded-full" />
				<Icon name="PauseIcon" class="absolute -translate-y-1/2 h-11 text-red-100
				bg-red-600
				border-2 border-red-500
				ring-2 ring-zinc-900
				p-1.5 rounded-full
			" />
			</div>
			<Icon name="ChevronRightIcon" @click="jumpToNextFrame" class="h-8 p-1 px-3 text-plastic-600" />
			<Icon name="ChevronDoubleRightIcon" class="h-8 p-1.5 px-4 text-plastic-600" />
		</div>
	</div>
</template>

<script setup lang="ts">
import {
	jumpToNextFrame, jumpToPreviousFrame, play, pause,
} from "@/api/document"
import { useAppStore } from "@/stores/app"
import Icon from "@/components/basic/Icon.vue"

const app = useAppStore();
</script>