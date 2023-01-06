<template>
	<ModalDialog title="Relocate Frames" :icon="PhotoIcon">
		<template #body>
			<div class="flex flex-col">
				<p class="pb-1 w-[800px]">Some frames in <span class="italic font-semibold text-orange-500">{{
				app.currentDocument?.name
				}}</span> could not be found. Please specify updated locations for the files below.</p>
				<div class="max-h-[500px] flex flex-col gap-8 pr-4 my-10 overflow-y-scroll styled-scrollbars">
					<RelocateFrame v-for="frame of framesToRelocate" :frame="frame" />
				</div>
			</div>
		</template>
		<template #actions>
			<Button label="Relocate" @click="endRelocateFrames" tabbable positive class="w-20" />
			<Button label="Cancel" @click="cancelRelocateFrames" tabbable class="w-20" />
		</template>
	</ModalDialog>
</template>

<script setup lang="ts">
import { computed } from "vue"
import { PhotoIcon } from "@heroicons/vue/24/outline"
import { useAppStore } from "@/stores/app"
import { cancelRelocateFrames, endRelocateFrames } from "@/api/document"
import RelocateFrame from "@/components/dialogs/RelocateFrame.vue"
import Button from "@/components/basic/Button.vue"
import ModalDialog from "@/components/basic/ModalDialog.vue"

const app = useAppStore();

const framesToRelocate = computed(() => {
	if (app.currentDocument == null) {
		return [];
	}
	return app.currentDocument.sheet.frames.filter(f => f.missingOnDisk);
});
</script>