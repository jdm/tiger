<template>
	<div class="flex h-full w-full justify-end items-end px-14 py-16 pointer-events-none">
		<Notifications ref="notifications" />
	</div>
</template>

<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { onMounted, Ref, ref } from "vue";
import { revealInExplorer, showErrorMessage } from "@/backend/api";
import { ExportError, ExportSuccess } from "@/backend/dto";
import { DocumentCheckIcon, ExclamationTriangleIcon } from "@heroicons/vue/20/solid"
import Notifications from "@/components/basic/Notifications.vue"

const notifications: Ref<InstanceType<typeof Notifications> | null> = ref(null);

onMounted(() => {
  listen("export-error", event => {
	const details = event.payload as ExportError;
	const title = "Export Error";
	const description = `Something went wrong while exporting <span class="italic font-medium text-amber-200">${details.documentName}</span>.`;
	const modalDescription = `Something went wrong while exporting <span class="italic font-medium text-orange-500">${details.documentName}</span>:`;
	notifications.value?.push({
		flavor: "error",
		title: title,
		icon: ExclamationTriangleIcon,
		description: description,
		actions: [{
			text: "View details",
			callback: () => {
				showErrorMessage(title, modalDescription, details.error);
			},
		}],
	});
  });

  listen("export-success", event => {
	const details = event.payload as ExportSuccess;
	notifications.value?.push({
		flavor: "success",
		title: "Export Complete",
		icon: DocumentCheckIcon,
		description: `Successfully exported as <span class="font-medium text-amber-200">${details.atlasImageFileName}</span> and <span class="font-medium text-yellow-200">${details.metadataFileName}</span>.`,
		actions: [{
			text: "View files",
			callback: () => {
				revealInExplorer(details.atlasImageFilePath);
				revealInExplorer(details.metadataFilePath);
			},
		}],
	});
  });
});

</script>
