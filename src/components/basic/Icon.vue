<template>
	<component :is="iconComponent" />
</template>

<script setup lang="ts">
import { computed } from '@vue/reactivity'
import * as solid from '@heroicons/vue/solid'
import * as outline from '@heroicons/vue/outline'

const props = defineProps<{
	name: keyof typeof solid,
	outline?: boolean,
}>();

const iconComponent = computed(() =>
	(props.outline ? outline : solid)[props.name]
);
</script>