<template>
	<Pane class="flex flex-col overflow-hidden">
		<PaneTabList>
			<PaneTab :closeable="true" v-for="document in app.documents" @select="focusDocument(document.path)"
				@close="closeDocument(document.path)" :selected="document.path == app.currentDocumentPath">
				{{ document.name }}
			</PaneTab>
		</PaneTabList>
		<div class="relative flex-1">
			<div @click="onClick" @mousedown="onMouseDown" @mouseup="onMouseUp" @mousemove="onMouseMove"
				class="flex-1 graph-paper h-full" :class="isDragging ? 'cursor-move' : 'cursor-default'"
				:style="graphPaperStyle">
			</div>
			<div class="absolute right-0 bottom-0 p-6 text-4xl font-bold text-neutral-600">
				{{ app.currentAnimation?.name }}
			</div>
		</div>
	</Pane>
</template>

<script setup lang="ts">
import { computed, ref } from '@vue/reactivity';
import { closeDocument, focusDocument } from '@/api/app'
import { clearSelection, pan } from '@/api/document'
import { useAppStore } from '@/stores/app'
import Pane from '@/components/basic/Pane.vue'
import PaneTab from '@/components/basic/PaneTab.vue'
import PaneTabList from '@/components/basic/PaneTabList.vue'

const app = useAppStore();
const isDragging = ref(false);

const graphPaperStyle = computed(() => {
	const offset = app.currentDocument?.view.workbenchOffset || [0, 0];
	return {
		'background-position': offset[0] + 'px ' + offset[1] + 'px',
	}
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
