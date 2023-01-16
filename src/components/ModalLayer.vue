<template>
	<div class="pointer-events-none">
		<ScreenCover :visible="activeModalId != null" />
		<Transition name="slide-fade">
			<div v-if="activeModalId" :key="activeModalId"
				class="absolute inset-0 pointer-events-auto flex items-center justify-center">
				<ErrorDialog v-if="state.error" :error="state.error" />
				<UnsavedChangesDialog v-else-if="state.currentDocument?.wasCloseRequested" />
				<RelocateFramesDialog v-else-if="!!state.currentDocument?.framesBeingRelocated" />
			</div>
		</Transition>
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { useStateStore } from "@/stores/state"
import ScreenCover from "@/components/basic/ScreenCover.vue"
import ErrorDialog from "@/components/dialogs/ErrorDialog.vue"
import RelocateFramesDialog from "@/components/dialogs/RelocateFramesDialog.vue"
import UnsavedChangesDialog from "@/components/dialogs/UnsavedChangesDialog.vue"

const state = useStateStore();

const activeModalId = computed(() => {
	if (state.error != null) {
	return state.error.key;
	} else if (state.currentDocument?.wasCloseRequested) {
	return "closing_" + state.currentDocument.path;
	} else if (!!state.currentDocument?.framesBeingRelocated) {
	return "relocating_" + state.currentDocument.path;
	}
	return null;
});
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