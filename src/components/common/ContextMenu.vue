<template>
  <div @click="hideMenu" @contextmenu.prevent="onContextMenuGlobal">
    <slot></slot>

    <div
        v-if="visible"
        class="context-menu"
        :style="{ top: y + 'px', left: x + 'px' }"
        @click.stop
    >
      <div class="context-menu-item" @click="execCommand('copy')">复制</div>
      <div class="context-menu-item" @click="execCommand('cut')">剪切</div>
      <div class="context-menu-item" @click="execCommand('paste')">粘贴</div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const visible = ref(false)
const x = ref(0)
const y = ref(0)
let currentTarget = null

function onContextMenuGlobal(e) {
  const el = e.target
  const isEditable = el.matches('input, textarea, [contenteditable="true"], [contenteditable]:not([contenteditable="false"])')

  const hasAllowedClass = el.classList.contains('CONTEXT_MENU_ALLOWED')

  if (isEditable || hasAllowedClass) {
    e.preventDefault()
    visible.value = true
    x.value = e.clientX
    y.value = e.clientY
    currentTarget = el
  } else {
    visible.value = false
  }
}

function hideMenu() {
  visible.value = false
}

async function execCommand(command) {
  if (!currentTarget) return
  currentTarget.focus()

  try {
    if (command === 'copy') {
      await copyText()
    } else if (command === 'cut') {
      await cutText()
    } else if (command === 'paste') {
      await pasteText()
    }
  } catch (err) {
    console.log(`操作失败：${err.message}`)
  }
  hideMenu()
}

async function copyText() {
  const text = getSelectedTextOrInputValue()
  if (navigator.clipboard && navigator.clipboard.writeText) {
    await navigator.clipboard.writeText(text)
  } else {
    fallbackExecCommand('copy')
  }
}

async function cutText() {
  const text = getSelectedTextOrInputValue()
  if (navigator.clipboard && navigator.clipboard.writeText) {
    await navigator.clipboard.writeText(text)
    clearSelectedTextOrInput()
  } else {
    fallbackExecCommand('cut')
  }
}

async function pasteText() {
  if (navigator.clipboard && navigator.clipboard.readText) {
    const text = await navigator.clipboard.readText()
    insertTextAtCursor(text)
  } else {
    fallbackExecCommand('paste')
  }
}

// 获取选中文本或输入框当前值
function getSelectedTextOrInputValue() {
  if (window.getSelection && window.getSelection().toString()) {
    return window.getSelection().toString()
  } else if ('value' in currentTarget) {
    // 输入框没选中文本时返回全部值
    return currentTarget.value || ''
  }
  return ''
}

// 清空输入框选中文本或全部（剪切后清除）
function clearSelectedTextOrInput() {
  if ('selectionStart' in currentTarget && 'selectionEnd' in currentTarget) {
    const start = currentTarget.selectionStart
    const end = currentTarget.selectionEnd
    if (start !== end) {
      const val = currentTarget.value
      currentTarget.value = val.slice(0, start) + val.slice(end)
      currentTarget.selectionStart = currentTarget.selectionEnd = start
      return
    }
  }
  // 如果没选中，清空全部
  if ('value' in currentTarget) {
    currentTarget.value = ''
  }
}

// 插入文本到光标位置（粘贴）
function insertTextAtCursor(text) {
  if ('selectionStart' in currentTarget && 'selectionEnd' in currentTarget) {
    const start = currentTarget.selectionStart
    const end = currentTarget.selectionEnd
    const val = currentTarget.value || ''
    currentTarget.value = val.slice(0, start) + text + val.slice(end)
    const pos = start + text.length
    currentTarget.selectionStart = currentTarget.selectionEnd = pos
  }
}

// 旧版execCommand回退
function fallbackExecCommand(command) {
  try {
    document.execCommand(command)
  } catch(err) {
    console.log(err)
  }
}
</script>

<style scoped>
.context-menu {
  position: fixed;
  z-index: 10000;
  background: white;
  border: 1px solid #ccc;
  border-radius: 6px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  min-width: 100px;
  user-select: none;
  padding: 4px 0;
}

.context-menu-item {
  padding: 6px 16px;
  cursor: pointer;
  font-size: 14px;
}

.context-menu-item:hover {
  background-color: #f0f0f0;
}
</style>
