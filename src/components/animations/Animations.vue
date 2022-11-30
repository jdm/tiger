<template>
	<div class="flex-1 flex flex-col min-h-0 p-4 gap-4">
		<div class="w-full flex gap-2 items-center">
			<InputSearch class="flex-1" placeholder="Search animations" v-model="searchQuery" />
			<Button :positive="true" :icon="FilmIcon" label="New" @click="createAnimation" />
		</div>
		<PaneInset class="flex-1 min-h-0">
			<div class="p-4 overflow-y-auto h-full styled-scrollbars" @click="clearSelection"
				@contextmenu.stop.prevent="onOpenContextMenu">
				<div class="flex flex-col">
					<Animation v-for="animation in visibleAnimations" :animation="animation" :key="animation.name" />
				</div>
				<ContextMenu ref="contextMenu" :content="contextMenuEntries" />
			</div>
		</PaneInset>
	</div>
</template>

<script setup lang="ts">
import { computed, Ref, ref } from "vue"
import { FilmIcon } from "@heroicons/vue/20/solid"
import { clearSelection, createAnimation, filterAnimations, paste } from "@/api/document"
import { ClipboardManifest } from "@/api/dto"
import { useAppStore } from "@/stores/app"
import Animation from "@/components/animations/Animation.vue"
import Button from "@/components/basic/Button.vue"
import ContextMenu from "@/components/basic/ContextMenu.vue"
import InputSearch from "@/components/basic/InputSearch.vue"
import PaneInset from "@/components/basic/PaneInset.vue"

const app = useAppStore();
const contextMenu: Ref<typeof ContextMenu | null> = ref(null);

const contextMenuEntries = computed(() => [
	{ name: "Paste", shortcut: "Ctrl+V", action: paste, disabled: app.clipboardManifest != ClipboardManifest.Animations },
]);

const visibleAnimations = computed(() => {
	return app.sortedAnimations?.filter((a) => !a.filteredOut);
});

const searchQuery = computed({
	get: () => app.currentDocument?.animationsFilter || "",
	set: filterAnimations,
});

function onOpenContextMenu(event: MouseEvent) {
	if (contextMenu.value) {
		contextMenu.value.show(event);
	}
}
</script>