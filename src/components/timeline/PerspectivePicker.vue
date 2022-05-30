<template>
	<MultiSwitch :items="items" @activate="onActivate" />
</template>

<script setup lang="ts">
import { Ref, ref } from 'vue';
import MultiSwitch, { MultiSwitchItem } from '@/components/basic/MultiSwitch.vue'

const items: Ref<MultiSwitchItem[]> = ref([
	{ icon: "ArrowsExpandIcon", rotate: true } as MultiSwitchItem,
	{ icon: "ArrowsExpandIcon" } as MultiSwitchItem,
	{ icon: "SunIcon" } as MultiSwitchItem,
	{ icon: "SwitchHorizontalIcon" } as MultiSwitchItem,
	{ icon: "SwitchVerticalIcon" } as MultiSwitchItem,
	{ icon: "CubeIcon" } as MultiSwitchItem,
]);

function onActivate(activatedItem: MultiSwitchItem) {
	for (let item of items.value) {
		item.active = item == activatedItem;
	}
}

</script>