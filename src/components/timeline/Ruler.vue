<template>
	<div class="h-6 px-2 bg-plastic-600">
		<DragArea button="left" inactive-cursor="cursor-pointer" active-cursor="cursor-pointer"
			class="h-full ruler transition-all" :style="rulerStyle" @drag-start="startScrub" @drag-end="endScrub"
			@drag-update="updateScrub" />
	</div>
</template>

<script setup lang="ts">
import { useAppStore } from "@/stores/app"
import { computed } from "@vue/reactivity"
import { scrubTimeline } from "@/api/document"
import DragArea, { DragAreaEvent } from "@/components/basic/DragArea.vue"

const props = defineProps<{
	scrubbing: boolean,
	animate: boolean,
}>();

const emit = defineEmits(["update:scrubbing"]);

const app = useAppStore();

const rulerStyle = computed(() => {
	const zoom = app.currentDocument?.timelineZoomFactor || 1;
	const tenMsSize = Math.round(10 * zoom);
	const tenMsTicks = `${tenMsSize}px 4px`;
	const hundredMsTicks = `${10 * tenMsSize}px 10px`;
	const secondTicks = `${100 * tenMsSize}px 100%`;
	return {
		transitionProperty: props.animate ? "background-size" : "none",
		backgroundSize: [secondTicks, hundredMsTicks, tenMsTicks].join()
	};
});

function startScrub(e: DragAreaEvent) {
	emit("update:scrubbing", true);
	updateScrub(e);
}

function endScrub(e: DragAreaEvent) {
	emit("update:scrubbing", false);
}

function updateScrub(event: DragAreaEvent) {
	const rulerStartX = event.htmlElement.getBoundingClientRect().left;
	const zoom = app.currentDocument?.timelineZoomFactor || 1;
	const newTime = Math.max(0, Math.round((event.mouseEvent.clientX - rulerStartX) / zoom));
	scrubTimeline(newTime);
}
</script>

<style scoped>
.ruler {
	background:
		/* 1s ticks */
		linear-gradient(90deg, theme("colors.plastic.400") 1px, transparent 1px) left bottom repeat-x,
		/* 100ms ticks */
		linear-gradient(90deg, theme("colors.plastic.400") 1px, transparent 1px) left bottom repeat-x,
		/* 10ms tick */
		linear-gradient(90deg, theme("colors.plastic.400") 1px, transparent 1px) left bottom repeat-x,
		/* Solig BG */
		theme("colors.plastic.600") repeat-x;
	background-origin: content-box;
}
</style>