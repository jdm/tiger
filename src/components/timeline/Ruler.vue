<template>
	<div class="h-6 px-1 ruler transition-[background-size]" :style="rulerStyle" />
</template>

<script setup lang="ts">
import { useAppStore } from '@/stores/app'
import { computed } from '@vue/reactivity';

const app = useAppStore();

const rulerStyle = computed(() => {
	const zoom = app.currentDocument?.timelineZoom || 1;
	const secondTicks = (1000 * zoom) + "px 100%";
	const hundredMsTicks = (100 * zoom) + "px 10px";
	const msTicks = (10 * zoom) + "px 4px";
	const mainBG = "100% 100%";
	return {
		backgroundSize: [secondTicks, hundredMsTicks, msTicks, mainBG].join()
	};
});
</script>

<style scoped>
.ruler {
	background:
		/* 1s ticks */
		linear-gradient(90deg, theme('colors.plastic.400') 1px, transparent 1px) left bottom repeat-x,
		/* 100ms ticks */
		linear-gradient(90deg, theme('colors.plastic.400') 1px, transparent 1px) left bottom repeat-x,
		/* 10ms tick */
		linear-gradient(90deg, theme('colors.plastic.400') 1px, transparent 1px) left bottom repeat-x,
		/* Solig BG */
		theme('colors.plastic.600') repeat-x;
	background-origin: content-box;
}
</style>