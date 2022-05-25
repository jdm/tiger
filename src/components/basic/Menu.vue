<template>
	<div class="py-2 text-plastic-300 bg-plastic-700  shadow-md  shadow-black/30">
		<component v-for="entry of content" :is="('name' in entry) ? MenuItem : MenuSeparator" :entry="entry" />
	</div>
</template>

<script setup lang="ts">
import { MenuEntry, Separator } from '@/components/basic/MenuBar.vue';
import MenuItem from '@/components/basic/MenuItem.vue';
import MenuSeparator from '@/components/basic/MenuSeparator.vue';

defineProps<{
	content: (MenuEntry | Separator)[],
}>();
</script>