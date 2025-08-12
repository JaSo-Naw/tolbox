<template>
  <div class="quickopen-container" @click="closeContextMenu">
    <!--- 应用选择区 --->
    <div class="input-section app-input">
      <div class="drop-zone">
        <div class="app-grid">
          <div
              v-for="app in applications"
              :key="app.id"
              class="app-card"
              :class="{selected: selectedApp && selectedApp.id === app.id}"
              @click="selectApp(app)"
              @contextmenu.prevent="showAppContextMenu($event, app)"
          >
            <span class="app-name">{{ app.name }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 文件/文件夹选择区 -->
    <div class="input-section">
      <div class="drop-zone">
        <div class="file-grid">
          <div
              v-for="file in filesAndDirs"
              :key="file.id"
              class="file-card"
              :class="{ selected: selectedFD && selectedFD.id === file.id , pinned: file.isPinned}"
              @click="selectFD(file)"
              @contextmenu.prevent="showFileContextMenu($event, file)"
          >
            <div class="file-icon">
              {{ file.isDirectory ? '📁' : '📄' }}
            </div>
            <div class="file-info">
              <div class="file-name">{{ getFileName(file.path) }}</div>
              <div class="file-path">{{ file.path }}</div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="action-buttons">
      <button @click="clean" class="action-btn clean-btn">🗑️</button>
      <button @click="openFileDialog" class="action-btn add-file-btn">file</button>
      <button @click="openDirDialog" class="action-btn add-dir-btn">dir</button>
      <button @click="showAddAppDialog" class="action-btn add-app-btn">+</button>
      <button @click="reset" class="action-btn reset-btn">↻</button>
      <button
          @click="openApp"
          class="action-btn process-btn"
          :disabled="(!selectedApp & !selectedFD)"
      >→</button>
    </div>

    <!-- 右键菜单 -->
    <div
        v-if="contextMenu.show"
        class="context-menu"
        :style="{ top: `${contextMenu.y}px`, left: `${contextMenu.x}px` }"
        @click.stop
    >
      <div v-show="contextMenu.type === 'file' " class="menu-item" @click="pinItem">Pin</div>
      <div class="menu-item" @click="removeItem">Remove</div>
    </div>

    <!-- 添加/编辑应用对话框 - 自定义样式 -->
    <div v-if="appDialog.visible" class="custom-dialog-overlay" >
      <div class="custom-dialog">
        <div class="dialog-header">
          <h3>{{ '添加' }}</h3>
          <button class="close-btn" @click="appDialog.visible = false">×</button>
        </div>
        <div class="dialog-body">
          <div class="form-group">
            <label>应用路径</label>
            <div class="path-input">
              <input v-model="appDialog.form.path" readonly />
              <button @click="selectAppPath">选择</button>
            </div>
          </div>
          <div class="form-group">
            <label>应用名称</label>
            <input v-model="appDialog.form.name" />
          </div>
        </div>
        <div class="dialog-footer">
          <button class="cancel-btn" @click="appDialog.visible = false">取消</button>
          <button class="confirm-btn" @click="confirmAddApp">确认</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { basename } from '@tauri-apps/api/path'
import { invoke } from '@tauri-apps/api/core'
import {
  getPinnedApps,
  getPinnedFiles,
  upsertPinnedApp,
  upsertPinnedFile,
  deletePinnedApp,
  deletePinnedFile
} from '@/db/quickOpen/index.js'
import {ElMessage} from "element-plus";


// 初始化数据库并加载数据
onMounted(async () => {
  await loadPinnedApps()
  await loadPinnedFiles()
})

// 应用列表
const applications = ref([])
// 选择的文件列表
const filesAndDirs = ref([])
// 当前选中的应用
const selectedApp = ref(null)
// 当前选中的文件/目录
const selectedFD = ref(null)

// 右键菜单相关
const contextMenu = ref({
  show: false,
  x: 0,
  y: 0,
  type: '', // 'app' | 'file'
  data: null
})

// 添加应用对话框
const appDialog = ref({
  visible: false,
  form: {
    path: '',
    name: '',
  },
})



// 加载已Pin的应用
async function loadPinnedApps() {
  try {
    const apps = await getPinnedApps()
    applications.value = apps.map(app => ({
      id: app.id,
      name: app.name,
      path: app.path
    }))
  } catch (err) {
    console.error('加载pin应用失败:', err)
  }
}

// 加载已Pin的文件
async function loadPinnedFiles() {
  try {
    const files = await getPinnedFiles()
    filesAndDirs.value = files.map(file => ({
      id: file.id,
      path: file.path,
      isDirectory: file.is_dir,
      isPinned: true
    }))

  } catch (err) {
    console.error('加载pin文件失败:', err)
  }
}


// 关闭右键菜单
function closeContextMenu() {
  if (contextMenu.value.show) {
    contextMenu.value.show = false
  }
}

// 获取文件名
function getFileName(path) {
  return path.split(/[/\\]/).pop()
}

// 显示添加应用对话框
function showAddAppDialog() {
  appDialog.value = {
    visible: true,
    form: {
      path: '',
      name: ''
    },
  }
}

// 选择应用路径
async function selectAppPath() {
  try {
    const path = await open({
      multiple: false,
      directory: false,
      filters: [{
        name: '应用程序',
        extensions: ['exe', 'app', 'dmg']
      }]
    })

    if (path) {
      appDialog.value.form.path = path
      // 自动填充名称
      appDialog.value.form.name = (await basename(path)).replace(/\.[^/.]+$/, '')
    }
  } catch (err) {
    console.error('选择应用失败:', err)
  }
}

// 确认添加/编辑应用
async function confirmAddApp() {
  try {
    const { path, name } = appDialog.value.form

    if (!path || !name ) {
      console.warn('请填写完整信息')
      return
    }

    await upsertPinnedApp({name, path})
    appDialog.value.visible = false
    await loadPinnedApps()
  } catch (err) {
    console.error('操作失败:', err)
  }
}

// 选择应用
function selectApp(app) {
  selectedApp.value = app
}

//选择文件
function selectFD(file) {
  selectedFD.value = file
}

// 打开文件对话框
async function openFileDialog() {
  try {
    const path = await open({
      multiple: true,
      directory: false
    })

    if (path) {
      const newFiles = path.map(p => ({
        id: 'f'+ Date.now(),
        path: p,
        isDirectory: false,
        isPinned: false
      }))
      filesAndDirs.value = [...newFiles,...filesAndDirs.value]
    }
  } catch (err) {
    console.error('选择文件失败:', err)
  }
}

// 打开文件夹对话框
async function openDirDialog() {
  try {
    const path = await open({
      multiple: true,
      directory: true
    })

    if (path) {
      const newDirs = path.map(p => ({
        id: 'd'+ Date.now(),
        path: p,
        isDirectory: true,
        isPinned: false
      }))
      filesAndDirs.value = [...newDirs,...filesAndDirs.value]
    }
  } catch (err) {
    console.error('选择文件夹失败:', err)
  }
}

// 显示应用右键菜单
function showAppContextMenu(e, app) {
  contextMenu.value = {
    show: true,
    x: e.clientX,
    y: e.clientY,
    type: 'app',
    data: app
  }
}

// 显示文件右键菜单
function showFileContextMenu(e, file) {
  contextMenu.value = {
    show: true,
    x: e.clientX,
    y: e.clientY,
    type: 'file',
    data: file
  }
}

// Pin项目
async function pinItem() {
  try {
    const { type, data } = contextMenu.value
    if (data.isPinned) {
      ElMessage.warning("已经被pin住")
      contextMenu.value.show = false
      return
    }

    if (type === 'file') {
      await upsertPinnedFile(data)
      let save = filesAndDirs.value.filter( file => !file.isPinned && file.id!==data.id )
      await loadPinnedFiles()
      filesAndDirs.value = [...save, ...filesAndDirs.value]
    }

    contextMenu.value.show = false
  } catch (err) {
    console.error('Pin失败:', err)
  }
}

// 移除项目
async function removeItem() {
  try {
    const { type, data } = contextMenu.value

    if (type === 'app') {
      await deletePinnedApp(data)
      await loadPinnedApps()
    } else {
      await deletePinnedFile(data)
      let save = filesAndDirs.value.filter( file => !file.isPinned && file.id!==data.id )
      await loadPinnedFiles()
      filesAndDirs.value = [...save, ...filesAndDirs.value]
    }
    contextMenu.value.show = false
  } catch (err) {
    console.error('移除失败:', err)
  }
}



// 调用应用打开文件
async function openApp() {
  try {

    await invoke('quick_open', {
      path: selectedFD.value?.path,
      app: selectedApp.value?.path
    })

  } catch (err) {
    console.error('打开失败:', err)
    ElMessage.error(err)
  }
}



// 重置选择
function reset() {
  selectedApp.value = null
  selectedFD.value = null
}

//清空
async function clean() {
  await loadPinnedApps()
  await loadPinnedFiles()
}
</script>

<style scoped>
.quickopen-container {
  min-height: 100vh;
  overflow-y: auto;
  width: 100vw;
  display: flex;
  flex-direction: column;
  align-items: center;
  background-color: #fff9f0;
  gap: 40px;
  padding: 60px 0;
}

.input-section {
  width: 60%;
  max-width: 800px;
  padding: 25px;
  background: rgba(255, 255, 255, 0.7);
  border-radius: 16px;
  box-shadow: 0 6px 18px rgba(0, 0, 0, 0.05);
}

.drop-zone {
  flex-grow: 1;
  text-align: center;
  padding: 15px;
  border: 2px dashed rgba(212, 160, 23, 0.3);
  border-radius: 8px;
  transition: all 0.3s;
}

.drop-zone:hover {
  border-color: rgba(212, 160, 23, 0.6);
}

.app-name {
  max-width: 100%;
  display: inline-block;
  white-space: nowrap;       /* 不换行 */
  overflow: hidden;          /* 超出隐藏 */
  text-overflow: ellipsis;   /* 省略号 */
}
.app-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
  gap: 15px;
  margin-top: 15px;
}

