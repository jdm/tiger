<template>
	<div @mouseover="onMouseOver" @mouseout="onMouseOut"
		class="px-4 py-1 flex flex-row items-center space-x-4 cursor-pointer"
		:class="selected ? 'text-blue-100 bg-blue-600' : 'text-plastic-200 hover:bg-plastic-600'">
		<div class="flex-1 flex flex-row items-center space-x-1 min-w-0">
			<Icon v-if="leftIcon" :name="leftIcon" class="inline w-7 p-1.5"
				:class="selected ? 'text-blue-200' : 'text-plastic-300'" />
			<div class="flex-1 min-w-0">
				<slot name="content">
					<div class=" whitespace-nowrap overflow-x-hidden text-ellipsis">{{ text }}</div>
				</slot>
			</div>
		</div>
		<div v-if="actions && actions.length > 0" class="flex flex-row">
			<Icon v-for="action in actions" @click.stop="action.callback" :name="action.icon"
				class="inline w-7 p-1.5 rounded-lg hover:visible" :class="interactiveIconClasses" />
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed, ref } from '@vue/reactivity';
import Icon from '@/components/basic/Icon.vue';
import * as solid from '@heroicons/vue/solid';

const props = defineProps<{
	selected: boolean
	text: string
	leftIcon?: keyof typeof solid
	actions?: { icon: keyof typeof solid, callback: () => void }[],
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
