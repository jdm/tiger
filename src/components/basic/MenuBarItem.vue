<template>
	<div class="flex" @mouseenter="onMouseEnter" @mouseleave="onMouseLeave">
		<div class="rounded-md py-0.5 px-4 self-center text-zinc-200" :class="dynamicClasses">
			{{ entry.name }}
		</div>
	</div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue"
import { MenuBarEntry } from "@/components/basic/MenuBar.vue"

const props = defineProps<{
	entry: MenuBarEntry,
	active: boolean,
}>();

const hovered = ref(false);

function onMouseEnter() {
	hovered.value = true;
}

function onMouseLeave() {
	hovered.value = false;
}

const dynamicClasses = computed(() => {
	return props.active || hovered.value ? ["bg-plastic-500"] : [];
});
</script>