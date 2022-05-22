<template>
	<div
		class="w-full h-9 flex flex-row p-1 rounded-sm bg-plastic-800 border-y border-t-plastic-900 border-b-plastic-600">
		<Keyframe v-for="keyframe in sequence.keyframes" :keyframe="keyframe" />
	</div>
</template>


<script setup lang="ts">
import { Sequence as SequenceDTO } from '@/api/dto'
import Keyframe from '@/components/timeline/Keyframe.vue'

defineProps<{
	sequence: SequenceDTO,
}>();
</script>