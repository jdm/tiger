<template>
	<div ref="scrollableElement" class="overflow-y-auto" @scroll="onScroll">
		<slot />
	</div>
</template>

<script setup lang="ts">
import { nextTick, ref, Ref, watch } from "vue";

const props = defineProps<{
	scrollTop: number,
}>();

const emit = defineEmits(["update:scrollTop"]);

defineExpose({
	scrollToElement
});

const scrollableElement: Ref<HTMLElement | null> = ref(null);
let autoScrolling = false;
let incoming = new Map<number, number>();
let outgoing = new Map<number, number>();

function onScroll() {
	if (!scrollableElement.value) {
		return;
	}
	if (autoScrolling) {
		autoScrolling = false;
		return;
	}
	const newPosition = scrollableElement.value.scrollTop;
	if (incoming.get(newPosition) != undefined) {
		decrement(incoming, newPosition);
	} else {
		increment(outgoing, newPosition);
		emit("update:scrollTop", scrollableElement.value.scrollTop);
	}
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
	nextTick(() => {
		if (!scrollableElement.value) {
			return;
		}
		if (outgoing.get(newPosition) != undefined) {
			decrement(outgoing, newPosition);
		} else {
			increment(incoming, newPosition);
			scrollableElement.value.scrollTop = newPosition;
		}
	});
});

function increment(ledger: Map<number, number>, value: number) {
	ledger.set(value, 1 + (ledger.get(value) || 0));
}

function decrement(ledger: Map<number, number>, value: number) {
	const current = ledger.get(value) || 0;
	if (current > 1) {
		ledger.set(value, current - 1);
	} else if (current == 1) {
		ledger.delete(value);
	}
}
</script>
