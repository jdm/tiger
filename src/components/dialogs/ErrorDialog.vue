<template>
	<ModalDialog :title="error.title" :icon="ExclamationTriangleIcon" :error="true">
		<template #body>
			<p class="pb-2" v-html="error.summary" />
			<div
				class="p-4 max-w-2xl max-h-80 select-text bg-plastic-700 border-4 border-plastic-700 rounded-md text-xs text-plastic-300 overflow-scroll styled-scrollbars">
				<div class="w-fit min-w-[500px] whitespace-pre font-mono">{{ error.details }}</div>
			</div>
		</template>
		<template #actions>
			<Button label="OK" @click="acknowledgeError" tabbable class="w-20" />
		</template>
	</ModalDialog>
</template>

<script setup lang="ts">
import { ExclamationTriangleIcon } from "@heroicons/vue/24/outline"
import { acknowledgeError } from "@/backend/api"
import { UserFacingError } from "@/backend/dto"
import Button from "@/components/basic/Button.vue"
import ModalDialog from "@/components/basic/ModalDialog.vue"

defineProps<{
	error: UserFacingError,
}>();

</script>