<template>
	<div @click="$emit('select')" @mouseover="onMouseOver" @mouseout="onMouseOut"
		class="flex-initial px-4 py-3 font-medium text-sm" :class="classes">
		<slot></slot>
		<XIcon v-if="closeable" @click="$emit('close')" :class="(selected || hovered) ? 'visible' : 'invisible'"
			class=" inline w-8 p-2 ml-2 rounded-lg hover:bg-zinc-900 hover:visible" />
	</div>
</template>


<script setup lang="ts">
import { XIcon } from '@heroicons/vue/solid'
import { computed, ref } from '@vue/reactivity';
const props = defineProps<{
	closeable?: Boolean
	selected?: Boolean
}>()

const classes = computed(() => ({
	...(props.selected ? { 'bg-zinc-800': true, 'text-zinc-200': true } : { 'bg-zinc-900': true, 'text-zinc-500': true, 'hover:bg-zinc-800': true }),
	...(props.closeable) ? { 'pr-2': true, 'pt-2': true, 'h-11': true } : {},
}));

let hovered = ref(false);
function onMouseOver() {
	hovered.value = true;
}
function onMouseOut() {
	hovered.value = false;
}
</script>
