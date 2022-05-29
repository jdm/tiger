<template>
	<div class="flex-1 flex flex-col items-stretch min-h-0 p-4 space-y-4">
		<div class="w-full flex flex-row space-x-2 items-center">
			<div class="flex flex-row rounded-md items-center cursor-pointer bg-plastic-800 ring-2 ring-plastic-900">
				<ViewGridIcon
					class="w-9 p-2 rounded-md text-plastic-200 border-y border-t-amber-600 border-b-amber-900 bg-gradient-to-b from-amber-800 to-amber-600" />
				<ViewListIcon class="w-9 p-2 text-plastic-700" />
			</div>
			<div
				class="flex-1 rounded-md overflow-hidden bg-plastic-800 border-y border-plastic-900 border-b-plastic-600">
				<input type="text" placeholder="Filter Frames" class="w-full px-4 bg-transparent border-0" />
			</div>
			<Button :positive="true" icon="PhotographIcon" label="Import" @click="importFrames" />
		</div>
		<PaneInset class="flex-1 min-h-0">
			<div class="p-4 overflow-y-auto h-full styled-scrollbars">
				<div class="grid grid-cols-4 gap-4">
					<Frame v-for="frame in app.currentDocument?.sheet.frames" :frame="frame" :key="frame.name" />
				</div>
			</div>
		</PaneInset>
	</div>
</template>

<script setup lang="ts">
import { ViewGridIcon, ViewListIcon } from '@heroicons/vue/solid'
import { useAppStore } from '@/stores/app'
import { importFrames } from '@/api/local'
import Button from '@/components/basic/Button.vue'
import PaneInset from '@/components/basic/PaneInset.vue'
import Frame from '@/components/content/Frame.vue'

const app = useAppStore();
</script>