<template>
	<div ref="el">
		<slot />
	</div>
</template>

<script setup lang="ts">
import * as focusTrap from "focus-trap";
import { onUnmounted, Ref, ref, watch } from "vue";
import { useFocusStore } from "@/stores/focus";

const emit = defineEmits<{
	(e: "escape"): void,
}>();

const focus = useFocusStore();

const el: Ref<HTMLElement | null> = ref(null);
let trap: focusTrap.FocusTrap | undefined = undefined;

watch(el, (current, previous) => {
	if (trap) {
		trap.deactivate();
	}
	if (current) {
		trap = focusTrap.createFocusTrap(current, {
			allowOutsideClick: true,
			escapeDeactivates: () => {
				emit("escape");
				return false;
			},
			isKeyBackward: (event: KeyboardEvent) => {
				return	(event.key == "Tab" && event.shiftKey && !event.ctrlKey)
					||	(canUseArrowKeys() && event.key == "ArrowLeft" && !event.shiftKey && !event.ctrlKey)
					;
			},
			isKeyForward: (event: KeyboardEvent) => {
				return	(event.key == "Tab" && !event.shiftKey && !event.ctrlKey)
					||	(canUseArrowKeys() && event.key == "ArrowRight" && !event.shiftKey && !event.ctrlKey)
					;
			},
			onActivate: () => {
				focus.trapInput();
			},
			onDeactivate: () => {
				focus.freeInput();
			},
		}).activate();
	}
});

onUnmounted(() => {
	if (trap) {
		trap.deactivate();
	}
});

function canUseArrowKeys() {
	return document.activeElement?.tagName != "INPUT";
}

</script>