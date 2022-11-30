<template>
	<div ref="el" class="h-11 relative box-border rounded-md border-2 border-plastic-900">
		<div @click="onClicked" class="h-full pl-2 space-x-1 flex items-center rounded-md border-2 cursor-pointer"
			:class="dynamicClasses">
			<component :is="icon" class="w-6" />
			<ChevronDownIcon v-if="canExpand" @click.stop="expand" class="w-8 h-8 p-1.5 -mb-0.5 rounded-lg "
				:class="chevronClasses" />
		</div>
		<FloatingWidget :open="expanded" :position="expansionPosition" @dismissed="close">
			<slot name="expanded" />
		</FloatingWidget>
	</div>
</template>

<script setup lang="ts">
import type { Component } from "vue"
import { computed, Ref, ref } from "vue"
import { ChevronDownIcon } from "@heroicons/vue/20/solid";
import FloatingWidget from "@/components/basic/FloatingWidget.vue";

const props = defineProps<{
	icon: Component,
	toggled: boolean,
	canExpand?: boolean,
}>();

const emit = defineEmits<{
	(e: "toggled", newValue: boolean): void
}>();

const el: Ref<HTMLElement | null> = ref(null);
const expanded = ref(false);
const expansionPosition = ref([0, 0] as [number, number]);

const chevronClasses = computed(() => [
	...(!expanded.value && !props.toggled) ? ["hover:bg-zinc-800", "hover:text-zinc-400"] : [],
	...(expanded.value && !props.toggled) ? ["text-zinc-200"] : [],
	...(!expanded.value && props.toggled) ? ["hover:bg-zinc-800/80", "hover:text-blue-200"] : [],
	...(expanded.value && props.toggled) ? ["bg-zinc-200", "text-zinc-800"] : [],
]);

const dynamicClasses = computed(() =>  [
	...props.canExpand ? ["pr-1"] : ["pr-2"],
	...props.toggled ?
		[
			"text-blue-200",
			"border-blue-600", "border-t-blue-500",
			"bg-gradient-to-b", "from-blue-900", "to-blue-700"
		]
		: [
			"text-zinc-700",
			"border-zinc-700",
			"bg-zinc-900"
		],
]);

function onClicked() {
	emit("toggled", !props.toggled);
}

function expand() {
	if (el.value) {
		const boundingBox = el.value.getBoundingClientRect();
		const x = (boundingBox.left + boundingBox.right) / 2;
		const y = boundingBox.bottom;
		expansionPosition.value = [x, y];
	}
	expanded.value = true;
}

function close() {
	expanded.value = false;
}
</script>
