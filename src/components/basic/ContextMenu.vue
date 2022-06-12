<template>
	<Teleport v-if="open" to="#contextMenuContainer">
		<div ref="menuHTMLElement" class="absolute pointer-events-auto" :style="positionStyle">
			<Menu :content="content" @executed="onExecuted" />
		</div>
	</Teleport>
</template>

<script setup lang="ts">
import { computed, Ref, ref, watch } from "vue";
import Menu from "@/components/basic/Menu.vue";
import { MenuEntry, Separator } from "@/components/basic/MenuBar.vue";

defineProps<{
	content: (MenuEntry | Separator)[],
}>();

const open = ref(false);
const position: Ref<[number, number]> = ref([0, 0] as [number, number]);
const menuHTMLElement: Ref<HTMLElement | null> = ref(null);

defineExpose({
	open(event: MouseEvent) {
		open.value = true;
		position.value = [event.clientX, event.clientY];
	}
});

watch(open, (isOpen, wasOpen) => {
	if (isOpen && !wasOpen) {
		window.addEventListener("mousedown", onClickedAnywhere);
	}
	if (!isOpen && wasOpen) {
		window.removeEventListener("mousedown", onClickedAnywhere);
	}
});

function onClickedAnywhere(e: MouseEvent) {
	if (!menuHTMLElement.value?.contains(e.target as HTMLElement)) {
		open.value = false;
	}
}

function onExecuted() {
	open.value = false;
}

const positionStyle = computed(() => {
	return {
		left: `${position.value[0]}px`,
		top: `${position.value[1]}px`,
	};
});

</script>
