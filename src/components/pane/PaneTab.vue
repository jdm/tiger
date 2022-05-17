<template>
	<div @click="$emit('select')" @mouseover="onMouseOver" @mouseout="onMouseOut" @auxclick="onMiddleClick"
		class="flex-initial px-4 py-3 font-medium text-sm" :class="classes">
		<slot></slot>
		<XIcon v-if="closeable" @click="$emit('close')" :class="(selected || hovered) ? 'visible' : 'invisible'"
			class="inline w-7 p-1.5 ml-2 rounded-lg hover:bg-plastic-900 hover:visible" />
	</div>
</template>


<script setup lang="ts">
import { XIcon } from '@heroicons/vue/solid'
import { computed, ref } from '@vue/reactivity';
const props = defineProps<{
	closeable?: Boolean
	selected?: Boolean
}>();

const emit = defineEmits(['select', 'close']);

const classes = computed(() => ({
	...(props.selected ? { 'bg-plastic-700': true, 'text-plastic-200': true } : { 'bg-plastic-800': true, 'text-plastic-400': true, 'hover:text-plastic-300': true }),
	...(props.closeable) ? { 'pr-2': true, 'pt-2': true, 'h-11': true } : {},
}));

let hovered = ref(false);

function onMouseOver() {
	hovered.value = true;
}
function onMouseOut() {
	hovered.value = false;
}

function onMiddleClick(event: MouseEvent) {
	if (props.closeable && event.button == 1) {
		emit("close");
	}
}
</script>
