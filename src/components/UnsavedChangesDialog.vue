<template>
	<ModalDialog title="Unsaved Changes" icon="ExclamationIcon">
		<template #body>
			<p><span class="italic font-semibold text-orange-500">{{ app.currentDocument?.name }}</span> has been
				modified. Would you like to save changes before closing the spritesheet?</p>
		</template>
		<template #actions>
			<Button label="Yes" @click="save" :positive="true" />
			<Button label="No" @click="closeWithoutSaving" :danger="true" />
			<Button label="Cancel" @click="cancelExit" />
		</template>
	</ModalDialog>
</template>

<script setup lang="ts">
import Button from '@/components/basic/Button.vue';
import ModalDialog from '@/components/basic/ModalDialog.vue';
import { useAppStore } from '@/stores/app'
import { cancelExit } from '@/api/app'
import { closeWithoutSaving, save } from '@/api/document'

const app = useAppStore();
</script>