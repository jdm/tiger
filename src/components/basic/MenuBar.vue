<template>
	<div class="relative text-lg ">
		<div class="h-full flex flex-row items-stretch">
			<MenuBarItem v-for="entry in entries" :entry="entry" :active="entry.name == currentEntry?.name"
				@click="onItemClicked($event, entry)" @mouseover="onItemHovered($event, entry)" class="px-4" />
		</div>
		<Menu v-if="currentEntry" class="absolute z-50" :style="menuPosition" :content="currentEntry.content" />
	</div>
</template>

<script setup lang="ts">
import MenuBarItem from '@/components/basic/MenuBarItem.vue'
import Menu from '@/components/basic/Menu.vue'
import { ref, Ref } from '@vue/reactivity';
import { computed, watch } from 'vue';

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

const currentItem: Ref<HTMLElement | null> = ref(null);
const currentEntry: Ref<MenuBarEntry | null> = ref(null);

function onItemClicked(event: MouseEvent, entry: MenuBarEntry) {
	currentItem.value = event.currentTarget as HTMLElement;
	currentEntry.value = entry;
}

function onItemHovered(event: MouseEvent, entry: MenuBarEntry) {
	if (currentEntry.value) {
		currentItem.value = event.currentTarget as HTMLElement;
		currentEntry.value = entry;
	}
}

function onClickedAnywhere() {
	currentEntry.value = null;
}

watch(currentEntry, (newEntry, oldEntry) => {
	if (newEntry && !oldEntry) {
		window.addEventListener("mousedown", onClickedAnywhere);
	}
	if (!newEntry && oldEntry) {
		window.removeEventListener("mousedown", onClickedAnywhere);
	}
});

const menuPosition = computed(() => {
	return {
		left: (currentItem.value?.offsetLeft || 0) + "px"
	};
});
</script>