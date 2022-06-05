<template>
	<div class="absolute z-50 w-[16px] h-[16px]" :style="originStyle()">
		<div class="relative w-full h-full ">
			<div class="absolute left-[7px] top-0 h-full w-[2px] bg-pink-400"></div>
			<div class="absolute top-[7px] left-0 w-full h-[2px] bg-pink-400"></div>
		</div>
	</div>
</template>

<script setup lang="ts">
const props = defineProps<{
	origin: [number, number]
}>();

function originStyle() {
	return {
		transform: "translate(-50%, -50%)",
		left: `${props.origin[0]}px`,
		top: `${props.origin[1]}px`,
	};
}
</script>
