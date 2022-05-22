<template>
	<Pane>
		<PaneTabList>
			<PaneTab @select="focusContentTab(ContentTab.Frames)" :selected="currentTab == ContentTab.Frames">Frames
			</PaneTab>
			<PaneTab @select="focusContentTab(ContentTab.Animations)" :selected="currentTab == ContentTab.Animations">
				Animations
			</PaneTab>
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
			<PaneInset class="flex-1 m-4 min-h-0">
				<div class="p-4 overflow-y-auto h-full styled-scrollbars">
					<div v-if="currentTab == ContentTab.Frames" class="grid grid-cols-4 gap-4">
						<Frame v-for="frame in app.currentDocument?.sheet.frames" :frame="frame" :key="frame.name" />
					</div>
					<div v-if="currentTab == ContentTab.Animations" class="text-plastic-200 flex flex-col">
						<Animation v-for="animation in app.sortedAnimations" :animation="animation" ref="animationRefs"
							:key="animation.name" />
					</div>
				</div>
			</PaneInset>
		</div>
	</Pane>
</template>

<script setup lang="ts">
import { useAppStore } from '@/stores/app'
import { watch } from 'vue';
import { computed, Ref, ref } from '@vue/reactivity';
import { focusContentTab } from '@/api/document'
import { ContentTab } from '@/api/dto'
import Animation from '@/components/Animation.vue'
import Frame from '@/components/Frame.vue'
import Pane from '@/components/basic/Pane.vue'
import PaneInset from '@/components/basic/PaneInset.vue'
import PaneTab from '@/components/basic/PaneTab.vue'
import PaneTabList from '@/components/basic/PaneTabList.vue'
import Separator from '@/components/basic/Separator.vue'
import { nextTick } from 'vue';

// TODO Consider saving and restoring scroll positions between content tab or document changes

const app = useAppStore()

const animationRefs: Ref<{ name: string, scrollIntoView: () => void }[]> = ref([]);

const currentTab = computed(() => {
	return app.currentDocument?.contentTab
});

// Auto-scroll to new animation
watch([
	() => app.currentDocument,
	() => Object.keys(app.currentDocument?.sheet.animations || [])],
	([newDocument, newAnimationNames], [oldDocument, oldAnimationNames]) => {
		if (newDocument != oldDocument
			|| !oldAnimationNames || !newAnimationNames
			|| app.currentDocument?.contentTab != ContentTab.Animations) {
			return;
		}
		const oldSet = new Set(oldAnimationNames);
		for (const name of newAnimationNames) {
			if (!oldSet.has(name)) {
				nextTick(() => {
					scrollToAnimation(name);
				});
				return;
			}
		}
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
