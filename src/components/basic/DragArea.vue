<template>
  <div ref="el" @mousedown="onMouseDown" :class="dragActive ? activeCursor : inactiveCursor" />
</template>

<script setup lang="ts">
import { computed, onUnmounted, Ref, ref } from 'vue';

export type DragAreaEvent = {
  mouseEvent: MouseEvent,
  htmlElement: HTMLElement,
}

type DragButton = "left" | "middle" | "right";

type Cursor = "cursor-move" | "cursor-pointer" | "cursor-ew-resize";

const el: Ref<HTMLElement | null> = ref(null)
const dragActive = ref(false);
defineExpose({ dragActive });

const props = defineProps<{
  button?: DragButton,
  inactiveCursor?: Cursor,
  activeCursor?: Cursor,
}>();

const emit =
  defineEmits<{
    (e: 'dragStart', event: DragAreaEvent): void
    (e: 'dragEnd', event: DragAreaEvent): void
    (e: 'dragUpdate', event: DragAreaEvent): void
  }>();

const buttonIndex = computed(() => {
  switch (props.button) {
    case "middle": return 1;
    case "right": return 2;
    default: return 0;
  }
});

onUnmounted(() => {
  if (dragActive.value) {
    cleanup();
  }
});

function cleanup() {
  window.removeEventListener("mouseup", onMouseUp);
  window.removeEventListener("mousemove", onMouseMove);
}

function onMouseDown(e: MouseEvent) {
  if (dragActive.value || !el.value || e.button != buttonIndex.value) {
    return;
  }
  window.addEventListener("mouseup", onMouseUp);
  window.addEventListener("mousemove", onMouseMove);
  dragActive.value = true;
  emit("dragStart", {
    mouseEvent: e,
    htmlElement: el.value
  });
}

function onMouseUp(e: MouseEvent) {
  if (!dragActive.value || !el.value || e.button != buttonIndex.value) {
    return;
  }
  cleanup();
  dragActive.value = false;
  emit("dragEnd", {
    mouseEvent: e,
    htmlElement: el.value
  });
}

function onMouseMove(e: MouseEvent) {
  if (!dragActive.value || !el.value) {
    return;
  }
  emit("dragUpdate", {
    mouseEvent: e,
    htmlElement: el.value
  });
}
</script>
