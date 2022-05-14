<script setup lang="ts">
import { useAppStore } from '@/stores/app'
import { openFiles } from '@/commands/local'
import TabList from '@/components/tabs/TabList.vue'
import Tab from '@/components/tabs/Tab.vue'
import Workbench from '@/components/Workbench.vue';

const app = useAppStore()

</script>

<template>

  <div class="flex flex-col h-screen w-screen overflow-clip select-none">

    <button @click="openFiles" class="place-self-start">Open document</button>

    <TabList>
      <Tab v-for="document in app.documents" :selected="document.path == app.currentDocumentPath">
        {{ document.name }}
      </Tab>
    </TabList>

    <div class="flex-1 grid grid-cols-5 grid-rows-5">

      <Workbench class="row-start-1 col-start-2 col-span-4 row-span-4">
      </Workbench>

      <div class="row-start-1 col-start-1 col-span-1 row-span-5 flex flex-col">
        <h2 class="px-3 py-1 bg-zinc-800 text-slate-50 font-semibold">
          Content</h2>
        <div class="flex-1 px-4 py-2 overflow-y-auto bg-neutral-900 text-zinc-300">
          <div class="overflow-x-clip">
            <ul v-if="app.currentDocument" v-for="frame in app.currentDocument.sheet.frames">
              <!-- TODO layout breaks when this is too tall -->
              <li>{{ frame }}</li>
            </ul>
          </div>
        </div>
      </div>

      <div class="row-start-5 col-start-2 row-span-1 col-span-4 flex flex-col">
        <h2 class="px-3 py-1 bg-zinc-800 text-slate-50 font-semibold">
          Timeline</h2>
        <div class="flex-1 px-4 py-2 overflow-y-auto bg-neutral-900 border-l border-zinc-800 text-gray-400">
          Beep boop I'm a timeline
        </div>
      </div>
    </div>

  </div>

</template>
