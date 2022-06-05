<template>
	<div class="relative rounded-md bg-plastic-800 border-2 border-plastic-900">
		<div v-if="activeItemElement" :style="knobStyle"
			class="absolute top-0 w-9 h-9 rounded-md transition-[left] border-y border-t-blue-600 border-b-blue-900 bg-gradient-to-b from-blue-800 to-blue-600" />
		<div ref="itemsElement" class="flex flex-row">
			<div v-for="item in items" class="z-10">
				<Icon :name="item.icon" @click="onItemClicked(item)" class="w-9 p-2 transition"
					:class="itemClasses(item)" />
			</div>
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed, Ref, ref } from "vue"
import * as solid from "@heroicons/vue/solid"
import Icon from "@/components/basic/Icon.vue"

export type MultiSwitchItem = {
	icon: keyof typeof solid,
	active?: boolean,
	rotate?: boolean,
};

const props = defineProps<{
	items: MultiSwitchItem[],
}>();

const emit = defineEmits(["activate"]);

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

function itemClasses(item: MultiSwitchItem) {
	return [
		...(item.active ? ["text-blue-200"] : ["text-plastic-500", "cursor-pointer"]),
		...(item.rotate ? ["rotate-45"] : [])
	];
}

function onItemClicked(item: MultiSwitchItem) {
	emit("activate", item);
}
</script>
