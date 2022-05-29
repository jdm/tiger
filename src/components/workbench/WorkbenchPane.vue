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
			<DragArea button="right" active-cursor="cursor-move" @drag-update="updatePanning" @click="onClick"
				class="flex-1 graph-paper h-full" :style="graphPaperStyle" />
			<div class="pointer-events-none">
				<img v-for="keyframe, index in app.currentSequence?.keyframes" :key="index + '_' + keyframe.frame"
					ref="frameRefs" :src="convertFileSrc(keyframe.frame)"
					@load="onFrameLoaded(convertFileSrc(keyframe.frame))"
					class="absolute pixelated transition-transform"
					:class="keyframe == app.currentKeyframe ? 'opacity-100' : 'opacity-0'"
					:style="frameStyle(keyframe)" />
				<Origin class="absolute" :style="originStyle" />
				<div class="absolute right-0 bottom-0 p-6 text-4xl font-bold text-neutral-600">
					{{ app.currentAnimation?.name }}
				</div>
			</div>
		</div>
	</Pane>
</template>

<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/tauri'
import { onUnmounted, watch } from 'vue';
import { computed, Ref, ref } from '@vue/reactivity';
import { closeDocument, focusDocument } from '@/api/app'
import { Keyframe } from '@/api/dto'
import { clearSelection, pan, zoomInWorkbench, zoomOutWorkbench } from '@/api/document'
import { useAppStore } from '@/stores/app'
import Button from '@/components/basic/Button.vue'
import DragArea, { DragAreaEvent } from '@/components/basic/DragArea.vue'
import Pane from '@/components/basic/Pane.vue'
import PaneTab from '@/components/basic/PaneTab.vue'
import PaneTabList from '@/components/basic/PaneTabList.vue'
import Origin from '@/components/workbench/Origin.vue'

const app = useAppStore();
const drawingArea: Ref<HTMLElement | null> = ref(null);
const drawingAreaSize = ref([0, 0]);
const frameRefs: Ref<HTMLImageElement[]> = ref([]);
const frameSizes: Ref<Record<string, [number, number]>> = ref({});

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

function onFrameLoaded(source: string) {
	for (let frameRef of frameRefs.value) {
		if (frameRef.src == source) {
			frameSizes.value[source] = [frameRef.naturalWidth, frameRef.naturalHeight];
		}
	}
}

const graphPaperStyle = computed(() => {
	const workbenchOffset = app.currentDocument?.workbenchOffset || [0, 0];
	const left = Math.floor(drawingAreaSize.value[0] / 2) + workbenchOffset[0];
	const top = Math.floor(drawingAreaSize.value[1] / 2) + workbenchOffset[1];
	return {
		'background-position': left + 'px ' + top + 'px',
	}
});

const origin = computed(() => {
	const workbenchOffset = app.currentDocument?.workbenchOffset || [0, 0];
	return [
		Math.floor(drawingAreaSize.value[0] / 2) + workbenchOffset[0],
		Math.floor(drawingAreaSize.value[1] / 2) + workbenchOffset[1],
	];
})

function frameStyle(keyframe: Keyframe) {
	const zoom = app.currentDocument?.workbenchZoom || 1;
	const source = convertFileSrc(keyframe.frame);
	const frameSize = frameSizes.value[source] || [0, 0];
	const left = origin.value[0] - Math.floor(frameSize[0] / 2) + keyframe.offset[0];
	const top = origin.value[1] - Math.floor(frameSize[1] / 2) + keyframe.offset[1];
	const transformOrigin = [origin.value[0] - left, origin.value[1] - top];
	return {
		left: left + "px",
		top: top + "px",
		width: frameSize + "px",
		height: frameSize + "px",
		transform: "scale(" + zoom + "," + zoom + ")",
		transformOrigin: transformOrigin[0] + "px " + transformOrigin[1] + "px",
	};
}

const originStyle = computed(() => {
	return {
		left: origin.value[0] + "px",
		top: origin.value[1] + "px",
	};
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
