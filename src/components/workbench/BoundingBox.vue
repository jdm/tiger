<template>
	<!-- This intentionally uses SVG and not a plain div. We must use transform:scale() to size
		elements (for perf reasons and also because width/height animates one frame behind).
		However, transform:scale() also applies to borders, with no way of scaling back to 1px borders
		when zoomed in.
	-->
	<div class="absolute">
		<svg class="absolute" :style="style" :viewBox="`0 0 ${size[0] + 2} ${size[1] + 2}`">
			<rect :x="1" :y="1" :width="Math.max(1, size[0])" :height="Math.max(1, size[1])"
				shape-rendering="crispEdges" :stroke-width="scale" :class="colorClasses"
				class="ease-in-out duration-150" style="transitionProperty: stroke-width" />
		</svg>
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { useStateStore } from "@/stores/state"

const state = useStateStore();

const props = defineProps<{
	position: [number, number],
	size: [number, number],
	colorClasses: string | string[],
}>();

const zoom = computed(() => state.currentDocument?.workbenchZoom || 1);
const scale = computed(() => 1 / zoom.value);

const style = computed(() => {
	return {
		transform: `translate(${props.position[0] - 1}px, ${props.position[1] - 1}px)`,
		width: `${props.size[0] + 2}px`,
		height: `${props.size[1] + 2}px`,
	};
});
</script>