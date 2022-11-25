<template>
	<div @click="$emit('select')" @mouseover="onMouseOver" @mouseout="onMouseOut" @auxclick="onMiddleClick"
		class="flex-initial h-11 px-4 flex items-center font-medium text-sm cursor-pointer" :class="classes">
		<slot></slot>
		<XMarkIcon v-if="closeable" @click="$emit('close')" :class="(selected || hovered) ? 'visible' : 'invisible'"
			class="inline w-8 p-1.5 mx-2 rounded-lg hover:bg-plastic-900 hover:visible" />
	</div>
</template>


<script setup lang="ts">
import { XMarkIcon } from "@heroicons/vue/20/solid"
import { computed, ref } from "@vue/reactivity"

const props = defineProps<{
	closeable?: Boolean
	selected?: Boolean
}>();

const emit = defineEmits(["select", "close"]);

const classes = computed(() => ([
	...(props.selected ? ["bg-plastic-700", "text-plastic-200"] : ["bg-plastic-800", "text-plastic-400", "hover:text-plastic-300"]),
	...(props.closeable) ? ["pr-0", "h-11"] : [],
]));

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
