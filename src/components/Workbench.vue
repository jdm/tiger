<template>
	<div class="h-full flex flex-col">
		<div class="flex flex-row ">
			<PaneTab :closeable="true" v-for="document in app.documents" @select="focusDocument(document.path)"
				@close="closeDocument(document.path)" :selected="document.path == app.currentDocumentPath">
				{{ document.name }}
			</PaneTab>
			<div class="flex-1 bg-plastic-900" />
		</div>
		<div @mousedown="onMouseDown" @mouseup="onMouseUp" @mousemove="onMouseMove" class="flex-1 graph-paper"
			:class="isDragging ? 'cursor-move' : 'cursor-default'" :style="graphPaperStyle" />
	</div>
</template>

<script setup lang="ts">
import { computed, ref } from '@vue/reactivity';
import { closeDocument, focusDocument } from '@/api/app'
import { pan } from '@/api/document'
import { useAppStore } from '@/stores/app'
import PaneTab from '@/components/pane/PaneTab.vue'

const app = useAppStore();
const isDragging = ref(false);

const graphPaperStyle = computed(() => {
	const offset = app.currentDocument?.view.workbenchOffset || [0, 0];
	return {
		'background-position': offset[0] + 'px ' + offset[1] + 'px',
	}
});

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