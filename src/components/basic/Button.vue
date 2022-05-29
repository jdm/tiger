<template>
	<button type="button"
		class="flex flex-row items-center justify-center space-x-2 px-4 py-1.5 rounded-md text-sm font-medium border-t border-b-2 ring-2"
		:class="dynamicClasses">
		<Icon v-if="icon" :name="icon" class="w-6 h-6" />
		<div v-if="label">{{ label }}</div>
	</button>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import Icon from '@/components/basic/Icon.vue';
import * as solid from '@heroicons/vue/solid';

const props = defineProps<{
	label?: string,
	positive?: boolean,
	danger?: boolean,
	icon?: keyof typeof solid,
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