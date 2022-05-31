<template>
	<div class="rounded-md border-2 border-plastic-900">
		<div @click="onClicked" class="py-1.5 px-2 cursor-pointer" :class="dynamicClasses">
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
			["text-blue-600", "border-2", "rounded-md", "border-blue-600", "bg-zinc-900"]
			: ["text-zinc-500", "border-2", "rounded-md", "border-plastic-500", "bg-plastic-800"]),
	];
});

function onClicked() {
	emit("toggled", !props.toggled);
}
</script>
