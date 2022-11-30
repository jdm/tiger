<template>
	<div class="relative text-lg">
		<div class="h-full flex items-stretch">
			<MenuBarItem v-for="entry in entries" :entry="entry" :active="entry.name == currentEntry?.name"
				@click="onItemClicked($event, entry)" @mouseover="onItemHovered($event, entry)" />
		</div>
		<FloatingWidget :open="!!currentEntry" :position="menuPosition" @dismissed="onDismissed">
			<Menu :content="currentEntry?.content || []" @executed="onExecuted" />
		</FloatingWidget>
	</div>
</template>

<script setup lang="ts">
import { computed, ref, Ref } from "vue"
import FloatingWidget from "@/components/basic/FloatingWidget.vue"
import Menu from "@/components/basic/Menu.vue"
import MenuBarItem from "@/components/basic/MenuBarItem.vue"

export type MenuBarEntry = {
	name: string,
	content: (MenuEntry | Separator)[],
};
export type Separator = {};

export type MenuEntry = {
  key: string,
  name: string,
  shortcut?: string,
  action?: () => Promise<void>,
  submenus?: (MenuEntry | Separator)[],
  disabled?: boolean,
};

defineProps<{
	entries: MenuBarEntry[],
}>();

const currentItem: Ref<HTMLElement | null> = ref(null);
const currentEntry: Ref<MenuBarEntry | null> = ref(null);

const menuPosition = computed((): [number, number] => {
	if (!currentItem.value) {
		return [0, 0];
	}
	const rect = currentItem.value.getBoundingClientRect();
	return [rect.left, rect.bottom - 2];
});

const menuPositionStyle = computed(() => {
	return {
		left: `${menuPosition.value[0]}px`,
		top: `${menuPosition.value[1]}px`,
	};
});

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

function onExecuted() {
	currentEntry.value = null;
}

function onDismissed() {
	currentEntry.value = null;
}
</script>

