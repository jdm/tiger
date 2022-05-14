<template>
	<li @click="$emit('select')" @mouseover="onMouseOver" @mouseout="onMouseOut"
		class="inline-block py-3 px-3 pb-1 cursor-pointer" :class="selected ? selectedClasses : unselectedClasses">
		<slot></slot>
		<XIcon @click="$emit('close')" :class="(selected || hovered) ? 'visible' : 'invisible'"
			class="inline h-7 w-7 p-1 ml-2 rounded-lg hover:bg-neutral-800 hover:visible" />
	</li>
</template>

<script setup lang="ts">
import { XIcon } from '@heroicons/vue/solid'
import { ref } from '@vue/reactivity';
const selectedClasses = " text-blue-300 border-b-2 border-blue-300";
const unselectedClasses = "text-slate-400 hover:border-b-2 border-gray-400";
defineProps<{
	selected?: Boolean
}>()

let hovered = ref(false);

function onMouseOver() {
	hovered.value = true;
}

function onMouseOut() {
	hovered.value = false;
}
</script>