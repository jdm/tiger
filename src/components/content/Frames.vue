<template>
	<div class="flex-1 flex flex-col items-stretch min-h-0 p-4 space-y-4">
		<div class="w-full flex flex-row space-x-2">
			<div
				class="h-10 flex flex-row rounded-md items-center cursor-pointer bg-plastic-800 ring-2 ring-plastic-900">
				<ViewGridIcon
					class="w-10 p-2 rounded-md text-plastic-200 bg-sky-600 border-y border-t-sky-500 border-b-sky-900" />
				<ViewListIcon class="w-10 p-2 text-plastic-700" />
			</div>
			<div
				class="flex-1 rounded-md overflow-hidden bg-plastic-800 border-y border-plastic-900 border-b-plastic-600">
				<input type="text" placeholder="Filter Frames"
					class="w-full h-[2.55rem] px-4 font-medium bg-transparent border-0" />
			</div>
			<Button :positive="true" icon="PhotographIcon" label="Import" />
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
import { useAppStore } from '@/stores/app'
import Button from '@/components/basic/Button.vue'
import PaneInset from '@/components/basic/PaneInset.vue'
import Frame from '@/components/content/Frame.vue'
import { ViewGridIcon, ViewListIcon } from '@heroicons/vue/solid'

const app = useAppStore();
</script>