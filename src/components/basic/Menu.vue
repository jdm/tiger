<template>
	<div class="py-2 text-plastic-300 bg-plastic-700 shadow-md shadow-black/30">
		<div v-for="entry of content">
			<MenuItem v-if="'name' in entry" :entry="entry" @executed="onExecuted" />
			<MenuSeparator v-else />
		</div>
	</div>
</template>

<script setup lang="ts">
import { MenuEntry, Separator } from '@/components/basic/MenuBar.vue';
import MenuItem from '@/components/basic/MenuItem.vue';
import MenuSeparator from '@/components/basic/MenuSeparator.vue';

defineProps<{
	content: (MenuEntry | Separator)[],
}>();

const emit = defineEmits(["executed"]);

function onExecuted() {
	emit("executed");
}
</script>