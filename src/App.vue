<template>
  <div class="relative h-screen w-screen overflow-hidden select-none" @contextmenu="onContextMenu">
    <MainLayer />
    <NotificationLayer class="absolute inset-0 z-[9997]" />
    <ModalLayer class="absolute inset-0 z-[9998]" />
    <FloatingLayer class="absolute inset-0 z-[9999]" />
  </div>
</template>

<script setup lang="ts">
import { listen } from "@tauri-apps/api/event"
import { onMounted, onUnmounted, watch } from "vue"
import { tick } from "@/backend/api"
import { State, Patch, TextureInvalidationEvent, } from "@/backend/dto"
import { useDevStore } from "@/stores/dev"
import { useSpriteStore } from "@/stores/sprite"
import { useStateStore } from "@/stores/state"
import { registerKeyboardShortcuts, unregisterKeyboardShortcuts } from "@/utils/keyboard"
import FloatingLayer from "@/components/basic/FloatingLayer.vue"
import MainLayer from "@/components/MainLayer.vue"
import ModalLayer from "@/components/ModalLayer.vue"
import NotificationLayer from "@/components/NotificationLayer.vue"

const dev = useDevStore();
const sprite = useSpriteStore();
const state = useStateStore();

onMounted(() => {
  listen("patch-state", event => {
    state.patch(event.payload as Patch);
  });
  listen("replace-state", event => {
    state.$state = event.payload as State;
  });
  listen("invalidate-texture", event => {
    const invalidationEvent = event.payload as TextureInvalidationEvent;
    sprite.invalidate(invalidationEvent.path);
  });
  registerKeyboardShortcuts();
});

onUnmounted(() => {
  unregisterKeyboardShortcuts();
});

let previousTimestamp: number | null = null;
async function runTick(timestamp: number) {
  if (previousTimestamp != null) {
    await tick(timestamp - previousTimestamp);
  }
  previousTimestamp = timestamp;
  if (state.currentDocument?.timelineIsPlaying) {
    window.requestAnimationFrame(runTick);
  }
}

watch(() => state.currentDocument?.timelineIsPlaying, (isPlaying) => {
  if (isPlaying) {
    previousTimestamp = null;
    window.requestAnimationFrame(runTick);
  }
});

function onContextMenu(e: Event) {
  if (!dev.debugModeEnabled) {
    e.preventDefault();
  }
}

</script>
