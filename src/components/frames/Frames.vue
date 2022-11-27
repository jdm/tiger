<template>
	<div class="flex-1 flex flex-col items-stretch min-h-0 p-4 space-y-4">
		<div class="w-full flex flex-row space-x-2 items-center">
			<MultiSwitch :items="listModes" @activate="switchListMode" />
			<InputSearch placeholder="Search frames" v-model="searchQuery" />
			<Button :positive="true" :icon="PhotoIcon" label="Import" @click="importFrames" />
		</div>
		<PaneInset class="flex-1 min-h-0">
			<div class="p-4 overflow-y-auto h-full styled-scrollbars" @click="clearSelection">
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
import { Bars4Icon, PhotoIcon, Squares2X2Icon } from "@heroicons/vue/20/solid"
import { useAppStore } from "@/stores/app"
import { importFrames } from "@/api/local"
import { clearSelection, filterFrames, setFramesListMode } from "@/api/document"
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
		{ icon: Squares2X2Icon, active: listMode.value == ListMode.Grid4xN, value: ListMode.Grid4xN },
		{ icon: Bars4Icon, active: listMode.value == ListMode.Linear, value: ListMode.Linear },
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