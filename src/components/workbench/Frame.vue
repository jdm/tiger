<template>
	<img ref="el" :src="convertFileSrc(keyframe.frame)" @load="onFrameLoaded"
		class="absolute pixelated transition-transform" :style="frameStyle(keyframe)" />
</template>

<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/tauri'
import { Keyframe } from '@/api/dto'
import { useAppStore } from '@/stores/app'
import { ref, Ref } from 'vue';

const app = useAppStore();

const props = defineProps<{
	keyframe: Keyframe,
	origin: [number, number]
}>();

const el: Ref<HTMLImageElement | null> = ref(null);
const frameSize: Ref<[number, number] | null> = ref(null);

function onFrameLoaded() {
	if (!el.value) {
		return;
	}
	frameSize.value = [el.value.naturalWidth, el.value.naturalHeight];
}

function frameStyle(keyframe: Keyframe) {
	const zoom = app.currentDocument?.workbenchZoom || 1;
	const opacity = frameSize.value && keyframe == app.currentKeyframe ? 1 : 0;
	const size: [number, number] = frameSize.value || [0, 0];
	const left = props.origin[0] - Math.floor(size[0] / 2) + keyframe.offset[0];
	const top = props.origin[1] - Math.floor(size[1] / 2) + keyframe.offset[1];
	const transformOrigin = [props.origin[0] - left, props.origin[1] - top];
	return {
		opacity: opacity,
		left: left + "px",
		top: top + "px",
		transform: "scale(" + zoom + "," + zoom + ")",
		transformOrigin: transformOrigin[0] + "px " + transformOrigin[1] + "px",
	};
}
</script>