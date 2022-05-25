<template>
	<div class="px-8 pr-16 py-1.5 whitespace-nowrap hover:text-blue-500 hover:bg-plastic-800">
		{{ entry.name }}
	</div>
</template>

<script setup lang="ts">
import { MenuEntry } from '@/components/basic/MenuBar.vue';

defineProps<{
	entry: MenuEntry,
}>();
</script>