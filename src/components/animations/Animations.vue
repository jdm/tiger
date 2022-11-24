<template>
	<div class="flex-1 flex flex-col min-h-0 p-4 space-y-4">
		<div class="w-full flex flex-row space-x-2 items-center">
			<InputSearch class="flex-1" placeholder="Search animations" v-model="searchQuery" />
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
import Animation from "@/components/animations/Animation.vue"
import Button from "@/components/basic/Button.vue"
import InputSearch from "@/components/basic/InputSearch.vue"
import PaneInset from "@/components/basic/PaneInset.vue"

const app = useAppStore();

const visibleAnimations = computed(() => {
	return app.sortedAnimations?.filter((a) => !a.filteredOut);
});

const searchQuery = computed({
	get: () => app.currentDocument?.animationsFilter || "",
	set: filterAnimations,
});
</script>