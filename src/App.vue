<template>
  <div class="relative h-screen w-screen overflow-hidden select-none" @contextmenu="onContextMenu">
    <div class="h-full w-full flex flex-col overflow-clip bg-plastic-900">
      <AppBar v-model:debugMode="allowContextMenu" />
      <div class="flex-1 min-h-0 flex flex-col space-y-5">
        <div class="flex-1 min-h-0 flex flex-col p-5">
          <div class="flex-1 min-h-0 flex flex-row space-x-5 pb-5">
            <ContentPane class="basis-96 min-w-0" />
            <WorkbenchPane class="flex-1" />
          </div>
          <TimelinePane class="basis-60" />
        </div>
      </div>
    </div>
    <ModalLayer class="absolute inset-0" />
  </div>
</template>

<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window';
import { onMounted, ref, watch } from 'vue';
import { tick } from '@/api/document';
import AppBar from '@/components/AppBar.vue'
import ContentPane from '@/components/content/ContentPane.vue'
import TimelinePane from '@/components/timeline/TimelinePane.vue'
import WorkbenchPane from '@/components/workbench/WorkbenchPane.vue';
import { useAppStore } from '@/stores/app'
import ModalLayer from '@/components/ModalLayer.vue';
import { AppState } from '@/api/dto';

const app = useAppStore();
const allowContextMenu = ref(false);

onMounted(() => {
  appWindow.listen("force-refresh-state", event => {
    app.$state = event.payload as AppState;
  });
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
