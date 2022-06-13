<template>
	<InputText>
		<template #after>
			<span class="inline-flex items-center px-3 rounded-md
				bg-plastic-600
				border border-l-0 border-t-plastic-500 border-b-plastic-900 border-r-plastic-800">
				<FolderIcon class="w-5 h-5 text-amber-400" />
			</span>
		</template>
	</InputText>
</template>


<script setup lang="ts">
import { FolderIcon } from "@heroicons/vue/solid"
import InputText from "@/components/basic/InputText.vue"

</script>