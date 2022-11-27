<template>
	<div @mouseover="onMouseOver" @mouseout="onMouseOut"
		class="px-2 py-1 flex flex-row items-center space-x-4 cursor-pointer"
		:class="selected ? 'text-blue-100 bg-blue-600' : 'text-plastic-300 hover:bg-plastic-600'">
		<component v-if="leftIcon" :is="leftIcon" class="inline w-5"
			:class="selected ? 'text-blue-200' : 'text-plastic-400'" />
		<div class="flex-1 min-w-0">
			<slot name="content">
				<div class="mb-0.5 whitespace-nowrap overflow-x-hidden text-ellipsis">{{ text }}</div>
			</slot>
		</div>
		<div v-if="actions && actions.length > 0" class="flex flex-row space-x-1">
			<component :is="action.icon" v-for="action in actions" @click.stop="action.callback"
				class="inline w-8 p-1.5 rounded-lg hover:visible" :class="interactiveIconClasses" />
		</div>
	</div>
</template>

<script setup lang="ts">
import type { Component } from "vue";
import { computed, ref } from "@vue/reactivity"

const props = defineProps<{
	selected: boolean,
	text: string,
	leftIcon?: Component,
	actions?: { icon: Component, callback: () => void }[],
}>();

const hovered = ref(false);

const interactiveIconClasses = computed(() => ([
	...(props.selected ? ["text-blue-200", "hover:bg-blue-900"] : ["text-plastic-300", "hover:bg-plastic-900"]),
	...(hovered.value ? ["visible"] : ["invisible"])
]));

function onMouseOver() {
	hovered.value = true;
}

function onMouseOut() {
	hovered.value = false;
}

</script>
