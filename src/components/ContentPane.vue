<template>
	<Pane>
		<template #header>
			<PaneTab @select="focusContentTab('frames')" :selected="currentTab == 'frames'">Frames</PaneTab>
			<PaneTab @select="focusContentTab('animations')" :selected="currentTab == 'animations'">Animations</PaneTab>
		</template>
		<template #content>
			<div v-if="currentTab == 'frames'" class="grid grid-cols-4 gap-4 m-4">
				<div v-for="frame in app.currentDocument?.sheet.frames" @click="(event) => onFrameClicked(frame, event)"
					class="flex flex-col rounded-sm cursor-pointer"
					:class="frame.selected ? 'outline outline-4 outline-blue-500' : ''">
					<div class="flex place-content-center aspect-square checkerboard rounded-sm overflow-hidden">
						<img :src="convertFileSrc(frame.path)" class="pixelated object-none" />
					</div>
					<div class="text-xs p-1 overflow-hidden text-ellipsis"
						:class="frame.selected ? 'bg-blue-500 text-white' : 'text-zinc-400'">{{ frame.name }}</div>
				</div>
			</div>
			<div v-if="currentTab == 'animations'">
				<button
					class="inline-flex items-center m-2 px-4 py-2 rounded-md uppercase text-xs font-bold  bg-blue-500">
					<PlusIcon class="inline-block w-4 h-4" />
					<div class="inline-block pl-2">New Animation</div>
				</button>
				<div v-for="animation in app.currentDocument?.sheet.animations"
					@click="(event) => onAnimationClicked(animation, event)"
					class="p-2 border-b last:border-0 border-gray-700 pl-4 overflow-x-hidden text-ellipsis hover:bg-gray-700"
					:class="animation.selected ? 'text-sky-400 bg-gray-900' : ''">
					<div class="flex flex-row">
						<FilmIcon class="self-center w-6 h-6" />
						<div class="px-4 flex flex-col">
							<div class="">{{ animation.name }}</div>
							<div class="text-xs text-gray-400">4 directions · 220ms</div>
						</div>
					</div>
				</div>
			</div>
		</template>
	</Pane>
</template>

<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/tauri'
import { useAppStore } from '@/stores/app'
import { computed } from '@vue/reactivity';
import { FilmIcon, PlusIcon, PhotographIcon } from '@heroicons/vue/outline'
import Pane from '@/components/pane/Pane.vue'
import PaneTab from '@/components/pane/PaneTab.vue'
import { focusContentTab, selectFrame, selectAnimation } from '@/api/document'
import { Animation, Frame } from '@/api/dto';

const app = useAppStore()

const currentTab = computed(() => {
	return app.currentDocument?.view?.contentTab
})

function onFrameClicked(frame: Frame, event: MouseEvent) {
	selectFrame(frame.path, event.shiftKey, event.ctrlKey)
}

function onAnimationClicked(animation: Animation, event: MouseEvent) {
	selectAnimation(animation.name, event.shiftKey, event.ctrlKey)
}

</script>

<style scoped>
.pixelated {
	image-rendering: pixelated;
}

.checkerboard {
	background-size: 16px 16px;
	background-image:
		linear-gradient(45deg, theme('colors.neutral.700') 25%, transparent 25%, transparent 75%, theme('colors.neutral.700') 75%, theme('colors.neutral.700') 100%),
		linear-gradient(45deg, theme('colors.neutral.700') 25%, theme('colors.neutral.600') 25%, theme('colors.neutral.600') 75%, theme('colors.neutral.700') 75%, theme('colors.neutral.700') 100%);
	background-position:
		0px 0px,
		8px 8px;
}
</style>