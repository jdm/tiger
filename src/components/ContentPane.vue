<template>
	<Pane>
		<template #header>
			<PaneTab @select="focusContentTab('frames')" :selected="currentTab == 'frames'">Frames</PaneTab>
			<PaneTab @select="focusContentTab('animations')" :selected="currentTab == 'animations'">Animations</PaneTab>
		</template>
		<template #content>
			<div class="px-4 py-2 ">
				<ul v-if="currentTab == 'frames'">
					<li v-for="frame in app.currentDocument?.sheet.frames"
						@click="(event) => onFrameClicked(frame, event)" class="overflow-x-hidden text-ellipsis"
						:class="frame.selected ? 'text-blue-500' : ''">
						{{ frame.name }}</li>
				</ul>
				<ul v-if="currentTab == 'animations'">
					<li v-for="animation in app.currentDocument?.sheet.animations"
						@click="(event) => onAnimationClicked(animation, event)" class="overflow-x-hidden text-ellipsis"
						:class="animation.selected ? 'text-blue-500' : ''">
						{{ animation.name }}
					</li>
				</ul>
			</div>
		</template>
	</Pane>
</template>

<script setup lang="ts">
import { useAppStore } from '@/stores/app'
import { computed } from '@vue/reactivity';
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