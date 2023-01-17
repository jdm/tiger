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
import { showErrorMessage, tick } from "@/backend/api"
import { State, Patch, TextureInvalidation, OpenDocumentError, SaveDocumentError, } from "@/backend/dto"
import { useDevStore } from "@/stores/dev"
import { useSpriteStore } from "@/stores/sprite"
import { useStateStore } from "@/stores/state"
import { registerKeyboardShortcuts, unregisterKeyboardShortcuts } from "@/utils/keyboard"
import FloatingLayer from "@/components/basic/FloatingLayer.vue"
import MainLayer from "@/components/MainLayer.vue"
import ModalLayer from "@/components/ModalLayer.vue"

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
    const invalidationEvent = event.payload as TextureInvalidation;
    sprite.invalidate(invalidationEvent.path);
  });
  listen("open-document-error", event => {
    const openDocumentError = event.payload as OpenDocumentError;
    const description = `Something went wrong while opening <span class="italic font-medium text-orange-500">${openDocumentError.documentName}</span>:`;
    showErrorMessage("Error", description, openDocumentError.error);
  });
  listen("save-document-error", event => {
    const saveDocumentError = event.payload as SaveDocumentError;
    const description = `Something went wrong while saving <span class="italic font-medium text-orange-500">${saveDocumentError.documentName}</span>:`;
    showErrorMessage("Error", description, saveDocumentError.error);
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
