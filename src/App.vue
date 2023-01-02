<template>
  <div class="relative h-screen w-screen overflow-hidden select-none" @contextmenu="onContextMenu">
    <MainLayer />
    <ModalLayer class="absolute inset-0 z-[9998]" />
    <FloatingLayer class="absolute inset-0 z-[9999]" />
  </div>
</template>

<script setup lang="ts">
import { listen } from "@tauri-apps/api/event"
import { onMounted, onUnmounted, watch } from "vue"
import { AppState, Patch, TextureInvalidationEvent, } from "@/api/dto"
import { tick } from "@/api/document"
import { useAppStore } from "@/stores/app"
import { useDevStore } from "@/stores/dev"
import { useSpriteStore } from "@/stores/sprite"
import { registerKeyboardShortcuts, unregisterKeyboardShortcuts } from "@/utils/keyboard"
import FloatingLayer from "@/components/basic/FloatingLayer.vue"
import MainLayer from "@/components/MainLayer.vue"
import ModalLayer from "@/components/ModalLayer.vue"

const app = useAppStore();
const dev = useDevStore();
const sprite = useSpriteStore();

onMounted(() => {
  listen("patch-state", event => {
    app.patch(event.payload as Patch);
  });
  listen("replace-state", event => {
    app.$state = event.payload as AppState;
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
  if (app.currentDocument?.timelineIsPlaying) {
    window.requestAnimationFrame(runTick);
  }
}

watch(() => app.currentDocument?.timelineIsPlaying, (isPlaying) => {
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
