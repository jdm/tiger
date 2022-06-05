<template>
	<Pane class="flex flex-col overflow-hidden">
		<PaneTabList>
			<PaneTab :closeable="true" v-for="document in app.documents" @select="focusDocument(document.path)"
				@close="closeDocument(document.path)" :selected="document.path == app.currentDocumentPath">
				{{ document.name + (document.hasUnsavedChanges ? "*" : "") }}
			</PaneTab>
		</PaneTabList>
		<div class="w-full p-2 flex flex-row items-center space-x-2">
			<Button @click="zoomInWorkbench" icon="ZoomInIcon" />
			<Button @click="zoomOutWorkbench" icon="ZoomOutIcon" />
		</div>
		<div class="relative flex-1 overflow-hidden" ref="drawingArea">
			<DragArea :buttons="['right']" active-cursor="cursor-move" @drag-update="updatePanning" @click="onClick"
				class="flex-1 graph-paper h-full" :style="graphPaperStyle" />
			<Frame v-for="keyframe in app.currentSequence?.keyframes" :key="keyframe.key" :keyframe="keyframe"
				:origin="origin" />
			<Origin :origin="origin" class="pointer-events-none" />
			<div class="absolute right-0 bottom-0 p-6 text-4xl font-bold text-neutral-600 pointer-events-none">
				{{ app.currentAnimation?.name }}
			</div>
		</div>
	</Pane>
</template>

<script setup lang="ts">
import { onUnmounted, watch } from 'vue';
import { computed, Ref, ref } from '@vue/reactivity';
import { closeDocument, focusDocument } from '@/api/app'
import { clearSelection, pan, zoomInWorkbench, zoomOutWorkbench } from '@/api/document'
import { useAppStore } from '@/stores/app'
import Button from '@/components/basic/Button.vue'
import DragArea, { DragAreaEvent } from '@/components/basic/DragArea.vue'
import Pane from '@/components/basic/Pane.vue'
import PaneTab from '@/components/basic/PaneTab.vue'
import PaneTabList from '@/components/basic/PaneTabList.vue'
import Frame from '@/components/workbench/Frame.vue'
import Origin from '@/components/workbench/Origin.vue'

const app = useAppStore();
const drawingArea: Ref<HTMLElement | null> = ref(null);
const drawingAreaSize = ref([0, 0]);

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
		'background-position': left + 'px ' + top + 'px',
	}
});

const origin = computed((): [number, number] => {
	const workbenchOffset = app.currentDocument?.workbenchOffset || [0, 0];
	return [
		Math.floor(drawingAreaSize.value[0] / 2) + workbenchOffset[0],
		Math.floor(drawingAreaSize.value[1] / 2) + workbenchOffset[1],
	];
})

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
		linear-gradient(-90deg, theme('colors.neutral.700') 1px, transparent 1px),
		linear-gradient(0deg, theme('colors.neutral.700') 1px, transparent 1px),
		linear-gradient(-90deg, theme('colors.neutral.800') 1px, transparent 1px),
		linear-gradient(0deg, theme('colors.neutral.800') 1px, transparent 1px),
		theme('colors.neutral.900');
	background-size:
		128px 128px,
		128px 128px,
		16px 16px,
		16px 16px;
}
</style>
