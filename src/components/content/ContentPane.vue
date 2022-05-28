<template>
	<Pane>
		<PaneTabList>
			<PaneTab @select="focusContentTab(ContentTab.Frames)" :selected="currentTab == ContentTab.Frames">Frames
			</PaneTab>
			<PaneTab @select="focusContentTab(ContentTab.Animations)" :selected="currentTab == ContentTab.Animations">
				Animations
			</PaneTab>
		</PaneTabList>
		<Frames v-if="currentTab == ContentTab.Frames" />
		<Animations v-if="currentTab == ContentTab.Animations" />
	</Pane>
</template>

<script setup lang="ts">
import { useAppStore } from '@/stores/app'
import { watch } from 'vue';
import { computed, Ref, ref } from '@vue/reactivity';
import { focusContentTab } from '@/api/document'
import { ContentTab } from '@/api/dto'
import Pane from '@/components/basic/Pane.vue'
import PaneTab from '@/components/basic/PaneTab.vue'
import PaneTabList from '@/components/basic/PaneTabList.vue'
import Animations from '@/components/content/Animations.vue'
import Frames from '@/components/content/Frames.vue'
import { nextTick } from 'vue';

// TODO Consider saving and restoring scroll positions between content tab or document changes

const app = useAppStore();

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
