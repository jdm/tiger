<template>
	<div class="flex flex-row items-center text-zinc-400 hover:bg-zinc-900" :class="dynamicClasses">
		<div>{{ entry.name }}</div>
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { MenuBarEntry } from "@/components/basic/MenuBar.vue"

const props = defineProps<{
	entry: MenuBarEntry,
	active: boolean,
}>();


const dynamicClasses = computed(() => {
	return props.active ? ["bg-zinc-900"] : [];

});
</script>