<template>
	<ModalDialog title="Unsaved Changes" :icon="ExclamationTriangleIcon">
		<template #body>
			<p class="max-w-md"><span class="italic font-semibold text-orange-500">{{ app.currentDocument?.name
			}}</span> has been
				modified. Would you like to save changes before closing the spritesheet?</p>
		</template>
		<template #actions>
			<Button label="Yes" @click="save" :positive="true" class="w-20" />
			<Button label="No" @click="closeWithoutSaving" :danger="true" class="w-20" />
			<Button label="Cancel" @click="cancelExit" class="w-20" />
		</template>
	</ModalDialog>
</template>

<script setup lang="ts">
import { ExclamationTriangleIcon } from "@heroicons/vue/24/outline"
import Button from "@/components/basic/Button.vue"
import ModalDialog from "@/components/basic/ModalDialog.vue"
import { useAppStore } from "@/stores/app"
import { cancelExit } from "@/api/app"
import { closeWithoutSaving, save } from "@/api/document"

const app = useAppStore();
</script>