<template>
	<img ref="el" :src="convertFileSrc(keyframe.frame)" @load="onFrameLoaded" class="pixelated" :style="frameStyle" />
</template>

<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/tauri'
import { Keyframe } from '@/api/dto'
import { useAppStore } from '@/stores/app'
import { computed, CSSProperties, ref, Ref } from 'vue';

const app = useAppStore();

const props = defineProps<{
	keyframe: Keyframe,
	origin: [number, number]
}>();

const el: Ref<HTMLImageElement | null> = ref(null);
const frameSize: Ref<[number, number] | null> = ref(null);
const isActiveFrame = computed(() => props.keyframe == app.currentKeyframe);

const positioning = computed((): CSSProperties => {
	const zoom = app.currentDocument?.workbenchZoom || 1;
	const size: [number, number] = frameSize.value || [0, 0];
	const left = props.origin[0] - Math.floor(size[0] / 2) + props.keyframe.offset[0];
	const top = props.origin[1] - Math.floor(size[1] / 2) + props.keyframe.offset[1];
	const width = size[0];
	const height = size[1];
	const transformOrigin = [props.origin[0] - left, props.origin[1] - top];
	return {
		position: "absolute",
		left: left + "px",
		top: top + "px",
		width: width + "px",
		height: height + "px",
		transform: "scale(" + zoom + "," + zoom + ")",
		transformOrigin: transformOrigin[0] + "px " + transformOrigin[1] + "px",
	};
});

const frameStyle = computed(() => {
	return {
		...positioning.value,
		opacity: frameSize.value && isActiveFrame.value ? 1 : 0,
	};
});

function onFrameLoaded() {
	if (!el.value) {
		return;
	}
	frameSize.value = [el.value.naturalWidth, el.value.naturalHeight];
}
</script>