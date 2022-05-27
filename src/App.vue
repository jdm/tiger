<template>
  <div class="relative h-screen w-screen select-none" @contextmenu="onContextMenu">
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

    <div v-if="app.currentDocument?.wasCloseRequested">
      <div class="absolute inset-0 bg-black/70" />
      <div class="absolute inset-0">
        <div class="h-full flex items-center justify-center">
          <div class="max-w-lg rounded-md bg-plastic-900 border-4 border-plastic-700 shadow-lg shadow-blue-900/25">
            <div class="flex p-6 pb-5 space-x-6 bg-plastic-900">
              <div class="h-9 w-9 flex-shrink-0 flex items-center justify-center rounded-full bg-amber-300">
                <ExclamationIcon class="h-6 w-6 text-amber-900" />
              </div>
              <div>
                <h3 class="text-lg leading-6 font-medium text-plastic-100">Unsaved Changes</h3>
                <div class="mt-3 flex flex-col space-y-1 text-sm text-plastic-300">
                  <p><span class="italic font-semibold text-orange-500">Sahagin.tiger</span> has been
                    modified. Would you like to save changes before closing the spritesheet?</p>
                </div>
              </div>
            </div>
            <div class="bg-plastic-800 px-6 py-3 flex flex-row justify-end">
              <button type="button"
                class="basis-20 inline-flex justify-center rounded-md px-4 py-2 bg-green-600 text-base font-medium text-white hover:bg-green-700 sm:ml-3 sm:text-sm">Yes</button>
              <button type="button"
                class="basis-20 inline-flex justify-center rounded-md px-4 py-2 bg-red-600 text-base font-medium text-white hover:bg-red-700 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm">No</button>
              <button type="button"
                class="basis-20 inline-flex justify-center rounded-md px-4 py-2 bg-plastic-400 text-base font-medium text-plastic-100 hover:bg-plastic-500 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm">Cancel</button>
            </div>

          </div>
        </div>
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
import { ExclamationIcon } from '@heroicons/vue/outline';

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
