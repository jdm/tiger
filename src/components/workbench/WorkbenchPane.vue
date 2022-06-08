<template>
	<Pane class="overflow-hidden">
		<PaneTabList>
			<PaneTab :closeable="true" v-for="document in app.documents" @select="focusDocument(document.path)"
				@close="closeDocument(document.path)" :selected="document.path == app.currentDocumentPath">
				{{ document.name + (document.hasUnsavedChanges ? "*" : "") }}
			</PaneTab>
		</PaneTabList>
		<div class="w-full px-2 my-4 flex flex-row items-center space-x-2">
			<Button @click="zoomInWorkbench" icon="ZoomInIcon" />
			<Button @click="zoomOutWorkbench" icon="ZoomOutIcon" />
		</div>
		<div class="relative flex-1 overflow-hidden pointer-events-none" ref="drawingArea">

			<!--
				Z-Ordering
				----------
				0 sprite BG
				10 sprite
				20 sprite outline
				30 hitbox BG & outline
				40 sprite drag area
				50 hitbox drag area
			-->

			<DragArea :buttons="['right']" active-cursor="cursor-move" @drag-update="updatePanning" @click="onClick"
				class="flex-1 graph-paper h-full pointer-events-auto" :style="graphPaperStyle" />
			<Frame v-for="k in allAnimationKeyframes" :key="k.keyframe.key" :keyframe="k.keyframe"
				:direction="k.direction" :index="k.index" :origin="origin" />
			<Hitbox v-for="hitbox, name in app.currentKeyframe?.hitboxes" :key="hitbox.key" :hitbox="hitbox"
				:name="name" :origin="origin" />
			<Origin :origin="origin" />

			<div class="absolute right-0 top-0 p-6">
				<div class="flex flex-col space-y-6">
					<div class="flex flex-col space-y-2">
						<div class="text-4xl p-1 px-2 font-bold bg-neutral-500 text-neutral-900">{{
								app.currentAnimation?.name
						}}</div>
						<div class="text-medium p-1 px-2 text-xs font-bold bg-neutral-900 text-neutral-500">
							Frame 2 / 4
						</div>
					</div>
					<div
						class="h-72 w-80 z-50 basis-72 bg-plastic-900 outline outline-plastic-600 rounded-md shadow-lg shadow-black/30 p-1">
						<div class="h-full flex flex-col">
							<div class="flex flex-row bg-plastic-900 font-medium text-xs ">
								<div class="p-3 px-4 pb-2 bg-plastic-700 rounded-t-md text-plastic-200">Hitboxes
								</div>
								<div class="p-3 px-4 pb-2 rounded-t-md text-plastic-400">FooBar</div>
							</div>
							<div class="flex-1 flex flex-col rounded-md rounded-tl-none bg-plastic-700">
								<PaneInset class="flex-1 m-4">
									<div class="p-4 overflow-y-auto h-full styled-scrollbars">
										<div class="text-plastic-200 flex flex-col">
											<Selectable text="hit" :selected="true" left-icon="CollectionIcon" />
											<Selectable text="weak" :selected="false" left-icon="CollectionIcon" />
											<Selectable text="strong" :selected="false" left-icon="CollectionIcon" />
										</div>
									</div>
								</PaneInset>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</Pane>
</template>

<script setup lang="ts">
import { onUnmounted, watch } from "vue"
import { computed, Ref, ref } from "@vue/reactivity"
import { Direction, Keyframe } from "@/api/dto"
import { closeDocument, focusDocument } from "@/api/app"
import { clearSelection, pan, zoomInWorkbench, zoomOutWorkbench } from "@/api/document"
import { useAppStore } from "@/stores/app"
import Button from "@/components/basic/Button.vue"
import DragArea, { DragAreaEvent } from "@/components/basic/DragArea.vue"
import Pane from "@/components/basic/Pane.vue"
import PaneInset from "@/components/basic/PaneInset.vue"
import PaneTab from "@/components/basic/PaneTab.vue"
import PaneTabList from "@/components/basic/PaneTabList.vue"
import Frame from "@/components/workbench/Frame.vue"
import Hitbox from "@/components/workbench/Hitbox.vue"
import Origin from "@/components/workbench/Origin.vue"
import Selectable from "@/components/basic/Selectable.vue"
import MultiSwitch, { MultiSwitchItem } from "@/components/basic/MultiSwitch.vue"
import { CollectionIcon, LocationMarkerIcon } from "@heroicons/vue/outline"

const app = useAppStore();
const drawingArea: Ref<HTMLElement | null> = ref(null);
const drawingAreaSize = ref([0, 0]);

const items: MultiSwitchItem[] = [
	{
		icon: "ViewGridIcon"
	},
	{
		icon: "ViewGridIcon",
		active: true,
	}
];

const resizeObserver = new ResizeObserver(entries => {
	for (let entry of entries) {
		if (entry.target === drawingArea.value) {
			drawingAreaSize.value = [entry.contentRect.width, entry.contentRect.height];
		}
	}
});

watch(drawingArea, (newArea, oldArea) => {
	if (oldArea) {
		resizeObserver.unobserve(oldArea);
	}
	if (newArea) {
		resizeObserver.observe(newArea);
	}
});

onUnmounted(() => {
	resizeObserver.disconnect();
});

const graphPaperStyle = computed(() => {
	const workbenchOffset = app.currentDocument?.workbenchOffset || [0, 0];
	const left = Math.floor(drawingAreaSize.value[0] / 2) + workbenchOffset[0];
	const top = Math.floor(drawingAreaSize.value[1] / 2) + workbenchOffset[1];
	return {
		"background-position": `${left}px ${top}px`,
	}
});

const origin = computed((): [number, number] => {
	const workbenchOffset = app.currentDocument?.workbenchOffset || [0, 0];
	return [
		Math.floor(drawingAreaSize.value[0] / 2) + workbenchOffset[0],
		Math.floor(drawingAreaSize.value[1] / 2) + workbenchOffset[1],
	];
})

const allAnimationKeyframes = computed((): { direction: Direction, index: number, keyframe: Keyframe }[] => {
	let keyframes = [];
	for (const direction in app.currentAnimation?.sequences) {
		for (const [index, keyframe] of (app.currentAnimation?.sequences[direction as Direction].keyframes.entries()) || []) {
			keyframes.push({
				direction: direction as Direction,
				index: index,
				keyframe: keyframe
			});
		}
	}
	return keyframes;
});

function onClick() {
	clearSelection();
}

function updatePanning(event: DragAreaEvent) {
	pan([event.mouseEvent.movementX, event.mouseEvent.movementY]);
}
</script>

<style scoped>
.graph-paper {
	background:
		linear-gradient(-90deg, theme("colors.neutral.700") 1px, transparent 1px),
		linear-gradient(0deg, theme("colors.neutral.700") 1px, transparent 1px),
		linear-gradient(-90deg, theme("colors.neutral.800") 1px, transparent 1px),
		linear-gradient(0deg, theme("colors.neutral.800") 1px, transparent 1px),
		theme("colors.neutral.900");
	background-size:
		128px 128px,
		128px 128px,
		16px 16px,
		16px 16px;
}
</style>
