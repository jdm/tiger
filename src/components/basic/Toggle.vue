<template>
	<div class="rounded-md border-2 border-plastic-900">
		<div @click="onClicked" class="py-1.5 px-2 rounded-md border-2 cursor-pointer" :class="dynamicClasses">
			<Icon :name="icon" class="w-6 -mb-px" />
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import * as solid from "@heroicons/vue/solid"
import Icon from "@/components/basic/Icon.vue"

const props = defineProps<{
	icon: keyof typeof solid,
	toggled: boolean,
}>();

const emit = defineEmits<{
	(e: "toggled", newValue: boolean): void
}>();

const dynamicClasses = computed(() => {
	return [
		...(props.toggled ?
			[
				"text-blue-200",
				"border-blue-600", "border-t-blue-500",
				"bg-gradient-to-b", "from-blue-900", "to-blue-700"
			]
			: [
				"text-zinc-700",
				"border-zinc-700",
				"bg-zinc-900"
			]),
	];
});

function onClicked() {
	emit("toggled", !props.toggled);
}
</script>
