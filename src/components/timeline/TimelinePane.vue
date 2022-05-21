<template>
	<Pane>
		<div class="w-full pl-4 p-2 pb-0 flex flex-row items-center space-x-2">
			<button
				class="px-4 py-2 rounded-md uppercase text-xs font-bold bg-green-500 border-y border-t-green-400 border-b-green-600">
				Import
			</button>
		</div>
		<PaneInset class="flex-1 m-4 mt-2 ">
			<div class="relative flex flex-row h-full">
				<div class="flex flex-col bg-plastic-700">
					<div class="h-8 bg-plastic-600" />
					<div
						class="w-40 flex-initial flex flex-col py-2 space-y-1 divide-plastic-800 text-plastic-300 text-xs uppercase font-semibold text-right">
						<div class="h-8 px-4 inline-flex items-center justify-end">North</div>
						<div class="h-8 px-4 inline-flex items-center justify-end">North East</div>
						<div class="h-8 px-4 inline-flex items-center justify-end">East</div>
						<div class="h-8 px-4 inline-flex items-center justify-end">South East</div>
						<div class="h-8 px-4 inline-flex items-center justify-end">South</div>
						<div class="h-8 px-4 inline-flex items-center justify-end">South West</div>
						<div class="h-8 px-4 inline-flex items-center justify-end">West</div>
						<div class="h-8 px-4 inline-flex items-center justify-end">North West</div>
					</div>
				</div>
				<div class="flex-1 flex flex-col relative overflow-x-scroll styled-scrollbars">
					<div class="h-8 w-[2800px] px-1 self-stretch ruler" />
					<div class="flex-1 flex flex-col min-w-0 py-2 space-y-1 bg-plastic-700">
						<div
							class="w-full h-9 flex flex-row p-1 rounded-sm bg-plastic-800 border-y border-t-plastic-900 border-b-plastic-600">
							<Keyframe v-for="keyframe in app.currentAnimation?.timeline" :keyframe="keyframe" />
						</div>
						<div
							class="w-full h-8 rounded-sm p-1 bg-plastic-800 border-y border-t-plastic-900 border-b-plastic-600" />
						<div
							class="w-full h-8 rounded-sm p-1 bg-plastic-800 border-y border-t-plastic-900 border-b-plastic-600" />
						<div
							class="w-full h-8 rounded-sm p-1 bg-plastic-800 border-y border-t-plastic-900 border-b-plastic-600" />
						<div
							class="w-full h-8 rounded-sm p-1 bg-plastic-800 border-y border-t-plastic-900 border-b-plastic-600" />
						<div
							class="w-full h-8 rounded-sm p-1 bg-plastic-800 border-y border-t-plastic-900 border-b-plastic-600" />
						<div
							class="w-full h-8 rounded-sm p-1 bg-plastic-800 border-y border-t-plastic-900 border-b-plastic-600" />
						<div
							class="w-full h-8 rounded-sm p-1 bg-plastic-800 border-y border-t-plastic-900 border-b-plastic-600" />
					</div>
					<div class="absolute top-0 left-[200px] h-full w-px bg-white" />
				</div>
			</div>
		</PaneInset>
	</Pane>
</template>

<script setup lang="ts">
import Pane from '@/components/basic/Pane.vue'
import PaneInset from '@/components/basic/PaneInset.vue'
import Keyframe from '@/components/timeline/Keyframe.vue'
import { useAppStore } from '@/stores/app'

const app = useAppStore()

</script>

<style scoped>
.ruler {
	background:
		/* 1s ticks */
		linear-gradient(90deg, theme('colors.plastic.400') 1px, transparent 1px) left bottom repeat-x,
		/* 100ms ticks */
		linear-gradient(90deg, theme('colors.plastic.400') 1px, transparent 1px) left bottom repeat-x,
		/* 10ms tick */
		linear-gradient(90deg, theme('colors.plastic.400') 1px, transparent 1px) left bottom repeat-x,
		/* Solig BG */
		theme('colors.plastic.600') repeat-x;
	background-origin: content-box;
	background-size:
		1000px 100%,
		100px 10px,
		10px 4px,
		100% 100%;
}
</style>