<script setup lang="ts">
import { useAppStore } from '@/stores/app'
import { openFiles } from '@/api/local'
import Pane from '@/components/pane/Pane.vue'
import PaneTab from '@/components/pane/PaneTab.vue'
import TabList from '@/components/tabs/TabList.vue'
import Tab from '@/components/tabs/Tab.vue'
import Workbench from '@/components/Workbench.vue';
import { closeDocument, focusDocument } from '@/api/app'
import ContentPane from './ContentPane.vue'

const app = useAppStore()

</script>

<template>

  <div class="flex flex-nowrap flex-col h-screen w-screen overflow-clip select-none">

    <button @click="openFiles" class="place-self-start">Open document</button>

    <TabList>
      <Tab v-for="document in app.documents" @select="focusDocument(document.path)"
        @close="closeDocument(document.path)" :selected="document.path == app.currentDocumentPath">
        {{ document.name }}
      </Tab>
    </TabList>

    <div class="flex-1 min-h-0 flex flex-row">

      <div class="flex-1 flex flex-col">
        <Workbench class="flex-1">
        </Workbench>

        <Pane class="basis-60">
          <template #header>
            <PaneTab :selected="true">Timeline</PaneTab>
          </template>
          <template #content>Beep boop I'm a timeline</template>
        </Pane>
      </div>

      <ContentPane class="basis-96 min-w-0" />

    </div>

  </div>

</template>
