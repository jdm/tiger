<template>
	<div class="flex-1 flex flex-col min-h-0 p-4 gap-4">
		<div class="w-full flex gap-2 items-center">
			<InputSearch class="flex-1" placeholder="Search animations" v-model="searchQuery" />
			<Button :positive="true" :icon="FilmIcon" label="New" @click="createAnimation" />
		</div>
		<PaneInset class="flex-1 min-h-0">
			<StatefulScroll ref="scrollableElement" v-model:scroll-top="scrollPosition"
				class="p-4 h-full styled-scrollbars" @click="clearSelection"
				@contextmenu.stop.prevent="onOpenContextMenu">
				<div class="flex flex-col">
					<Animation ref="animationElements" v-for="animation in visibleAnimations" :animation="animation"
						:key="animation.name" />
				</div>
				<ContextMenu ref="contextMenu" :content="contextMenuEntries" />
			</StatefulScroll>
		</PaneInset>
	</div>
</template>

<script setup lang="ts">
import { computed, nextTick, Ref, ref, watch } from "vue"
import { FilmIcon } from "@heroicons/vue/20/solid"
import { clearSelection, createAnimation, filterAnimations, paste, setAnimationsListOffset } from "@/backend/api"
import { ClipboardManifest } from "@/backend/dto"
import { useStateStore } from "@/stores/state"
import Animation from "@/components/animations/Animation.vue"
import Button from "@/components/basic/Button.vue"
import ContextMenu from "@/components/basic/ContextMenu.vue"
import InputSearch from "@/components/basic/InputSearch.vue"
import PaneInset from "@/components/basic/PaneInset.vue"
import StatefulScroll from "@/components/basic/StatefulScroll.vue"

const state = useStateStore();
const contextMenu: Ref<typeof ContextMenu | null> = ref(null);
const scrollableElement: Ref<typeof StatefulScroll | null> = ref(null);
const animationElements: Ref<(typeof Animation)[]> = ref([]);

const contextMenuEntries = computed(() => [
	{ name: "Paste", shortcut: "Ctrl+V", action: paste, disabled: state.clipboardManifest != ClipboardManifest.Animations },
]);

const scrollPosition =  computed({
	get: () => state.currentDocument?.animationsListOffset || 0,
	set: (offset) => setAnimationsListOffset(offset),
});

watch(() => state.currentDocument?.lastInteractedAnimation, (name) => {
	if (!name) {
		return;
	}
	nextTick(() => {
		const target = animationElements.value.find((el) => el.getAnimation().name == name);
		if (!target || !scrollableElement.value) {
			return;
		}
		scrollableElement.value.scrollToElement(target.$el);
	});
});

const visibleAnimations = computed(() => {
	return state.currentDocument?.sheet.animations.filter((a) => !a.filteredOut);
});

const searchQuery = computed({
	get: () => state.currentDocument?.animationsFilter || "",
	set: filterAnimations,
});

function onOpenContextMenu(event: MouseEvent) {
	if (contextMenu.value) {
		contextMenu.value.show(event);
	}
}
</script>