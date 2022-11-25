<template>
	<div class="relative text-lg">
		<div class="h-full flex flex-row items-stretch">
			<MenuBarItem v-for="entry in entries" :entry="entry" :active="entry.name == currentEntry?.name"
				@click="onItemClicked($event, entry)" @mouseover="onItemHovered($event, entry)" />
		</div>
		<MenuTree :open="!!currentEntry" :content="currentEntry?.content || []" @executed="onExecuted"
			@dismissed="onDismissed" :position="menuPosition" />
	</div>
</template>

<script setup lang="ts">
import { computed, ref, Ref, watch } from "vue"
import MenuBarItem from "@/components/basic/MenuBarItem.vue"
import MenuTree from "@/components/basic/MenuTree.vue"

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
