<template>
	<Teleport v-if="open" to="#menuLayer">
		<div ref="menuHTMLElement" class="absolute pointer-events-auto" :style="positionStyle">
			<Menu :content="content" @executed="onExecuted" />
		</div>
	</Teleport>
</template>

<script setup lang="ts">
import { computed, Ref, ref, watch } from "vue";
import Menu from "@/components/basic/Menu.vue";
import { MenuEntry, Separator } from "@/components/basic/MenuBar.vue";

const props = defineProps<{
	content: (MenuEntry | Separator)[],
	open: boolean,
	position: [number, number],
}>();

const emit = defineEmits<{
	(e: "executed"): void,
	(e: "dismissed"): void,
}>();

const menuHTMLElement: Ref<HTMLElement | null> = ref(null);

watch(() => props.open, (isOpen, wasOpen) => {
	if (isOpen && !wasOpen) {
		window.addEventListener("mousedown", onClickedAnywhere);
	}
	if (!isOpen && wasOpen) {
		window.removeEventListener("mousedown", onClickedAnywhere);
	}
});

function onClickedAnywhere(e: MouseEvent) {
	if (!menuHTMLElement.value?.contains(e.target as HTMLElement)) {
		emit("dismissed");
	}
}

function onExecuted() {
	emit("executed");
}

const positionStyle = computed(() => {
	return {
		left: `${props.position[0]}px`,
		top: `${props.position[1]}px`,
	};
});
</script>
