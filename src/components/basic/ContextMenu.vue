<template>
	<FloatingWidget :open="open" :position="position" @dismissed="onDismissed">
		<Menu :content="content" @executed="onExecuted" />
	</FloatingWidget>
</template>

<script setup lang="ts">
import { Ref, ref } from "vue";
import FloatingWidget from "@/components/basic/FloatingWidget.vue";
import Menu from "@/components/basic/Menu.vue";
import { MenuEntry, Separator } from "@/components/basic/MenuBar.vue";

defineProps<{
	content: (MenuEntry | Separator)[],
}>();

const open = ref(false);
const position: Ref<[number, number]> = ref([0, 0] as [number, number]);

defineExpose({
	show(event: MouseEvent) {
		open.value = true;
		position.value = [event.clientX, event.clientY];
	}
});

function onExecuted() {
	open.value = false;
}

function onDismissed() {
	open.value = false;
}
</script>
