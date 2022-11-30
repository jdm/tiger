<template>
	<Teleport v-if="open" to="#floatingLayer">
		<div ref="el" class="absolute pointer-events-auto" :style="positionStyle">
			<slot />
		</div>
	</Teleport>
</template>

<script setup lang="ts">
import { computed, Ref, ref, watch } from "vue";

const props = defineProps<{
	open: boolean,
	position: [number, number],
}>();

const emit = defineEmits<{
	(e: "dismissed"): void,
}>();

const el: Ref<HTMLElement | null> = ref(null);

const positionStyle = computed(() => {
	return {
		left: `${props.position[0]}px`,
		top: `${props.position[1]}px`,
	};
});

watch(() => props.open, (isOpen, wasOpen) => {
	if (isOpen && !wasOpen) {
		window.addEventListener("mousedown", onClickedAnywhere);
	}
	if (!isOpen && wasOpen) {
		window.removeEventListener("mousedown", onClickedAnywhere);
	}
});

function onClickedAnywhere(e: MouseEvent) {
	e.stopPropagation();
	if (!el.value?.contains(e.target as HTMLElement)) {
		emit("dismissed");
	}
}
</script>
