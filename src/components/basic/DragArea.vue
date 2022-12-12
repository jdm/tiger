<template>
  <div ref="el" @mousedown="onMouseDown" :class="inactiveCursor">
    <div @contextmenu.capture="onContextMenu">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
import { debounceAnimation } from "@/utils/animation";
import { computed, onUnmounted, Ref, ref } from "vue"

export type DragButton = "left" | "middle" | "right";
export type Cursor = "cursor-move" | "cursor-pointer"
  | "cursor-nwse-resize" | "cursor-nesw-resize" | "cursor-ew-resize" | "cursor-ns-resize";

export type DragAreaEvent = {
  initialMouseEvent: MouseEvent,
  mouseEvent: MouseEvent,
  htmlElement: HTMLElement,
  button: DragButton,
  didMove: boolean,
}

const styleOverrideID = "drag-area-global-style";
const el: Ref<HTMLElement | null> = ref(null)
const activeDrag: Ref<DragButton | null> = ref(null);
let initialMouseEvent: MouseEvent | null = null;
const didMove = ref(false);

const props = defineProps<{
  buttons?: DragButton[],
  inactiveCursor?: Cursor,
  activeCursor?: Cursor,
}>();

const emit =
  defineEmits<{
    (e: "dragStart", event: DragAreaEvent): void
    (e: "dragEnd", event: DragAreaEvent): void
    (e: "dragUpdate", event: DragAreaEvent): void
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

const allowContextMenu = debounceAnimation([activeDrag, didMove], () => activeDrag.value == null || !didMove.value, 50);

onUnmounted(() => {
  if (activeDrag.value != null) {
    cleanup();
  }
});

function cleanup() {
  window.removeEventListener("mouseup", onMouseUp);
  window.removeEventListener("mousemove", onMouseMove);
  if (props.activeCursor) {
    document.getElementById(styleOverrideID)?.remove();
    document.body.classList.remove(props.activeCursor);
  }
}

function onMouseDown(e: MouseEvent) {
  if (activeDrag.value || !el.value || !buttonIndexes.value.includes(e.button)) {
    return;
  }
  window.addEventListener("mouseup", onMouseUp);
  window.addEventListener("mousemove", onMouseMove);
  activeDrag.value = indexToButton(e.button);

  if (props.activeCursor) {
    const cursorStyle = document.createElement('style');
    cursorStyle.id = styleOverrideID;
    cursorStyle.innerHTML = `body * { cursor: inherit !important; }`;
    document.head.appendChild(cursorStyle);
    document.body.classList.add(props.activeCursor);
  }

  initialMouseEvent = e;
  didMove.value = false;
  emit("dragStart", {
    mouseEvent: e,
    htmlElement: el.value,
    button: activeDrag.value,
    initialMouseEvent: initialMouseEvent,
    didMove: didMove.value,
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
    didMove: didMove.value,
  });
}

function onMouseMove(e: MouseEvent) {
  if (!activeDrag.value || !el.value || !initialMouseEvent) {
    return;
  }
  didMove.value = true;
  emit("dragUpdate", {
    mouseEvent: e,
    htmlElement: el.value,
    button: activeDrag.value,
    initialMouseEvent: initialMouseEvent,
    didMove: didMove.value,
  });
}


function onContextMenu(e: Event) {
  if (allowContextMenu.value || !buttonIndexes.value.includes(2)) {
    return;
  }
  e.preventDefault();
  e.stopPropagation();
}
</script>
