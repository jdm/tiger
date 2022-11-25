<template>
	<component :is="iconComponent" />
</template>

<script setup lang="ts">
import { computed } from "@vue/reactivity"
import * as mini from "@heroicons/vue/20/solid"
import * as outline from "@heroicons/vue/24/outline"
import * as solid from "@heroicons/vue/24/solid"

const props = defineProps<{
	name: keyof typeof solid,
	outline?: boolean,
	mini?: boolean,
}>();

const iconComponent = computed(() => {
	if (props.mini) {
		return mini[props.name]
	} else {
		return (props.outline ? outline : solid)[props.name]
	}
});
</script>