.app-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 10px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s;
}

.app-card:hover {
  background: #f0f0f0;
}

.app-card.selected {
  background-color: #ecf5ff;
  border: 1px solid #409eff;
}





.file-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 15px;
  margin-top: 15px;
}

.file-card {
  display: flex;
  align-items: center;
  padding: 12px;
  border-radius: 8px;
  background: #f9f9f9;
  cursor: pointer;
  transition: all 0.3s;
}

.file-card:hover {
  background: #f0f0f0;
}

.file-card.selected {
  background-color: #ecf5ff;
  border: 1px solid #409eff;
}
.file-card.pinned {
  border: 1px solid red;
}


.file-icon {
  margin-right: 12px;
  font-size: 1.5rem;
}

.file-info {
  flex: 1;
  overflow: hidden;
}

.file-name {
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-path {
  font-size: 0.8rem;
  color: #666;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.action-buttons {
  display: flex;
  gap: 30px;
}

.action-btn {
  width: 60px;
  height: 60px;
  border-radius: 50%;
  border: none;
  font-size: 1.8rem;
  cursor: pointer;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.action-btn:hover {
  background-color: #f6cde4;
}

.add-file-btn {
  font-size: 1.2rem;
  background: #ffefd1;
  color: #b27878;
  rotate: -20deg;
}

.add-dir-btn {
  font-size: 1.2rem;
  background: #ffefd1;
  color: #9f7474;
  rotate: -20deg;
}

.add-app-btn {
  background: #fdebc7;
  color: #b66b6b;
}

.reset-btn {
  background: rgba(212, 160, 23, 0.1);
  color: #8a6d0b;
}
.clean-btn {
  background: #f3e8cc;
  font-size: 1.2rem;
}

.process-btn {
  background: #d4a017;
  color: white;
}

.process-btn:disabled {
  background: rgba(212, 160, 23, 0.3);
  cursor: not-allowed;
}

/* 右键菜单样式 */
.context-menu {
  position: fixed;
  background: white;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  z-index: 999;
  min-width: 120px;
  overflow: hidden;
}

.menu-item {
  padding: 10px 16px;
  cursor: pointer;
  transition: background 0.2s;
}

.menu-item:hover {
  background: #f5f7fa;
}

/* 自定义对话框样式 */
.custom-dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.custom-dialog {
  background: white;
  border-radius: 12px;
  width: 500px;
  max-width: 90%;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
  overflow: hidden;
}

.dialog-header {
  padding: 16px 20px;
  background: #f8f8f8;
  border-bottom: 1px solid #eee;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.dialog-header h3 {
  margin: 0;
  font-size: 1.2rem;
  color: #333;
}

.close-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: #999;
  padding: 0 8px;
}

.close-btn:hover {
  color: #666;
}

.dialog-body {
  padding: 20px;
}

.form-group {
  margin-bottom: 20px;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  font-weight: 500;
  color: #555;
}

.form-group input {
  width: 100%;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 1rem;
}

.path-input {
  display: flex;
}

.path-input input {
  flex: 1;
  border-top-right-radius: 0;
  border-bottom-right-radius: 0;
  border-right: none;
}

.path-input button {
  padding: 10px 16px;
  background: #f0f0f0;
  border: 1px solid #ddd;
  border-left: none;
  border-top-right-radius: 6px;
  border-bottom-right-radius: 6px;
  cursor: pointer;
}

.path-input button:hover {
  background: #e6e6e6;
}

.dialog-footer {
  padding: 16px 20px;
  background: #f8f8f8;
  border-top: 1px solid #eee;
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.cancel-btn, .confirm-btn {
  padding: 8px 16px;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 500;
}

.cancel-btn {
  background: #f0f0f0;
  border: 1px solid #ddd;
  color: #555;
}

.cancel-btn:hover {
  background: #e6e6e6;
}

.confirm-btn {
  background: #d4a017;
  border: 1px solid #c29115;
  color: white;
}

.confirm-btn:hover {
  background: #c29115;
}
</style>

