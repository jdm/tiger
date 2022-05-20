<template>
	<Pane>
		<div class="w-full pl-4 p-2 pb-0 flex flex-row items-center space-x-2">
			<button
				class="px-4 py-2 rounded-md uppercase text-xs font-bold bg-green-500 border-y border-t-green-400 border-b-green-600">
				Import
			</button>
		</div>
		<div class="flex-1 m-4 mt-2 rounded-md bg-plastic-900 border-y border-plastic-800 border-b-plastic-600">
			<div class="relative flex flex-row h-full">
				<div class="flex flex-col">
					<div class="h-8 bg-plastic-900" />
					<div
						class="w-40 flex-initial flex flex-col bg-plastic-900 pt-1 space-y-px divide-plastic-800 text-plastic-300 text-right">
						<div class="h-8 bg-plastic-500 px-4">North</div>
						<div class="h-8 bg-plastic-500 px-4">North East</div>
						<div class="h-8 bg-plastic-500 px-4">East</div>
						<div class="h-8 bg-plastic-500 px-4">South East</div>
						<div class="h-8 bg-plastic-500 px-4">South</div>
						<div class="h-8 bg-plastic-500 px-4">South West</div>
						<div class="h-8 bg-plastic-500 px-4">West</div>
						<div class="h-8 bg-plastic-500 px-4">North West</div>
					</div>
				</div>
				<div class="flex-1 pl-1 flex flex-col relative overflow-x-scroll styled-scrollbars">
					<div class="h-8 w-[2800px] self-stretch ruler" />
					<div class="flex-1 flex flex-col min-w-0 pt-1 space-y-px">
						<div class="h-8 flex flex-row min-w-fit bg-plastic-700">
							<Keyframe v-for="keyframe in app.currentAnimation?.timeline" :keyframe="keyframe" />
						</div>
						<div class="w-full bg-plastic-700 h-8" />
						<div class="w-full bg-plastic-700 h-8" />
						<div class="w-full bg-plastic-700 h-8" />
						<div class="w-full bg-plastic-700 h-8" />
						<div class="w-full bg-plastic-700 h-8" />
						<div class="w-full bg-plastic-700 h-8" />
						<div class="w-full bg-plastic-700 h-8" />
					</div>
					<div class="absolute top-0 left-[200px] h-full w-px bg-white" />
				</div>
				<div
					class="absolute top-0 left-0 w-full h-full shadow-inner shadow-neutral-900/70 pointer-events-none" />
			</div>
		</div>
	</Pane>
</template>

<script setup lang="ts">
import Pane from '@/components/basic/Pane.vue'
import Keyframe from '@/components/timeline/Keyframe.vue'
import { useAppStore } from '@/stores/app'

const app = useAppStore()

</script>

<style scoped>
.ruler {
	background:
		/* Hide initial tick*/
		linear-gradient(90deg, theme('colors.plastic.600') 1px, transparent 1px),
		/* 1s ticks */
		linear-gradient(90deg, theme('colors.plastic.400') 1px, transparent 1px) left bottom repeat-x,
		/* 100ms ticks */
		linear-gradient(90deg, theme('colors.plastic.400') 1px, transparent 1px) left bottom repeat-x,
		/* 10ms tick */
		linear-gradient(90deg, theme('colors.plastic.400') 1px, transparent 1px) left bottom repeat-x,
		/* Solig BG */
		theme('colors.plastic.600') repeat-x;
	background-size:
		100% 100%,
		1000px 100%,
		100px 10px,
		10px 4px,
		100% 100%;
}
</style>