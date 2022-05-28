<template>
	<button type="button"
		class="w-20 min-w-fit px-4 py-1.5 rounded-md text-sm upper font-medium border-t border-b-2 ring-2"
		:class="dynamicClasses">{{ label }}</button>
</template>

<script setup lang="ts">import { computed } from 'vue';

const props = defineProps<{
	label: string,
	positive?: boolean,
	danger?: boolean,
}>();

const dynamicClasses = computed(() => {
	return [
		...palette.value,
	]
});

const palette = computed(() => {
	if (props.danger) {
		return ["text-red-100", "bg-red-600", "border-t-red-500", "border-b-red-900", "ring-plastic-900"];
	} else if (props.positive) {
		return ["text-green-100", "bg-green-600", "border-t-green-500", "border-b-green-900", "ring-plastic-900"];
	}
	return ["text-plastic-200", "bg-plastic-500", "border-t-plastic-400", "border-b-plastic-700", "ring-plastic-900"];
});
</script>