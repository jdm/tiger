<template>
	<div class="h-6 px-1 bg-plastic-600">
		<div ref="el" class="h-full ruler transition-[background-size]" :style="rulerStyle" @mousedown="beginScrub" />
	</div>
</template>

<script setup lang="ts">
import { useAppStore } from '@/stores/app'
import { computed, Ref, ref } from '@vue/reactivity';
import { scrubTimeline } from '@/api/document'
import { onMounted, onUnmounted } from 'vue';

const props = defineProps<{
	scrubbing: boolean,
}>();

const emit = defineEmits(['update:scrubbing']);

const app = useAppStore();

const el: Ref<HTMLElement | null> = ref(null);

const rulerStyle = computed(() => {
	const zoom = app.currentDocument?.timelineZoom || 1;
	const secondTicks = (1000 * zoom) + "px 100%";
	const hundredMsTicks = (100 * zoom) + "px 10px";
	const msTicks = (10 * zoom) + "px 4px";
	return {
		backgroundSize: [secondTicks, hundredMsTicks, msTicks].join()
	};
});

onMounted(() => {
	endScrub();
	window.addEventListener("mouseup", endScrub)
})

onUnmounted(() => {
	window.removeEventListener("mouseup", endScrub)
})

function beginScrub(e: MouseEvent) {
	emit("update:scrubbing", true);
	window.addEventListener("mousemove", updateScrub);
	updateScrub(e);
}

function updateScrub(e: MouseEvent) {
	if (!props.scrubbing || !el.value) {
		return;
	}
	const rulerStartX = el.value.getBoundingClientRect().left;
	const zoom = app.currentDocument?.timelineZoom || 1;
	const newTime = Math.max(0, Math.round((e.clientX - rulerStartX) / zoom));
	scrubTimeline(newTime);
}

function endScrub() {
	emit("update:scrubbing", false);
	window.removeEventListener("mousemove", updateScrub);
}
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