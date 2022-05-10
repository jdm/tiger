import { defineStore } from 'pinia'

export const useStore = defineStore('app', {
	state: () => ({
		test: 'oink',
		open_documents: [],
	})
})
