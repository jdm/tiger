<template>
	<div class="rounded-md border-2 border-plastic-900">
		<button type="button"
			class="w-full h-full flex flex-row items-center justify-center space-x-2 px-4 py-1.5 rounded-md text-sm font-medium border-t border-b-2"
			:class="dynamicClasses">
			<Icon v-if="icon" :name="icon" class="w-6 h-6" />
			<div v-if="label">{{ label }}</div>
		</button>
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import Icon from "@/components/basic/Icon.vue"
import * as solid from "@heroicons/vue/solid"

type ButtonColor = "pink";

const props = defineProps<{
	label?: string,
	positive?: boolean,
	danger?: boolean,
	customColor?: ButtonColor,
	icon?: keyof typeof solid,
}>();

const dynamicClasses = computed(() => {
	return [
		...palette.value,
	]
});

const palette = computed(() => {
	if (props.customColor == "pink") {
		return ["text-pink-100", "bg-gradient-to-b", "from-pink-800", "to-pink-600", "border-t-pink-600", "border-b-pink-900"];
	} else if (props.danger) {
		return ["text-red-100", "bg-gradient-to-b", "from-red-800", "to-red-600", "border-t-red-600", "border-b-red-900"];
	} else if (props.positive) {
		return ["text-green-100", "bg-gradient-to-b", "from-green-800", "to-green-600", "border-t-green-600", "border-b-green-900"];
	}
	return ["text-plastic-200", "bg-gradient-to-b", "from-plastic-600", "to-plastic-500", "border-t-plastic-500", "border-b-plastic-700"];
});
</script>