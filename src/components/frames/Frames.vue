<template>
	<div class="flex-1 flex flex-col items-stretch min-h-0 p-4 gap-4">
		<Transition>
			<div v-if="state.anyFramesMissing" class="-mt-4 -mx-4 overflow-hidden">
				<div class="w-full flex items-center py-2 pl-4 pr-6 bg-red-600 text-plastic-900 text-sm font-medium">
					<ExclamationTriangleIcon class="mr-4 w-9 p-1.5 text-red-600 bg-plastic-900 rounded-full" />
					<div class="grow">Some frames are missing from your computer.</div>
					<a class="underline underline-offset-2 text-red-100 cursor-pointer"
						@click="beginRelocateFrames">Relocate</a>
				</div>
			</div>
		</Transition>
		<div class="w-full flex gap-2 items-center transition-all" :class="darkening">
			<MultiSwitch :items="listModes" @activate="switchListMode" />
			<InputSearch placeholder="Search frames" v-model="searchQuery" />
			<Button :positive="true" :icon="PhotoIcon" label="Import" @click="importFrames" />
		</div>
		<PaneInset class="flex-1 min-h-0 transition-all" :class="darkening">
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
import { ExclamationTriangleIcon } from "@heroicons/vue/24/solid"
import { beginRelocateFrames, clearSelection, filterFrames, importFrames, setFramesListMode, setFramesListOffset } from "@/backend/api"
import { ListMode } from "@/backend/dto"
import { useStateStore } from "@/stores/state"
import Button from "@/components/basic/Button.vue"
import InputSearch from "@/components/basic/InputSearch.vue"
import MultiSwitch, { MultiSwitchItem } from "@/components/basic/MultiSwitch.vue"
import PaneInset from "@/components/basic/PaneInset.vue"
import StatefulScroll from "@/components/basic/StatefulScroll.vue"
import Frame from "@/components/frames/Frame.vue"

const state = useStateStore();
const scrollableElement: Ref<typeof StatefulScroll | null> = ref(null);
const frameElements: Ref<(typeof Frame)[]> = ref([]);

const visibleFrames = computed(() => {
	return state.currentDocument?.sheet.frames.filter((f) => !f.filteredOut);
});

const listMode = computed(() => state.currentDocument?.framesListMode || ListMode.Grid4xN);

const listModes = computed((): MultiSwitchItem[] => {
	return [
		{ icon: Squares2X2Icon, active: listMode.value == ListMode.Grid4xN, value: ListMode.Grid4xN },
		{ icon: Bars4Icon, active: listMode.value == ListMode.Linear, value: ListMode.Linear },
	];
});

const scrollPosition = computed({
	get: () => state.currentDocument?.framesListOffset || 0,
	set: setFramesListOffset,
});

watch(() => state.currentDocument?.lastInteractedFrame, (path) => {
	if (!path) {
		return;
	}
	nextTick(() => {
		const target = frameElements.value.find((el) => el.getFrame().path == path);
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
	get: () => state.currentDocument?.framesFilter || "",
	set: filterFrames,
});

const darkening = computed(() => [
	...state.anyFramesMissing ? ["brightness-[.4]", "saturate-0"] : [],
]);

</script>

<style>
.v-enter-active {
	transition: max-height 0.15s ease-out;
	max-height: 56px;
}

.v-leave-active {
	transition: max-height 0.15s ease-in;
	max-height: 56px;
}

.v-enter-from,
.v-leave-to {
	max-height: 0px;
}
</style>