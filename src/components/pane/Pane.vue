<template>
	<div class="flex flex-col">
		<div class="flex bg-plastic-900">
			<slot name="header"></slot>
		</div>
		<div class="flex-1 bg-plastic-700 text-white min-h-0">
			<slot name="content"></slot>
		</div>
	</div>
</template>
