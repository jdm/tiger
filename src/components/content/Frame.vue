<template>
	<div @click="(event) => onFrameClicked(event)" @mouseover="onMouseOver" @mouseout="onMouseOut"
		@dragstart="onDragStart" draggable="true"
		class="aspect-square checkerboard flex place-content-center rounded-sm cursor-pointer overflow-hidden outline-offset-2"
		:class="props.frame.selected ? 'outline outline-blue-600' : 'hover:outline outline-plastic-500'">
		<img :src="convertFileSrc(frame.path)" class="pixelated object-none" />
	</div>
</template>

<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/tauri'
import { ref } from '@vue/reactivity';
import { Frame as FrameDTO } from '@/api/dto'
import { beginDragAndDropFrame, selectFrame } from '@/api/document'

const props = defineProps<{
	frame: FrameDTO
}>();

const hovered = ref(false);

function onMouseOver() {
	hovered.value = true;
}

function onMouseOut() {
	hovered.value = false;
}

function onDragStart() {
	beginDragAndDropFrame(props.frame.path);
}

function onFrameClicked(event: MouseEvent) {
	selectFrame(props.frame.path, event.shiftKey, event.ctrlKey)
}
</script>


<style scoped>
.checkerboard {
	background-size: 16px 16px;
	background-image:
		linear-gradient(45deg, theme('colors.plastic.700') 25%, transparent 25%, transparent 75%, theme('colors.plastic.700') 75%, theme('colors.plastic.700') 100%),
		linear-gradient(45deg, theme('colors.plastic.700') 25%, theme('colors.plastic.600') 25%, theme('colors.plastic.600') 75%, theme('colors.plastic.700') 75%, theme('colors.plastic.700') 100%);
	background-position:
		0px 0px,
		8px 8px;
}
</style>