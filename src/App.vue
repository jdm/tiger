<template>
  <div class="relative h-screen w-screen overflow-hidden select-none" @contextmenu="onContextMenu">
    <div class="h-full w-full flex flex-col overflow-clip bg-plastic-900">
      <AppBar v-model:debugMode="allowContextMenu" />
      <div class="flex-1 relative">
        <div class="absolute inset-0 min-h-0 flex flex-row space-x-5 p-5">
          <div class="basis-[28rem] min-w-0 flex flex-col space-y-5">
            <AnimationsPane class="flex-1" />
            <FramesPane class="flex-1" />
          </div>
          <div class="flex-1 min-w-0 flex flex-col">
            <div class="flex-1 min-h-0 flex flex-row space-x-5 pb-5">
              <WorkbenchPane class="flex-1" />
              <div class="basis-80 min-w-0 flex flex-col space-y-5">
                <KeyframePane class="flex-1" />
                <DetailsPane class="basis-80" />
              </div>
            </div>
            <TimelinePane />
          </div>
        </div>
        <ExportOverlay class="absolute inset-0 z-[9997]" />
      </div>
    </div>
    <ModalLayer class="absolute inset-0 z-[9998]" />
    <ContextMenuLayer class="absolute inset-0 z-[9999]" />
  </div>
</template>

<script setup lang="ts">
import { listen } from "@tauri-apps/api/event"
import { onMounted, onUnmounted, ref, watch } from "vue"
import { getState } from "@/api/app"
import { AppState, Patch, TextureInvalidationEvent, } from "@/api/dto"
import { tick } from "@/api/document"
import { useAppStore } from "@/stores/app"
import { useSpriteStore } from "@/stores/sprite"
import { registerKeyboardShortcuts, unregisterKeyboardShortcuts } from "@/utils/keyboard"
import AppBar from "@/components/AppBar.vue"
import AnimationsPane from "@/components/animations/AnimationsPane.vue"
import ContextMenuLayer from "@/components/basic/MenuLayer.vue"
import DetailsPane from "@/components/details/DetailsPane.vue"
import ExportOverlay from "@/components/export/ExportOverlay.vue"
import FramesPane from "@/components/frames/FramesPane.vue"
import KeyframePane from "@/components/keyframe/KeyframePane.vue"
import TimelinePane from "@/components/timeline/TimelinePane.vue"
import WorkbenchPane from "@/components/workbench/WorkbenchPane.vue"
import ModalLayer from "@/components/ModalLayer.vue"

const app = useAppStore();
const sprite = useSpriteStore();
const allowContextMenu = ref(false);

onMounted(() => {
  listen("force-patch-state", event => {
    app.patch(event.payload as Patch);
  });
  listen("force-replace-state", event => {
    app.$state = event.payload as AppState;
  });
  listen("invalidate-texture", event => {
    const invalidationEvent = event.payload as TextureInvalidationEvent;
    sprite.invalidate(invalidationEvent.path);
  });
  listen("invalidate-template", event => {
    getState();
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
  if (!allowContextMenu.value) {
    e.preventDefault();
  }
}

</script>
