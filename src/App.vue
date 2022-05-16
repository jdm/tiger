<template>

  <div @contextmenu="onContextMenu"
    class="flex flex-nowrap flex-col h-screen w-screen space-y-5 p-5 bg-plastic-900 overflow-clip select-none">

    <button @click="openFiles" class="place-self-start">Open document</button>

    <div class="flex-1 min-h-0 flex flex-row space-x-5">
      <ContentPane class="basis-96 min-w-0" />
      <WorkbenchPane class="flex-1" />
    </div>

    <TimelinePane class="basis-60" />

  </div>
</template>

<script setup lang="ts">
import { openFiles } from '@/api/local'
import WorkbenchPane from '@/components/WorkbenchPane.vue';
import ContentPane from '@/components/ContentPane.vue'
import TimelinePane from '@/components/TimelinePane.vue'

function onContextMenu(event: Event) {
  event.preventDefault();
}
</script>
