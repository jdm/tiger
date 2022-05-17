<template>
	<div @click="(event) => onAnimationClicked(event)" @dblclick="onAnimationDoubleClicked" @mouseover="onMouseOver"
		@mouseout="onMouseOut" class="px-4 py-1 inline-flex items-center space-x-4 cursor-pointer" :class="classes">
		<div class="flex-1 space-x-1">
			<DocumentIcon class="inline w-7 p-1.5 -mt-1" :class="decorativeIconClasses" />
			<span class="flex-1 overflow-x-hidden text-ellipsis">{{ animation.name }}</span>
		</div>
		<div>
			<PencilAltIcon class="inline w-7 p-1.5 rounded-lg hover:visible" :class="interactiveIconClasses" />
			<XIcon class="inline w-7 p-1.5 rounded-lg hover:visible" :class="interactiveIconClasses" />
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed, ref } from '@vue/reactivity';
import { DocumentIcon, PencilAltIcon, XIcon } from '@heroicons/vue/solid'
import { Animation as AnimationDTO } from '@/api/dto'
import { editAnimation, selectAnimation } from '@/api/document'

const props = defineProps<{
	animation: AnimationDTO
}>();

const hovered = ref(false);

const classes = computed(() => ({
	...(props.animation.selected ? { 'text-blue-100': true, 'bg-blue-600': true } : { 'text-plastic-200': true, 'hover:bg-plastic-600': true }),
}));

const decorativeIconClasses = computed(() => ({
	...(props.animation.selected ? { 'text-blue-200': true } : { 'text-plastic-300': true }),
}));

const interactiveIconClasses = computed(() => ({
	...(props.animation.selected ? { 'text-blue-200': true, 'hover:bg-blue-900': true } : { 'text-plastic-300': true, 'hover:bg-plastic-900': true }),
	...(hovered.value ? { 'visible': true } : { 'invisible': true })
}));

function onMouseOver() {
	hovered.value = true;
}

function onMouseOut() {
	hovered.value = false;
}

function onAnimationClicked(event: MouseEvent) {
	selectAnimation(props.animation.name, event.shiftKey, event.ctrlKey)
}

function onAnimationDoubleClicked() {
	editAnimation(props.animation.name);
}
</script>
