<template>
	<Icon @click="onClicked" :name="icon" class="w-9 h-9 p-2 cursor-pointer rounded-md" :class="iconClass" />
</template>

<script setup lang="ts">
import { computed } from "vue"
import * as solid from "@heroicons/vue/solid"
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
			"bg-plastic-700"
		];
	}
	if (props.color == "orange") {
		return ["text-orange-200", "bg-orange-700"];
	} else if (props.color == "pink") {
		return ["text-pink-200", "bg-pink-700"];
	} else if (props.color == "sky") {
		return ["text-sky-200", "bg-sky-600"];
	}
	return ["text-blue-200", "bg-blue-600"];
});

function onClicked() {
	emit("update:modelValue", !props.modelValue);
}
</script>
