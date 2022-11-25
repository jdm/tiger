<template>
	<div class="h-11 box-border rounded-md border-2 border-plastic-900">
		<div @click="onClicked" class="h-full flex rounded-md border-2 cursor-pointer" :class="dynamicClasses">
			<!-- Using mini icon at non-mini size for extra chonk-->
			<Icon :name="icon" mini class="w-6 mx-2" />
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import * as mini from "@heroicons/vue/20/solid"
import Icon from "@/components/basic/Icon.vue"

const props = defineProps<{
	icon: keyof typeof mini,
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
