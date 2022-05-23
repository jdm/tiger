<template>
	<Pane class="flex flex-col overflow-hidden">
		<PaneTabList>
			<PaneTab :closeable="true" v-for="document in app.documents" @select="focusDocument(document.path)"
				@close="closeDocument(document.path)" :selected="document.path == app.currentDocumentPath">
				{{ document.name }}
			</PaneTab>
		</PaneTabList>
		<div class="w-full p-2 flex flex-row items-center space-x-2">
			<button @click="zoomInWorkbench"
				class="py-1 px-2 rounded-md uppercase text-xs text-gray-800 font-bold bg-gray-300 border-y border-t-gray-100 border-b-gray-900">
				<ZoomInIcon class="w-6" />
			</button>
			<button @click="zoomOutWorkbench"
				class="py-1 px-2 rounded-md uppercase text-xs text-gray-800 font-bold bg-gray-300 border-y border-t-gray-100 border-b-gray-900">
				<ZoomOutIcon class="w-6" />
			</button>
		</div>
		<div class="relative flex-1 overflow-hidden" ref="drawingArea">
			<div @click="onClick" @mousedown="onMouseDown" @mouseup="onMouseUp" @mousemove="onMouseMove"
				class="flex-1 graph-paper h-full" :class="isDragging ? 'cursor-move' : 'cursor-default'"
				:style="graphPaperStyle">
			</div>
			<img v-if="app.currentKeyframe" ref="frame" :src="convertFileSrc(app.currentKeyframe.frame)"
				class="absolute pixelated" :style="frameStyle" />
			<div class="absolute right-0 bottom-0 p-6 text-4xl font-bold text-neutral-600">
				{{ app.currentAnimation?.name }}
			</div>
		</div>
	</Pane>
</template>

<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/tauri'
import { computed, Ref, ref } from '@vue/reactivity';
import { closeDocument, focusDocument } from '@/api/app'
import { clearSelection, pan, zoomInWorkbench, zoomOutWorkbench } from '@/api/document'
import { useAppStore } from '@/stores/app'
import Pane from '@/components/basic/Pane.vue'
import PaneTab from '@/components/basic/PaneTab.vue'
import PaneTabList from '@/components/basic/PaneTabList.vue'
import { ZoomInIcon, ZoomOutIcon } from '@heroicons/vue/solid'
import { onUnmounted, watch } from 'vue';

const app = useAppStore();
const isDragging = ref(false);
const drawingArea: Ref<HTMLElement | null> = ref(null);
const frame: Ref<HTMLImageElement | null> = ref(null);
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

const frameStyle = computed(() => {
	const zoom = app.currentDocument?.workbenchZoom || 1;
	const workbenchOffset = app.currentDocument?.workbenchOffset || [0, 0];
	const keyframeOffset = app.currentKeyframe?.offset || [0, 0];
	const frameSize = [frame.value?.naturalWidth || 0, frame.value?.naturalHeight || 0];
	const drawSize = [frameSize[0] * zoom, frameSize[1] * zoom];
	const left = Math.floor(drawingAreaSize.value[0] / 2) + workbenchOffset[0] - Math.floor(drawSize[0] / zoom / 2) * zoom + (zoom * keyframeOffset[0]);
	const top = Math.floor(drawingAreaSize.value[1] / 2) + workbenchOffset[1] - Math.floor(drawSize[1] / zoom / 2) * zoom + (zoom * keyframeOffset[1]);

	return {
		left: left + "px",
		top: top + "px",
		width: drawSize[0] + "px",
		height: drawSize[1] + "px"
	};
});

function onClick() {
	clearSelection();
}

function onMouseDown(event: MouseEvent) {
	if (event.button == 2) {
		isDragging.value = true;
	}
}

function onMouseUp(event: MouseEvent) {
	if (event.button == 2) {
		isDragging.value = false;
	}
}

function onMouseMove(event: MouseEvent) {
	if (isDragging.value) {
		pan([event.movementX, event.movementY]);
	}
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
