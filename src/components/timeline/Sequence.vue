<template>
	<div class="h-9 flex flex-row p-1 rounded-sm bg-plastic-800 border-y border-t-plastic-900 border-b-plastic-600">
		<Keyframe v-for="keyframe, index in sequence.keyframes" :keyframe="keyframe" :direction="direction"
			:index="index" class="transition-[flex-basis]" :key="index + '_' + keyframe.name"
			:style="keyframeStyle(keyframe)" />
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
		"flex-basis": (zoom * keyframe.durationMillis) + "px"
	};
}
</script>