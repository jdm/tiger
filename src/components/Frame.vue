<template>
	<div class="flex flex-col rounded-sm cursor-pointer"
		:class="frame.selected ? 'outline outline-4 outline-blue-500' : ''">
		<div class="flex place-content-center aspect-square checkerboard rounded-sm overflow-hidden">
			<img :src="convertFileSrc(frame.path)" class="pixelated object-none" />
		</div>
		<div class="text-xs p-1 overflow-hidden text-ellipsis"
			:class="frame.selected ? 'bg-blue-500 text-white' : 'text-zinc-400'">{{ frame.name }}</div>
	</div>
</template>

<script setup lang="ts">
import { convertFileSrc } from '@tauri-apps/api/tauri'
import { Frame as FrameDTO } from '@/api/dto'
const props = defineProps<{
	frame: FrameDTO
}>()
</script>


<style scoped>
.pixelated {
	image-rendering: pixelated;
}

.checkerboard {
	background-size: 16px 16px;
	background-image:
		linear-gradient(45deg, theme('colors.neutral.700') 25%, transparent 25%, transparent 75%, theme('colors.neutral.700') 75%, theme('colors.neutral.700') 100%),
		linear-gradient(45deg, theme('colors.neutral.700') 25%, theme('colors.neutral.600') 25%, theme('colors.neutral.600') 75%, theme('colors.neutral.700') 75%, theme('colors.neutral.700') 100%);
	background-position:
		0px 0px,
		8px 8px;
}
</style>