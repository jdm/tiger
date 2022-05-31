<template>
	<div @mouseover="onMouseOver" @mouseout="onMouseOut" @click="onClick"
		class="flex flex-row justify-between px-8 space-x-20 py-1.5 whitespace-nowrap hover:bg-blue-600">
		<div :class="hovered ? 'text-blue-100' : 'text-zinc-400'">{{ entry.name }}</div>
		<div :class="hovered ? 'text-blue-400' : 'text-zinc-600'">{{ entry.shortcut }}</div>
	</div>
</template>

<script setup lang="ts">
import { MenuEntry } from '@/components/basic/MenuBar.vue';
import { ref } from 'vue';

const props = defineProps<{
	entry: MenuEntry,
}>();

const emit = defineEmits(["executed"]);

const hovered = ref(false);

function onMouseOver() {
	hovered.value = true;
}

function onMouseOut() {
	hovered.value = false;
}

function onClick() {
	if (props.entry.action) {
		emit("executed");
		props.entry.action();
	}
}

</script>