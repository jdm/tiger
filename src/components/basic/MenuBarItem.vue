<template>
	<div class="flex flex-row items-center text-plastic-300 hover:text-plastic-200 hover:bg-plastic-500">
		<div>{{ entry.name }}</div>
	</div>
</template>

<script setup lang="ts">
import { MenuBarEntry } from '@/components/basic/MenuBar.vue';

defineProps<{
	entry: MenuBarEntry
}>();
</script>