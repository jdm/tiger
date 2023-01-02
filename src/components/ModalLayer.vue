<template>
	<div class="pointer-events-none">
		<ScreenCover :visible="activeModalID != null" />
		<Transition name="slide-fade">
			<div v-if="activeModalID" :key="activeModalID"
				class="absolute inset-0 pointer-events-auto flex items-center justify-center">
				<ErrorDialog v-if="app.error" :error="app.error" />
				<UnsavedChangesDialog v-else-if="app.currentDocument?.wasCloseRequested" />
			</div>
		</Transition>
	</div>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { useAppStore } from "@/stores/app"
import ScreenCover from "@/components/basic/ScreenCover.vue"
import ErrorDialog from "@/components/dialogs/ErrorDialog.vue"
import UnsavedChangesDialog from "@/components/dialogs/UnsavedChangesDialog.vue"

const app = useAppStore();

const activeModalID = computed(() => {
	if (app.error != null) {
		return app.error.key;
	} else if (app.currentDocument?.wasCloseRequested) {
		return "closing_" + app.currentDocument.path;
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