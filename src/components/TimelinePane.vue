<template>
	<Pane>
		<div class="w-full pl-4 p-2 pb-0 flex flex-row items-center space-x-2">
			<button
				class="px-4 py-2 rounded-md uppercase text-xs font-bold bg-orange-500 border-y border-t-orange-400 border-b-orange-600">
				Import
			</button>
		</div>
		<div class="flex-1 m-4 mt-2 rounded-md bg-plastic-900 border-y border-plastic-800 border-b-plastic-600">
			<div class="relative flex flex-row h-full">
				<div class="flex flex-col">
					<div class="h-6 bg-plastic-600" />
					<div
						class="w-40 flex-initial flex flex-col bg-plastic-900 pt-1 space-y-px divide-plastic-800 text-plastic-300 text-right">
						<div class="h-7 bg-plastic-500 px-4">North</div>
						<div class="h-7 bg-plastic-500 px-4">North East</div>
						<div class="h-7 bg-plastic-500 px-4">East</div>
						<div class="h-7 bg-plastic-500 px-4">South East</div>
						<div class="h-7 bg-plastic-500 px-4">South</div>
						<div class="h-7 bg-plastic-500 px-4">South West</div>
						<div class="h-7 bg-plastic-500 px-4">West</div>
						<div class="h-7 bg-plastic-500 px-4">North West</div>
					</div>
				</div>
				<div class="flex-1 flex flex-col relative overflow-x-scroll styled-scrollbars">
					<div class="h-6 w-[2800px] self-stretch ruler" />
					<div class="flex-1 flex flex-col min-w-0 pt-1 space-y-px">
						<div class="h-7 flex flex-row min-w-fit font-semibold text-orange-900">
							<div class="w-60 px-3 bg-yellow-400 border-r border-orange-700 border-t-yellow-200">Frame A
							</div>
							<div class="w-40 px-3 bg-yellow-400 border-r border-orange-700">Frame B</div>
							<div class="w-[2200px] px-3 bg-yellow-400 border-r border-orange-700">Frame C</div>
						</div>
						<div class="h-7 flex flex-row min-w-fit font-semibold text-orange-900">
							<div class="w-60 px-3 bg-yellow-400 border-r border-orange-700 border-t-yellow-200">Frame A
							</div>
							<div class="w-40 px-3 bg-yellow-400 border-r border-orange-700">Frame B</div>
							<div class="w-[2200px] px-3 bg-yellow-400 border-r border-orange-700">Frame C</div>
						</div>
						<div class="w-full bg-plastic-600 h-7" />
						<div class="w-full bg-plastic-600 h-7" />
						<div class="w-full bg-plastic-600 h-7" />
						<div class="w-full bg-plastic-600 h-7" />
						<div class="w-full bg-plastic-600 h-7" />
						<div class="w-full bg-plastic-600 h-7" />
					</div>
					<div class="absolute top-0 left-44 ml-[0px] h-full w-px bg-red-500" />
				</div>
				<div class="absolute top-0 left-0 w-full h-full shadow-inner shadow-neutral-800 pointer-events-none" />
			</div>
			<!--
			</div> -->

		</div>
	</Pane>
</template>

<script setup lang="ts">
import Pane from '@/components/basic/Pane.vue'
</script>

<style scoped>
.ruler {
	background:
		/* Hide initial tick*/
		linear-gradient(90deg, theme('colors.plastic.500') 1px, transparent 1px),
		/* 1s ticks */
		linear-gradient(90deg, theme('colors.plastic.800') 1px, transparent 1px) left bottom repeat-x,
		/* 100ms ticks */
		linear-gradient(90deg, theme('colors.plastic.800') 1px, transparent 1px) left bottom repeat-x,
		/* 10ms tick */
		linear-gradient(90deg, theme('colors.plastic.800') 1px, transparent 1px) left bottom repeat-x,
		/* Solig BG */
		theme('colors.plastic.500') repeat-x;
	background-size:
		100% 100%,
		1000px 100%,
		100px 10px,
		10px 4px,
		100% 100%;
}
</style>