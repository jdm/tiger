<template>
	<button type="button" class="w-20 min-w-fit px-4 py-2 rounded-md text-sm upper font-medium "
		:class="dynamicClasses">{{ label }}</button>
</template>

<script setup lang="ts">import { computed } from 'vue';

const props = defineProps<{
	label: string,
	positive?: boolean,
	danger?: boolean,
}>();

const dynamicClasses = computed(() => {
	return {
		...palette.value,
	}
});

const palette = computed(() => {
	if (props.danger) {
		return { "text-white": true, "bg-red-500": true, "border-y": true, "border-t-red-400": true, "border-b-red-700": true };
	} else if (props.positive) {
		return { "text-white": true, "bg-green-500": true, "border-y": true, "border-t-green-400": true, "border-b-green-700": true };
	}
	return { "text-black": true, "bg-gray-300": true, "border-y": true, "border-t-gray-100": true, "border-b-gray-400": true };
});
</script>