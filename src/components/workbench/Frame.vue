<template>
	<div>
		<!-- Sprite -->
		<img ref="el" :src="convertFileSrc(keyframe.frame)" @load="onFrameLoaded"
			class="absolute pixelated transition-transform z-10"
			:class="(frameSize && isActiveFrame) ? 'opacity-100' : 'opacity-0'" :style="frameStyle" />
		<!-- Selection indicator -->
		<!-- This intentionally uses SVG and not a plain div. We must use transform:scale() to size
			elements (for perf reasons and also because width/height animates one frame behind).
			However, transform:scale() also applies to borders, with no way of scaling back to 1px borders
			when zoomed in.
		 -->
		<svg v-if="frameSize && props.keyframe.selected && !app.currentDocument?.timelineIsPlaying"
			class="absolute transition-transform z-20" :style="selectionStyle"
			:viewBox="'0 0 ' + (frameSize[0] + 1) + ' ' + (frameSize[1] + 1)">
			<rect :x="1" :y="1" :width="frameSize[0]" :height="frameSize[1]" shape-rendering="crispEdges"
				:stroke-width="1 / zoom" class="stroke-blue-600 fill-blue-600/10" />
		</svg>
	</div>
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

const zoom = computed(() => app.currentDocument?.workbenchZoom || 1);
const left = computed(() => props.origin[0] - Math.floor((frameSize.value?.[0] || 0) / 2) + props.keyframe.offset[0]);
const top = computed(() => props.origin[1] - Math.floor((frameSize.value?.[1] || 0) / 2) + props.keyframe.offset[1]);

const frameStyle = computed(() => {
	const size: [number, number] = frameSize.value || [0, 0];
	const transformOrigin = [props.origin[0] - left.value, props.origin[1] - top.value];
	return {
		left: left.value + "px",
		top: top.value + "px",
		width: size[0] + "px",
		height: size[1] + "px",
		transform: "scale(" + zoom.value + "," + zoom.value + ")",
		transformOrigin: transformOrigin[0] + "px " + transformOrigin[1] + "px",
	};
});

const selectionStyle = computed(() => {
	const size: [number, number] = frameSize.value || [0, 0];
	const transformOrigin = [props.origin[0] - left.value + 1, props.origin[1] - top.value + 1];
	return {
		left: (left.value - 1) + "px",
		top: (top.value - 1) + "px",
		width: (size[0] + 1) + "px",
		height: (size[1] + 1) + "px",
		transform: "scale(" + zoom.value + "," + zoom.value + ")",
		transformOrigin: transformOrigin[0] + "px " + transformOrigin[1] + "px",
	};
});

function onFrameLoaded() {
	if (!el.value) {
		return;
	}
	frameSize.value = [el.value.naturalWidth, el.value.naturalHeight];
}
</script>