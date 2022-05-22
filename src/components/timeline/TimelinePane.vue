<template>
	<Pane>
		<div class="w-full pl-4 p-2 pb-0 flex flex-row items-center space-x-2">
			<button v-if="app.currentDocument?.timelineIsPlaying" @click="pause"
				class="py-1 px-2 rounded-md uppercase text-xs text-gray-800 font-bold bg-gray-300 border-y border-t-gray-100 border-b-gray-900">
				<PauseIcon class="w-6" />
			</button>
			<button v-if="!app.currentDocument?.timelineIsPlaying" @click="play"
				class="py-1 px-2 rounded-md uppercase text-xs text-gray-800 font-bold bg-gray-300 border-y border-t-gray-100 border-b-gray-900">
				<PlayIcon class="w-6" />
			</button>
		</div>
		<PaneInset class="flex-1 m-4 mt-2 ">
			<div class="relative flex flex-row h-full">
				<div class="flex flex-col bg-plastic-700">
					<div class="h-6 bg-plastic-600" />
					<div
						class="w-36 flex-initial flex flex-col py-2 space-y-1 text-plastic-300 text-xs uppercase font-semibold text-right">
						<div v-for="sequence, direction in app.currentAnimation?.sequences"
							class="h-9 px-4 mx-2 inline-flex items-center justify-end">
							{{ direction }}
						</div>
					</div>
				</div>
				<div class="flex-1 flex flex-col relative overflow-x-scroll styled-scrollbars">
					<Ruler />
					<div class="flex-1 flex flex-col py-2 space-y-1 bg-plastic-700">
						<Sequence v-for="sequence in app.currentAnimation?.sequences" :sequence="sequence" />
					</div>
					<div class="absolute top-0 mx-1 left-[0px] h-full w-px bg-white" />
				</div>
			</div>
		</PaneInset>
	</Pane>
</template>

<script setup lang="ts">
import { play, pause } from '@/api/document'
import Pane from '@/components/basic/Pane.vue'
import PaneInset from '@/components/basic/PaneInset.vue'
import Ruler from '@/components/timeline/Ruler.vue'
import Sequence from '@/components/timeline/Sequence.vue'
import { useAppStore } from '@/stores/app'
import { PauseIcon, PlayIcon } from '@heroicons/vue/solid'

const app = useAppStore();

</script>
