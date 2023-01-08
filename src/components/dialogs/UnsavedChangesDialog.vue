<template>
	<ModalDialog title="Unsaved Changes" :icon="ExclamationTriangleIcon">
		<template #body>
			<p class="max-w-md"><span class="italic font-semibold text-orange-500">{{ state.currentDocument?.name
			}}</span> has been
				modified. Would you like to save changes before closing the spritesheet?</p>
		</template>
		<template #actions>
			<Button label="Yes" @click="save" tabbable positive class="w-20" />
			<Button label="No" @click="closeWithoutSaving" tabbable danger class="w-20" />
			<Button label="Cancel" @click="cancelExit" tabbable class="w-20" />
		</template>
	</ModalDialog>
</template>

<script setup lang="ts">
import { ExclamationTriangleIcon } from "@heroicons/vue/24/outline"
import { cancelExit, closeWithoutSaving, save } from "@/backend/api"
import { useStateStore } from "@/stores/state"
import Button from "@/components/basic/Button.vue"
import ModalDialog from "@/components/basic/ModalDialog.vue"

const state = useStateStore();
</script>