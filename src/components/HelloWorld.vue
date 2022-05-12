<script setup lang="ts">
import { useAppStore } from '@/stores/app'
import { openDocument } from '@/commands/app'

const app = useAppStore()

function o() {
  openDocument("C:\\Users\\agersant\\Documents\\GitHub\\crystal\\source\\arpg\\sprite\\duran\\\\duran.tiger");
}
</script>

<template>
  <p>Current document: {{ app.currentDocument?.source ?? "none" }}</p>
  <div v-if="app.currentDocument">
    <ul v-for="frame in app.currentDocument.sheet.frames">
      <li>{{ frame }}</li>
    </ul>
  </div>

  <button @click="o">Open document</button>
  <p>Open documents:</p>
  <ul v-for="path in app.documents">
    <li>{{ path }}</li>
  </ul>
</template>