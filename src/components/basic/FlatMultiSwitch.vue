<template>
	<div class="relative">
		<div v-if="activeItemElement" :style="knobStyle"
			class="absolute top-0 w-11 h-9 rounded-md transition-[left] border-2 border-blue-600" />
		<div ref="itemsElement" class="flex flex-row items-center font-xs font-semibold">
			<div v-for="item in items" @click="onItemClicked(item)"
				class="w-11 h-9 inline-flex items-center justify-center z-10" :class="itemClass(item)">
				{{ item.text }}
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed, Ref, ref } from "vue"

export type FlatMultiSwitchItem = {
	text: string,
	active?: boolean,
};

const props = defineProps<{
	items: FlatMultiSwitchItem[],
}>();

const emit = defineEmits<{
	(e: "activate", item: FlatMultiSwitchItem): void
}>();

const itemsElement: Ref<HTMLElement | null> = ref(null);

const activeItemElement = computed(() => {
	if (!itemsElement.value) {
		return null;
	}
	const activeIndex = props.items.findIndex((item) => item.active);
	return itemsElement.value.children.item(activeIndex) as HTMLElement;
});

const knobStyle = computed(() => {
	if (!activeItemElement) {
		return {};
	}
	const left = activeItemElement.value?.offsetLeft;
	return {
		left: `${left}px`,
	};
});

function itemClass(item: FlatMultiSwitchItem) {
	return [
		...(item.active ? ["text-plastic-200"] : ["text-plastic-400", "cursor-pointer"]),
	];
}

function onItemClicked(item: FlatMultiSwitchItem) {
	if (item.active) {
		return;
	}
	emit("activate", item);
}
</script>
