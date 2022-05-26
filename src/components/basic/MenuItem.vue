<template>
	<div @mouseover="onMouseOver" @mouseout="onMouseOut"
		class="flex flex-row justify-between px-8 space-x-20 py-1.5 whitespace-nowrap hover:text-blue-100 hover:bg-blue-600">
		<div>{{ entry.name }}</div>
		<div :class="hovered ? 'text-blue-400' : 'text-plastic-400'">{{ entry.shortcut }}</div>
	</div>
</template>

<script setup lang="ts">
import { MenuEntry } from '@/components/basic/MenuBar.vue';
import { ref } from 'vue';

defineProps<{
	entry: MenuEntry,
}>();

const hovered = ref(false);

function onMouseOver() {
	hovered.value = true;
}

function onMouseOut() {
	hovered.value = false;
}

</script>