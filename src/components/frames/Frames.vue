<template>
	<div class="flex-1 flex flex-col items-stretch min-h-0 p-4 gap-4">
		<div class="w-full flex gap-2 items-center">
			<MultiSwitch :items="listModes" @activate="switchListMode" />
			<InputSearch placeholder="Search frames" v-model="searchQuery" />
			<Button :positive="true" :icon="PhotoIcon" label="Import" @click="importFrames" />
		</div>
		<PaneInset class="flex-1 min-h-0">
			<StatefulScroll ref="scrollableElement" v-model:scroll-top="scrollPosition"
				class="p-4 h-full styled-scrollbars" @click="clearSelection">
				<div :class="listMode == ListMode.Grid4xN ? 'grid grid-cols-4 gap-4' : 'flex flex-col'">
					<Frame ref="frameElements" v-for="frame in visibleFrames" :frame="frame" :key="frame.path"
						:compact="listMode == ListMode.Linear" />
				</div>
			</StatefulScroll>
		</PaneInset>
	</div>
</template>

<script setup lang="ts">
import { computed, nextTick, Ref, ref, watch } from "vue"
import { Bars4Icon, PhotoIcon, Squares2X2Icon } from "@heroicons/vue/20/solid"
import { useAppStore } from "@/stores/app"
import { ListMode } from "@/api/dto"
import { importFrames } from "@/api/local"
import { clearSelection, filterFrames,  setFramesListMode, setFramesListOffset } from "@/api/document"
import Button from "@/components/basic/Button.vue"
import InputSearch from "@/components/basic/InputSearch.vue"
import MultiSwitch, { MultiSwitchItem } from "@/components/basic/MultiSwitch.vue"
import PaneInset from "@/components/basic/PaneInset.vue"
import StatefulScroll from "@/components/basic/StatefulScroll.vue"
import Frame from "@/components/frames/Frame.vue"

const app = useAppStore();
const scrollableElement: Ref<typeof StatefulScroll | null> = ref(null);
const frameElements: Ref<(typeof Frame)[]> = ref([]);

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

const scrollPosition = computed({
	get: () => app.currentDocument?.framesListOffset || 0,
	set: setFramesListOffset,
});

watch(() => app.currentDocument?.lastInteractedFrame, (path) => {
	if (!path) {
		return;
	}
	nextTick(() => {
		const target = frameElements.value.find((el) => el.framePath == path);
		if (!target || !scrollableElement.value) {
			return;
		}
		scrollableElement.value.scrollToElement(target.$el);
	});
});

function switchListMode(item: MultiSwitchItem) {
	setFramesListMode(item.value as ListMode);
}

const searchQuery = computed({
	get: () => app.currentDocument?.framesFilter || "",
	set: filterFrames,
});

</script>