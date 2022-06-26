<template>
	<!-- This intentionally uses SVG and not a plain div. We must use transform:scale() to size
		elements (for perf reasons and also because width/height animates one frame behind).
		However, transform:scale() also applies to borders, with no way of scaling back to 1px borders
		when zoomed in.
	-->
	<svg class="absolute" :style="style" :viewBox="`0 0 ${size[0] + 1} ${size[1] + 1}`">
		<rect :x="1" :y="1" :width="size[0]" :height="size[1]" shape-rendering="crispEdges" :stroke-width="scale"
			:class="colorClasses" class="ease-in-out duration-150" style="transitionProperty: stroke-width" />
	</svg>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { useAppStore } from "@/stores/app"

const app = useAppStore();

const props = defineProps<{
	position: [number, number],
	size: [number, number],
	colorClasses: string | string[],
}>();

const zoom = computed(() => app.currentDocument?.workbenchZoom || 1);
const scale = computed(() => 1 / zoom.value);

const style = computed(() => {
	return {
		transform: `translate(${props.position[0] - 1}px, ${props.position[1] - 1}px)`,
		width: `${props.size[0] + 1}px`,
		height: `${props.size[1] + 1}px`,
	};
});
</script>