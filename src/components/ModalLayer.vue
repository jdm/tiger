<template>
	<div class="pointer-events-none">
		<Transition name="fade">
			<div v-if="activeModalID" class="absolute inset-0 pointer-events-auto bg-black/70" />
		</Transition>
		<Transition name="slide-fade">
			<div v-if="activeModalID" :key="activeModalID"
				class="absolute inset-0 pointer-events-auto flex items-center justify-center">
				<UnsavedChangesDialog v-if="app.currentDocument?.wasCloseRequested" />
				<!-- TODO Error dialogs go here -->
			</div>
		</Transition>
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { useAppStore } from "@/stores/app"
import UnsavedChangesDialog from "@/components/dialogs/UnsavedChangesDialog.vue"

const app = useAppStore();

const activeModalID = computed(() => {
	if (app.currentDocument?.wasCloseRequested) {
		return "closing_" + app.currentDocument.path;
	}
	return null;
});
</script>

<style scoped>
.fade-enter-active {
	transition: opacity 0.3s ease-out;
}

.fade-leave-active {
	transition: opacity 0.2s ease-in;
}

.fade-enter-from,
.fade-leave-to {
	opacity: 0;
}

.slide-fade-enter-active {
	transition-property: opacity, transform;
	transition: 0.3s ease-out;
}

.slide-fade-leave-active {
	transition-property: opacity, transform;
	transition: 0.2s ease-in;
}

.slide-fade-enter-from,
.slide-fade-leave-to {
	opacity: 0;
	transform: translateY(1rem);
}
</style>