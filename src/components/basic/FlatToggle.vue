<template>
	<Icon @click="onClicked" :name="icon" class="w-9 h-9 p-1.5 cursor-pointer rounded-md border-2" :class="iconClass" />
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
			"bg-plastic-700",
			"border-plastic-700",
		];
	}
	if (props.color == "orange") {
		return ["text-orange-400", "bg-orange-900", "border-orange-700"];
	} else if (props.color == "pink") {
		return ["text-pink-400", "bg-pink-900", "border-pink-700"];
	} else if (props.color == "sky") {
		return ["text-sky-400", "bg-sky-900", "border-sky-600"];
	}
	return ["text-blue-400", "bg-blue-900", "border-blue-600"];
});

function onClicked() {
	emit("update:modelValue", !props.modelValue);
}
</script>
