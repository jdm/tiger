<template>
	<!-- Mini icon at full size for extra chonk -->
	<Icon @click="onClicked" :name="icon" mini class="w-9 h-9 p-1.5 cursor-pointer rounded-md border-2"
		:class="iconClass" />
</template>

<script setup lang="ts">
import { computed } from "vue"
import * as solid from "@heroicons/vue/24/solid"
import Icon from "@/components/basic/Icon.vue"

export type ToggleColor = "blue" | "orange" | "pink" | "sky";

const props = defineProps<{
	icon: keyof typeof solid,
	color?: ToggleColor,
	modelValue: boolean,
}>();

const emit = defineEmits<{
	(e: "update:modelValue", toggled: boolean): void
}>();

const iconClass = computed(() => {
	if (!props.modelValue) {
		return [
			"text-plastic-400",
			"bg-plastic-700",
			"border-plastic-700",
		];
	}
	if (props.color == "orange") {
		return ["text-orange-600", "bg-plastic-900", "border-orange-800"];
	} else if (props.color == "pink") {
		return ["text-pink-600", "bg-plastic-900", "border-pink-800"];
	} else if (props.color == "sky") {
		return ["text-sky-600", "bg-plastic-900", "border-sky-800"];
	}
	return ["text-blue-600", "bg-plastic-900", "border-blue-800"];
});

function onClicked() {
	emit("update:modelValue", !props.modelValue);
}
</script>
