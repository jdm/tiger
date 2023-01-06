<template>
	<div class="pointer-events-none">
		<ScreenCover :visible="app.activeModalId != null" />
		<Transition name="slide-fade">
			<div v-if="app.activeModalId" :key="app.activeModalId"
				class="absolute inset-0 pointer-events-auto flex items-center justify-center">
				<ErrorDialog v-if="app.error" :error="app.error" />
				<UnsavedChangesDialog v-else-if="app.currentDocument?.wasCloseRequested" />
				<RelocateFramesDialog v-else-if="!!app.currentDocument?.framesBeingRelocated" />
			</div>
		</Transition>
	</div>
</template>

<script setup lang="ts">
import { useAppStore } from "@/stores/app"
import ScreenCover from "@/components/basic/ScreenCover.vue"
import ErrorDialog from "@/components/dialogs/ErrorDialog.vue"
import RelocateFramesDialog from "@/components/dialogs/RelocateFramesDialog.vue"
import UnsavedChangesDialog from "@/components/dialogs/UnsavedChangesDialog.vue"

const app = useAppStore();
</script>

<style scoped>
.slide-fade-enter-active {
	transition-property: opacity, transform;
	transition: 0.2s ease-out;
}

.slide-fade-leave-active {
	transition-property: opacity, transform;
	transition: 0.1s ease-in;
}

.slide-fade-enter-from,
.slide-fade-leave-to {
	opacity: 0;
	transform: translateY(1rem);
}
</style>