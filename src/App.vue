<template>
  <div class="flex flex-col h-screen w-screen overflow-clip bg-plastic-900 select-none" @contextmenu="onContextMenu">

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
</template>

<script setup lang="ts">
import AppBar from '@/components/AppBar.vue'
import ContentPane from '@/components/ContentPane.vue'
import TimelinePane from '@/components/timeline/TimelinePane.vue'
import WorkbenchPane from '@/components/workbench/WorkbenchPane.vue';
import { useAppStore } from '@/stores/app'
import { ref, watch } from 'vue';
import { tick } from '@/api/document';

const app = useAppStore();
const allowContextMenu = ref(false);

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

function onToggleDevTools() {
  allowContextMenu.value = !allowContextMenu.value;
}

function onContextMenu(e: Event) {
  if (!allowContextMenu.value) {
    e.preventDefault();
  }
}

</script>
