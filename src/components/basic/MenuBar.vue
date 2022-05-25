<template>
	<div class="relative text-lg ">
		<div class="h-full flex flex-row items-stretch">
			<MenuBarItem v-for="entry in entries" :entry="entry" @mouseover="onItemHovered(entry)" class="px-4" />
		</div>
		<Menu v-if="currentEntry" class="absolute z-50" :content="currentEntry.content" />
	</div>
</template>

<script setup lang="ts">
import MenuBarItem from '@/components/basic/MenuBarItem.vue'
import Menu from '@/components/basic/Menu.vue'
import { ref, Ref } from '@vue/reactivity';

export type MenuBarEntry = {
	name: string,
	content: (MenuEntry | Separator)[]
};

export type Separator = {};

export type MenuEntry = {
	name: string,
	shortcut?: string,
	action?: () => Promise<void>,
};

defineProps<{
	entries: MenuBarEntry[],
}>();

const currentEntry: Ref<MenuBarEntry | null> = ref(null);

function onItemHovered(entry: MenuBarEntry) {
	currentEntry.value = entry;
}
</script>