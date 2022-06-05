<template>
	<div class="flex-1 flex flex-col min-h-0 p-4 space-y-4">
		<div class="w-full flex flex-row space-x-2 items-center">
			<div
				class="flex flex-row rounded-md items-center cursor-pointer bg-plastic-800 border-2 border-plastic-900">
				<ViewGridIcon class="w-9 p-2 text-plastic-700" />
				<ViewListIcon
					class="w-9 p-2 rounded-md text-plastic-200 border-y border-t-blue-600 border-b-blue-900 bg-gradient-to-b from-blue-800 to-blue-600" />
			</div>
			<div
				class="flex-1 rounded-md overflow-hidden bg-plastic-800 border-y border-plastic-900 border-b-plastic-600">
				<input type="text" placeholder="Filter Animations" class="w-full px-4 bg-transparent border-0" />
			</div>
			<Button :positive="true" icon="FilmIcon" label="New" @click="createAnimation" />
		</div>
		<PaneInset class="flex-1 min-h-0">
			<div class="p-4 overflow-y-auto h-full styled-scrollbars">
				<div class="text-plastic-200 flex flex-col">
					<Animation v-for="animation in app.sortedAnimations" :animation="animation" ref="animationRefs"
						:key="animation.name" />
				</div>
			</div>
		</PaneInset>
	</div>
</template>

<script setup lang="ts">
import { watch, nextTick } from 'vue'
import { Ref, ref } from '@vue/reactivity'
import { ContentTab } from '@/api/dto'
import { createAnimation } from '@/api/document'
import { useAppStore } from '@/stores/app'
import Button from '@/components/basic/Button.vue'
import PaneInset from '@/components/basic/PaneInset.vue'
import Animation from '@/components/content/Animation.vue'
import { ViewGridIcon, ViewListIcon } from '@heroicons/vue/solid'

const app = useAppStore();


// Auto-scroll to new animation
const animationRefs: Ref<{ name: string, scrollIntoView: () => void }[]> = ref([]);
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