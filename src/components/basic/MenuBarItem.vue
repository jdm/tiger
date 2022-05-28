<template>
	<div class="flex flex-row items-center hover:text-plastic-200 hover:bg-plastic-500" :class="dynamicClasses">
		<div>{{ entry.name }}</div>
	</div>
</template>

<script setup lang="ts">
import { MenuBarEntry } from '@/components/basic/MenuBar.vue';
import { computed } from 'vue';

const props = defineProps<{
	entry: MenuBarEntry,
	active: boolean,
}>();


const dynamicClasses = computed(() => {
	return props.active ? ['text-plastic-200', 'bg-plastic-500'] : ['text-plastic-300'];

});
</script>