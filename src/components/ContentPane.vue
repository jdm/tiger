<template>
	<Pane>
		<template #header>
			<PaneTab @select="focusContentTab('frames')" :selected="currentTab == 'frames'">Frames</PaneTab>
			<PaneTab @select="focusContentTab('animations')" :selected="currentTab == 'animations'">Animations</PaneTab>
		</template>
		<template #content>
			<div v-if="currentTab == 'frames'">
				<div v-for="frame in app.currentDocument?.sheet.frames" @click="(event) => onFrameClicked(frame, event)"
					class="p-2 border-b last:border-0 border-gray-700 pl-4 overflow-x-hidden text-ellipsis hover:bg-gray-700"
					:class="frame.selected ? 'text-sky-400 bg-gray-900' : ''">
					<div class="flex flex-row">
						<PhotographIcon class="self-center w-6 h-6" />
						<div class="px-4 flex flex-col">
							<div class="">{{ frame.name }}</div>
							<div class="text-xs text-gray-400">64x64 px</div>
						</div>
					</div>
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