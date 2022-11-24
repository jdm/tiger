<template>
	<div class="flex-1 flex flex-col items-stretch min-h-0 p-4 space-y-4">
		<div class="w-full flex flex-row space-x-2 items-center">
			<MultiSwitch :items="listModes" @activate="switchListMode" />
			<InputSearch placeholder="Search frames" v-model="searchQuery" />
			<Button :positive="true" icon="PhotographIcon" label="Import" @click="importFrames" />
		</div>
		<PaneInset class="flex-1 min-h-0">
			<div class="p-4 overflow-y-auto h-full styled-scrollbars">
				<div :class="listMode == ListMode.Grid4xN ? 'grid grid-cols-4 gap-4' : 'flex flex-col'">
					<Frame v-for="frame in visibleFrames" :frame="frame" :key="frame.name"
						:compact="listMode == ListMode.Linear" />
				</div>
			</div>
		</PaneInset>
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { useAppStore } from "@/stores/app"
import { importFrames } from "@/api/local"
import { filterFrames, setFramesListMode } from "@/api/document"
import Button from "@/components/basic/Button.vue"
import InputSearch from "@/components/basic/InputSearch.vue"
import MultiSwitch, { MultiSwitchItem } from "@/components/basic/MultiSwitch.vue"
import PaneInset from "@/components/basic/PaneInset.vue"
import Frame from "@/components/frames/Frame.vue"
import { ListMode } from "@/api/dto"

const app = useAppStore();

const visibleFrames = computed(() => {
	return app.currentDocument?.sheet.frames.filter((f) => !f.filteredOut);
});

const listMode = computed(() => app.currentDocument?.framesListMode || ListMode.Grid4xN);

const listModes = computed((): MultiSwitchItem[] => {
	return [
		{ icon: "ViewGridIcon", active: listMode.value == ListMode.Grid4xN, value: ListMode.Grid4xN },
		{ icon: "ViewListIcon", active: listMode.value == ListMode.Linear, value: ListMode.Linear },
	];
});

function switchListMode(item: MultiSwitchItem) {
	setFramesListMode(item.value as ListMode);
}

const searchQuery = computed({
	get: () => app.currentDocument?.framesFilter || "",
	set: filterFrames,
});

</script>