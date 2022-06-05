<template>
  <div ref="el" @mousedown="onMouseDown" :class="activeDrag != null ? activeCursor : inactiveCursor" />
</template>

<script setup lang="ts">
import { computed, onUnmounted, Ref, ref } from 'vue';

export type DragButton = "left" | "middle" | "right";
export type Cursor = "cursor-move" | "cursor-pointer" | "cursor-ew-resize";

export type DragAreaEvent = {
  initialMouseEvent: MouseEvent,
  mouseEvent: MouseEvent,
  htmlElement: HTMLElement,
  button: DragButton,
  didMove: boolean,
}

const el: Ref<HTMLElement | null> = ref(null)
const activeDrag: Ref<DragButton | null> = ref(null);
let initialMouseEvent: MouseEvent | null = null;
let didMove: boolean = false;

const props = defineProps<{
  buttons?: DragButton[],
  inactiveCursor?: Cursor,
  activeCursor?: Cursor,
}>();

const emit =
  defineEmits<{
    (e: 'dragStart', event: DragAreaEvent): void
    (e: 'dragEnd', event: DragAreaEvent): void
    (e: 'dragUpdate', event: DragAreaEvent): void
  }>();

const buttonIndexes = computed((): number[] => {
  return (props.buttons || ["left"]).map(button => {
    switch (button) {
      case "middle": return 1;
      case "right": return 2;
      default: return 0;
    }
  });
});

function indexToButton(index: number): DragButton {
  switch (index) {
    case 1: return "middle";
    case 2: return "right";
    default: return "left";
  }
}

onUnmounted(() => {
  if (activeDrag != null) {
    cleanup();
  }
});

function cleanup() {
  window.removeEventListener("mouseup", onMouseUp);
  window.removeEventListener("mousemove", onMouseMove);
}

function onMouseDown(e: MouseEvent) {
  if (activeDrag.value || !el.value || !buttonIndexes.value.includes(e.button)) {
    return;
  }
  window.addEventListener("mouseup", onMouseUp);
  window.addEventListener("mousemove", onMouseMove);
  activeDrag.value = indexToButton(e.button);
  initialMouseEvent = e;
  didMove = false;
  emit("dragStart", {
    mouseEvent: e,
    htmlElement: el.value,
    button: activeDrag.value,
    initialMouseEvent: initialMouseEvent,
    didMove: didMove,
  });
}

function onMouseUp(e: MouseEvent) {
  const button = indexToButton(e.button);
  if (activeDrag.value != button || !el.value || !initialMouseEvent) {
    return;
  }
  cleanup();
  activeDrag.value = null;
  emit("dragEnd", {
    mouseEvent: e,
    htmlElement: el.value,
    button: button,
    initialMouseEvent: initialMouseEvent,
    didMove: didMove,
  });
}

function onMouseMove(e: MouseEvent) {
  if (!activeDrag.value || !el.value || !initialMouseEvent) {
    return;
  }
  didMove = true;
  emit("dragUpdate", {
    mouseEvent: e,
    htmlElement: el.value,
    button: activeDrag.value,
    initialMouseEvent: initialMouseEvent,
    didMove: didMove,
  });
}
</script>
