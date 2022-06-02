<template>
	<div class="h-9 p-1 rounded-sm bg-plastic-800 border-y border-t-plastic-900 border-b-plastic-600">
		<div class="relative h-full">
			<Keyframe v-for="keyframe, index in sequence.keyframes" :keyframe="keyframe" :direction="direction"
				:index="index" :key="index + '_' + keyframe.name" :is-preview="index == 2"
				class="absolute h-full transition top-1/2 -translate-y-1/2" :style="keyframeStyle(keyframe)" />
		</div>
	</div>
</template>

<script setup lang="ts">
import { Direction, Keyframe as KeyframeDTO, Sequence as SequenceDTO } from '@/api/dto';
import { useAppStore } from '@/stores/app';
import Keyframe from '@/components/timeline/Keyframe.vue';

const app = useAppStore();

defineProps<{
	sequence: SequenceDTO,
	direction: Direction
}>();

function keyframeStyle(keyframe: KeyframeDTO) {
	const zoom = app.currentDocument?.timelineZoom || 1;
	return {
		"transitionProperty": app.currentDocument?.isDraggingKeyframeDuration ? "none" : "width, left",
		"left": (zoom * keyframe.startTimeMillis) + "px",
		"width": (zoom * keyframe.durationMillis) + "px"
	};
}
</script>