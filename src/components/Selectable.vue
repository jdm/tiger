<template>
	<div @mouseover="onMouseOver" @mouseout="onMouseOut"
		class="px-4 py-1 inline-flex items-center space-x-4 cursor-pointer"
		:class="selected ? 'text-blue-100 bg-blue-600' : 'text-plastic-200 hover:bg-plastic-600'">
		<div class="flex-1 space-x-1">
			<Icon v-if="leftIcon" :name="leftIcon" class="inline w-7 p-1.5 -mt-1"
				:class="selected ? 'text-blue-200' : 'text-plastic-300'" />
			<span class="flex-1 overflow-x-hidden text-ellipsis">{{ text }}</span>
		</div>
		<div>
			<Icon v-for="action in actions" :name="action.icon" class="inline w-7 p-1.5 rounded-lg hover:visible"
				:class="interactiveIconClasses" />
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed, ref } from '@vue/reactivity';
import Icon from '@/components/Icon.vue';
import * as solid from '@heroicons/vue/solid';
import * as outline from '@heroicons/vue/outline';

const props = defineProps<{
	selected: boolean
	text: string
	leftIcon?: keyof typeof solid | keyof typeof outline
	actions?: { icon: keyof typeof solid | keyof typeof outline, callback: () => void }[],
}>();

const hovered = ref(false);

const interactiveIconClasses = computed(() => ({
	...(props.selected ? { 'text-blue-200': true, 'hover:bg-blue-900': true } : { 'text-plastic-300': true, 'hover:bg-plastic-900': true }),
	...(hovered.value ? { 'visible': true } : { 'invisible': true })
}));

function onMouseOver() {
	hovered.value = true;
}

function onMouseOut() {
	hovered.value = false;
}

</script>
