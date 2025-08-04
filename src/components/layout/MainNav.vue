<template>

  <div class="main-nav">
    <!-- 导航区 -->
    <div class="nav-items">
      <div
          v-for="item in navItems"
          :key="item.id"
          class="nav-item"
          :class="{ active: activeNav === item.id }"
          @mouseenter="showTooltip(item.text, $event)"
          @mouseleave="hideTooltip"
          @click="switchNav(item.id)"
      >
        <Icon :name="item.icon" size="26" />
      </div>
    </div>

    <!-- 底部设置按钮 -->
    <div class="nav-footer">
      <div class="setting-wrapper">
        <button
            class="setting-btn"
            @mouseenter="showTooltip('设置', $event)"
            @mouseleave="hideTooltip"
            @click="showSetting"
        >
          <Icon name="settings" size="20" />
        </button>
      </div>
    </div>

    <!-- Tooltip -->
    <transition name="fade-slide">
      <div
          v-if="tooltip.visible"
          class="tooltip"
          :style="{ top: tooltip.y + 'px', left: tooltip.x + 'px' }"
      >
        {{ tooltip.text }}
      </div>
    </transition>

    <!-- 设置菜单 -->
    <transition name="fade-slide">
      <div v-if="showFanMenu" class="fan-menu-container">
        <div class="fan-menu">
          <template v-for="item in menuItems" :key="item.id || item.type">
            <!-- 分隔线 -->
            <div v-if="item.type === 'divider'" class="menu-divider"></div>

            <!-- 菜单项 -->
            <div
                v-else
                class="menu-item"
                :class="{ 'has-submenu': item.submenu }"
                @mouseenter="item.submenu ? showSubMenu(item.id) : null"
                @click="handleMenuItemClick(item)"
            >
              {{ item.text }}

              <!-- 子菜单 -->
              <div v-if="item.submenu && activeSubMenu === item.id" class="sub-menu">
                <div
                    v-for="subItem in item.submenu"
                    :key="subItem.id"
                    class="menu-item"
                    @click="handleMenuItemClick(subItem)"
                >
                  {{ subItem.text }}
                </div>
              </div>
            </div>
          </template>
        </div>
      </div>
    </transition>
  </div>



</template>

<script setup>
import {onBeforeUnmount, onMounted, reactive, ref} from 'vue'
import { useLayoutStore } from '@/stores'
import Icon from '@/components/common/Icon.vue'
import { getCurrentWindow } from '@tauri-apps/api/window'
import {mainNav, settingItem} from '@/common/layout.js'

const layoutStore = useLayoutStore()

/* 导航区 */
const navItems = mainNav
const activeNav = ref(layoutStore.activeNav)
const switchNav = (id) => {
  activeNav.value = id
  layoutStore.setActiveNav(id)
}

/* Tooltip */
const tooltip = reactive({ visible: false, text: '', x: 0, y: 0 })
const showTooltip = (text, event) => {
  tooltip.text = text
  tooltip.visible = true
  const rect = event.currentTarget.getBoundingClientRect()
  tooltip.x = rect.right + 10
  tooltip.y = rect.top + rect.height / 2
}
const hideTooltip = () => (tooltip.visible = false)


// 菜单状态
const showFanMenu = ref(false)
const activeSubMenu = ref(null)

// 菜单数据
const menuItems = settingItem

// 显示菜单及其子菜单
const showSetting = () => {
  showFanMenu.value = !showFanMenu.value
  if (showFanMenu.value) {
    activeSubMenu.value = null
  }
}

const showSubMenu = (menuId) => {
  activeSubMenu.value = menuId
}

// 点击菜单项，触发其功能
const handleMenuItemClick = (item) => {
  if (item.action) {
    showFanMenu.value = false
    item.action()
  } else if (item.submenu) {
    showSubMenu(item.id)
  }
}

// 点击外部，关闭设置菜单
const handleClickOutside = (event) => {
  const fanMenu = document.querySelector('.fan-menu-container')
  if (fanMenu && !fanMenu.contains(event.target) &&
      !event.target.closest('.setting-btn')) {
    showFanMenu.value = false
    activeSubMenu.value = null
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onBeforeUnmount(() => {
  document.removeEventListener('click', handleClickOutside)
})

</script>

<style scoped>
.main-nav {
  width: 64px;
  background: var(--bg-nav);
  border-right: 1px solid #e5e7eb;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: center;
  padding: 12px 0;
  user-select: none;
}

/* 导航 */
.nav-items {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  flex: 1;
  margin-top: 20px;

}
.nav-item {
  width: 44px;
  height: 44px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-light);
  transition: all 0.25s ease;

}
.nav-item:hover {
  background: var(--bg-hover);
  color: var(--primary);

}
.nav-item.active {
  background: var(--bg-active);
  color: var(--primary);
  font-weight: 600;
}

/* 设置按钮 */
.setting-btn {
  width: 40px;
  height: 40px;
  border-radius: 10px;
  border-color: #feffff;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-light);
  cursor: pointer;
}
.setting-btn:hover {
  background: var(--bg-hover);
  color: var(--primary);
}

/* Tooltip */
.tooltip {
  position: fixed;
  background: #ffffff;
  border: 1px solid #e5e7eb;
  color: var(--text);
  font-size: 13px;
  padding: 6px 10px;
  border-radius: 6px;
  white-space: nowrap;
  box-shadow: 0 2px 8px rgba(0,0,0,0.08);
  pointer-events: none;
  z-index: 1000;
  transform: translateY(-50%);
}
/*菜单*/
.fan-menu-container {
  position: absolute;
  bottom: 60px;
  left: 10px;
  z-index: 1000;
}

/* 一级菜单 */
.fan-menu {
  background: #fffdfa;
  border-radius: 12px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
  padding: 8px;
  min-width: 160px;
  border: 1px solid #e5d8c0;
}

/* 二级菜单 */
.sub-menu {
  position: absolute;
  left: 100%;
  top: 0;
  background: #fffef7;
  border-radius: 12px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
  padding: 8px;
  min-width: 160px;
  border: 1px solid #e5d8c0;
  margin-left: 8px;
}

/* 菜单项通用样式 */
.menu-item {
  padding: 8px 12px;
  border-radius: 6px;
  display: flex;
  align-items: center;
  cursor: pointer;
  color: #5a4a3a;
  font-size: 14px;
  transition: all 0.2s ease;
  position: relative;
}

.menu-item:hover {
  background: #fff0f0;
  color: #3a2e24;
}

.menu-item .icon {
  margin-right: 8px;
  color: #8a7b69;
}

.menu-item.has-submenu::after {
  content: "›";
  margin-left: auto;
  font-size: 16px;
  color: #8a7b69;
}

/* 分隔线 */
.menu-divider {
  height: 1px;
  background: #e5d8c0;
  margin: 6px 0;
}


</style>
