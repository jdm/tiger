<template>
	<div class="rounded-md border-2 border-plastic-900">
		<div @click="onClicked" class="py-1.5 px-2 rounded-md border-2 cursor-pointer" :class="dynamicClasses">
			<Icon :name="icon" class="w-6" />
		</div>
	</div>
</template>

<script setup lang="ts">
import * as solid from '@heroicons/vue/solid';
import Icon from '@/components/basic/Icon.vue'
import { computed } from 'vue';

const props = defineProps<{
	icon: keyof typeof solid,
	toggled: boolean,
}>();

const emit = defineEmits<{
	(e: 'toggled', newValue: boolean): void
}>();

const dynamicClasses = computed(() => {
	return [
		...(props.toggled ?
			[
				"text-emerald-200",
				"border-emerald-600", "border-t-emerald-500",
				"bg-gradient-to-b", "from-emerald-900", "to-emerald-700"
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
