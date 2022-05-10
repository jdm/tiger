<script setup lang="ts">
import { useStore } from '@/stores/app'
const store = useStore()
</script>

<template>
  Oink oink
  {{ store.test }}
</template>