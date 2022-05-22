<template>
	<div
		class="flex min-w-0 items-center rounded-sm px-2 bg-amber-500 border-r border-amber-900 text-amber-900 text-xs font-semibold overflow-hidden">
		{{ keyframe.name }}
	</div>
</template>

<script setup lang="ts">
import { Keyframe as KeyframeDTO } from '@/api/dto'

defineProps<{
	keyframe: KeyframeDTO,
}>();
</script>