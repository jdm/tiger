<template>
	<div class="flex-1 flex flex-col min-h-0 p-4 space-y-4">
		<div class="w-full flex flex-row space-x-2 items-center">
			<div
				class="flex flex-row rounded-md items-center cursor-pointer bg-plastic-800 border-2 border-plastic-900">
				<ViewGridIcon class="w-9 p-2 text-plastic-700" />
				<ViewListIcon
					class="w-9 p-2 rounded-md text-plastic-200 border-y border-t-blue-600 border-b-blue-900 bg-gradient-to-b from-blue-800 to-blue-600" />
			</div>
			<InputSearch placeholder="Search animations" v-model="searchQuery" />
			<Button :positive="true" icon="FilmIcon" label="New" @click="createAnimation" />
		</div>
		<PaneInset class="flex-1 min-h-0">
			<div class="p-4 overflow-y-auto h-full styled-scrollbars">
				<div class="flex flex-col">
					<Animation v-for="animation in visibleAnimations" :animation="animation" :key="animation.name" />
				</div>
			</div>
		</PaneInset>
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { createAnimation, filterAnimations } from "@/api/document"
import { useAppStore } from "@/stores/app"
import Button from "@/components/basic/Button.vue"
import InputSearch from "@/components/basic/InputSearch.vue"
import PaneInset from "@/components/basic/PaneInset.vue"
import Animation from "@/components/content/Animation.vue"
import { ViewGridIcon, ViewListIcon } from "@heroicons/vue/solid"

const app = useAppStore();

const visibleAnimations = computed(() => {
	return app.sortedAnimations?.filter((a) => !a.filteredOut);
});

const searchQuery = computed({
	get: () => app.currentDocument?.animationsFilter || "",
	set: filterAnimations,
});
</script>