<template>
	<Pane>
		<PaneTabList>
			<PaneTab @select="focusContentTab('frames')" :selected="currentTab == 'frames'">Frames</PaneTab>
			<PaneTab @select="focusContentTab('animations')" :selected="currentTab == 'animations'">Animations</PaneTab>
		</PaneTabList>
		<div class="flex-1 flex flex-col min-h-0">
			<div class="w-full p-4 flex flex-row items-center space-x-2">
				<input type="text" placeholder="Idle"
					class="w-full h-10 px-4 placeholder-plastic-500 font-bold bg-plastic-800 rounded-md border-y border-plastic-900 border-b-plastic-600" />
				<button
					class="m-2 px-4 py-2 rounded-md uppercase text-xs font-bold bg-green-500 border-y border-t-green-400 border-b-green-600">
					<div class="inline-block">Import</div>
				</button>
			</div>
			<Separator />
			<div
				class="flex-1 m-4 p-4 overflow-y-auto styled-scrollbars rounded-md bg-plastic-800 border-y border-plastic-900 border-b-plastic-600">
				<div v-if="currentTab == 'frames'" class="grid grid-cols-4 gap-4">
					<Frame v-for="frame in app.currentDocument?.sheet.frames" :frame="frame" :key="frame.name" />
				</div>
				<div v-if="currentTab == 'animations'" class="text-plastic-200 flex flex-col">
					<Animation v-for="animation in app.currentDocument?.sheet.animations" :animation="animation"
						ref="animationRefs" :key="animation.name" />
				</div>
			</div>
		</div>
	</Pane>
</template>

<script setup lang="ts">
import { useAppStore } from '@/stores/app'
import { watch } from 'vue';
import { computed, Ref, ref } from '@vue/reactivity';
import { focusContentTab } from '@/api/document'
import Animation from '@/components/Animation.vue'
import Frame from '@/components/Frame.vue'
import Pane from '@/components/pane/Pane.vue'
import PaneTab from '@/components/pane/PaneTab.vue'
import PaneTabList from '@/components/pane/PaneTabList.vue'
import Separator from '@/components/Separator.vue'
import { nextTick } from 'vue';

// TODO Consider saving and restoring scroll positions between content tab or document changes

const app = useAppStore()

const animationRefs: Ref<{ name: string, scrollIntoView: () => void }[]> = ref([]);

const currentTab = computed(() => {
	return app.currentDocument?.view?.contentTab
});

// Auto-scroll to new animation
// TODO watch current document instead of currentDocumentPath (looks cleaner) when store patching is implemented
watch([() => app.currentDocumentPath, () => app.currentDocument?.sheet.animations], ([newPath, newAnimations], [oldPath, oldAnimations]) => {
	if (newPath != oldPath || !oldAnimations || !newAnimations || app.currentDocument?.view.contentTab != "animations") {
		return;
	}
	const oldNames = new Set(oldAnimations.map((a) => a.name));
	const createdAnimations = newAnimations.filter((a) => !oldNames.has(a.name));
	if (createdAnimations.length == 0) {
		return;
	}
	nextTick(() => {
		scrollToAnimation(createdAnimations[0].name);
	});
});

function scrollToAnimation(name: string) {
	for (let animationRef of animationRefs.value) {
		if (animationRef.name == name) {
			animationRef.scrollIntoView();
			return;
		}
	}
}
</script>
