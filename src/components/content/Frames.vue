<template>
	<div class="flex-1 flex flex-col items-stretch min-h-0 p-4 space-y-4">
		<div class="w-full flex flex-row space-x-2 items-center">
			<div
				class="flex flex-row rounded-md items-center cursor-pointer bg-plastic-800 border-2 border-plastic-900">
				<ViewGridIcon
					class="w-9 p-2 rounded-md text-plastic-200 border-y border-t-blue-600 border-b-blue-900 bg-gradient-to-b from-blue-800 to-blue-600" />
				<ViewListIcon class="w-9 p-2 text-plastic-700" />
			</div>
			<InputSearch placeholder="Search frames" v-model="searchQuery" />
			<Button :positive="true" icon="PhotographIcon" label="Import" @click="importFrames" />
		</div>
		<PaneInset class="flex-1 min-h-0">
			<div class="p-4 overflow-y-auto h-full styled-scrollbars">
				<div class="grid grid-cols-4 gap-4">
					<Frame v-for="frame in visibleFrames" :frame="frame" :key="frame.name" />
				</div>
			</div>
		</PaneInset>
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { ViewGridIcon, ViewListIcon } from "@heroicons/vue/solid"
import { useAppStore } from "@/stores/app"
import { importFrames } from "@/api/local"
import { filterFrames } from "@/api/document"
import Button from "@/components/basic/Button.vue"
import InputSearch from "@/components/basic/InputSearch.vue"
import PaneInset from "@/components/basic/PaneInset.vue"
import Frame from "@/components/content/Frame.vue"

const app = useAppStore();

const visibleFrames = computed(() => {
	return app.currentDocument?.sheet.frames.filter((f) => !f.filteredOut);
});

const searchQuery = computed({
	get: () => app.currentDocument?.framesFilter || "",
	set: filterFrames,
});

</script>