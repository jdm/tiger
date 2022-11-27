<template>
	<ModalDialog :title="error.title" :icon="ExclamationTriangleIcon" :error="true">
		<template #body>
			<p class="pb-1">{{ error.summary }}</p>
			<div
				class="py-2 pl-4 pr-32 max-w-xl select-text bg-plastic-700 border-4 border-plastic-700 rounded-md text-xs text-plastic-300 overflow-y-scroll styled-scrollbars">
				<p class="whitespace-pre-wrap font-mono">{{ error.details }}</p>
			</div>
		</template>
		<template #actions>
			<Button label="OK" @click="acknowledgeError" class="w-20" />
		</template>
	</ModalDialog>
</template>

<script setup lang="ts">
import { ExclamationTriangleIcon } from "@heroicons/vue/24/outline"
import { UserFacingError } from "@/api/dto"
import { acknowledgeError } from "@/api/app"
import Button from "@/components/basic/Button.vue"
import ModalDialog from "@/components/basic/ModalDialog.vue"

defineProps<{
	error: UserFacingError,
}>();

</script>