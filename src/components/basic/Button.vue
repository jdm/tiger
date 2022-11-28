<template>
	<div class="h-11 rounded-md" :class="containerClasses">
		<button type="button"
			class="w-full h-full flex items-center justify-center space-x-2 px-3 py-1.5 rounded-md text-sm font-medium border-t border-b-2"
			:disabled="disabled" :class="buttonClasses" @click="emit('click')">
			<component :is="icon" v-if="icon" class="w-5" />
			<div v-if="label">{{ label }}</div>
		</button>
	</div>
</template>

<script setup lang="ts">
import type { Component } from "vue"
import { computed } from "vue"

type ButtonColor = "pink";

const props = defineProps<{
	label?: string,
	disabled?: boolean,
	positive?: boolean,
	danger?: boolean,
	customColor?: ButtonColor,
	icon?: Component,
}>();

const emit = defineEmits<{
	(e: 'click'): void
}>();

const containerClasses = computed(() => [...outline.value,]);
const buttonClasses = computed(() => [...palette.value,]);

const outline = computed(() => {
	if (props.disabled){
		return ["border-2", "border-plastic-600"];
	}
	return ["border-2", "border-plastic-900"];
});

const palette = computed(() => {
	if (props.disabled){
		return ["text-plastic-600", "bg-plastic-800", "border-none"];
	} else if (props.customColor == "pink") {
		return ["text-pink-100", "bg-gradient-to-b", "from-pink-800", "to-pink-600", "border-t-pink-600", "border-b-pink-900"];
	} else if (props.danger) {
		return ["text-red-100", "bg-gradient-to-b", "from-red-800", "to-red-600", "border-t-red-600", "border-b-red-900"];
	} else if (props.positive) {
		return ["text-green-100", "bg-gradient-to-b", "from-green-800", "to-green-600", "border-t-green-600", "border-b-green-900"];
	}
	return ["text-plastic-200", "bg-gradient-to-b", "from-plastic-600", "to-plastic-500", "border-t-plastic-500", "border-b-plastic-700"];
});
</script>