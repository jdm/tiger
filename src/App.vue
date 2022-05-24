<template>
  <div class="relative h-screen w-screen overflow-clip bg-plastic-900 select-none" @contextmenu="onContextMenu">

    <div class="w-full h-full flex flex-col space-y-5 p-5">
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

    <div v-if="!app.isReleaseBuild" class="absolute top-0 right-0 px-5 py-3">
      <button @click="onToggleDevTools" class="rounded-md p-2 px-4 text-white"
        :class="allowContextMenu ? 'bg-green-500' : 'bg-red-500'">🐛</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { newDocument, openDocuments } from '@/api/local'
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
