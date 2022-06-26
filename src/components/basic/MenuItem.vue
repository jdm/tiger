<template>
	<div @mouseenter="onMouseEnter" @mouseleave="onMouseLeave" @click="onClick"
		class="flex flex-row justify-between px-8 space-x-20 py-1.5 whitespace-nowrap hover:bg-blue-600"
		:class="entry.disabled ? 'pointer-events-none' : ''">
		<div :class="entry.disabled ? 'text-zinc-600' : hovered ? 'text-blue-100' : 'text-zinc-400'">
			{{ entry.name }}
		</div>
		<div :class="hovered ? 'text-blue-400' : 'text-zinc-600'">{{ entry.shortcut }}</div>
	</div>
</template>

<script setup lang="ts">
import { ref } from "vue"
import { MenuEntry } from "@/components/basic/MenuBar.vue"

const props = defineProps<{
	entry: MenuEntry,
}>();

const emit = defineEmits(["executed"]);

const hovered = ref(false);

function onMouseEnter() {
	hovered.value = true;
}

function onMouseLeave() {
	hovered.value = false;
}

function onClick() {
	if (props.entry.action) {
		emit("executed");
		props.entry.action();
	}
}

</script>