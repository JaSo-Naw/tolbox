import { defineStore } from 'pinia'
import router from "@/router";

export const useLayoutStore = defineStore('layout', {
  state: () => ({
    activeNav: 'home',
    activeSubNav: '',
  }),
  actions: {
    async setActiveNav(nav) {
      this.activeNav = nav
      await this.setActiveSubNav('')
      if (nav === 'home') {
        await router.push({ name: 'home' })
      }
    },
    async setActiveSubNav(subNav) {
      this.activeSubNav = subNav
      if (subNav) {
        await router.push({ name: subNav })
      } else {
        await router.push({ name: 'home' })
      }


    },
  }
})