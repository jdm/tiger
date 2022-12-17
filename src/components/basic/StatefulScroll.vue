<template>
	<div ref="scrollableElement" class="overflow-y-auto" @scroll="onScroll">
		<slot />
	</div>
</template>

<script setup lang="ts">
import { ref, Ref, watch } from "vue";

const props = defineProps<{
	scrollTop: number,
}>();

const emit = defineEmits(["update:scrollTop"])

defineExpose({
	scrollToElement
});

const scrollableElement: Ref<HTMLElement | null> = ref(null);
let autoScrolling = false;

function onScroll() {
	if (!scrollableElement.value) {
		return;
	}
	if (autoScrolling) {
		autoScrolling = false;
		return;
	}
	emit("update:scrollTop", scrollableElement.value.scrollTop);
}

function scrollToElement(element: HTMLElement) {
	if (!scrollableElement.value) {
		return;
	}
	const scrollAreaRect = scrollableElement.value.getBoundingClientRect();
	const elementRect = element.getBoundingClientRect();
	const outOfViewUp = elementRect.top < scrollAreaRect.top;
	const outofViewDown = elementRect.bottom > scrollAreaRect.bottom;
	if (!outOfViewUp && !outofViewDown) {
		return;
	}
	autoScrolling = true;
	window.setTimeout(() => autoScrolling = false, 50);
	element.scrollIntoView(outOfViewUp);
}

watch(() => props.scrollTop, (newPosition) => {
	if (!scrollableElement.value) {
		return;
	}
	scrollableElement.value.scrollTop = newPosition;
});

</script>
