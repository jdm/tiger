<template>

  <div class="flex flex-nowrap flex-col h-screen w-screen space-y-5 p-5 bg-plastic-900 overflow-clip select-none">

    <div class="space-x-4 text-white flex flex-row">
      <button @click="newDocument" class="place-self-start">New Document</button>
      <button @click="openDocuments" class="place-self-start">Open Document</button>
    </div>

    <div class="flex-1 min-h-0 flex flex-row space-x-5">
      <ContentPane class="basis-96 min-w-0" />
      <WorkbenchPane class="flex-1" />
    </div>

    <TimelinePane class="basis-60" />

  </div>
</template>

<script setup lang="ts">
import { newDocument, openDocuments } from '@/api/local'
import WorkbenchPane from '@/components/WorkbenchPane.vue';
import ContentPane from '@/components/ContentPane.vue'
import TimelinePane from '@/components/timeline/TimelinePane.vue'
import { useAppStore } from '@/stores/app'
import { watch } from 'vue';
import { tick } from '@/api/document';

const app = useAppStore();
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

</script>
