<template>
  <div class="processor-container">
    <!-- 文件夹输入 -->
    <div class="input-section folder-input">
      <div class="drop-zone"
           @click="selectFolder">
        <div class="input-value">{{ folder || 'folder' }}</div>
      </div>
    </div>

    <!-- 输出文件 -->
<!--    <div class="input-section output-input">
      <textarea
          v-model="output"
          placeholder="选择文件输出路径"
          class="styled-textarea"
          rows="1"
          ></textarea>
    </div>-->
    <div class="input-section folder-input">
      <div class="drop-zone"
           @click="selectOutput">
        <div class="input-value">{{ output || `output\n(default:${desktop}\\tolbox_codeFeeder.txt)` }}</div>
      </div>
    </div>

    <!-- 忽略列表部分 -->
    <div class="input-section ignore-section">
      <div class="ignore-container">
        <div class="drop-zone" @click="addIgnore">
          <div class="input-value">ignore</div>
        </div>
        <div class="ignore-tags">
          <div v-for="(item, index) in ignore" :key="index" class="ignore-tag">
            {{ item }}
            <span @click="removeIgnore(index)" class="remove-tag">×</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="action-buttons">
      <button @click="reset" class="action-btn reset-btn">↻</button>
      <button @click="process" class="action-btn process-btn">→</button>
    </div>
  </div>
</template>

<script setup>
import {onMounted, ref} from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import { desktopDir } from '@tauri-apps/api/path'
import {ElMessage} from "element-plus";
let desktop = ref('')

onMounted(async () => {
  try {
    desktop.value = await desktopDir()
  }catch (e){
    ElMessage.error(e.message)
  }
})
const folder = ref('')
const output = ref('')
const ignore = ref([])


// 选择文件夹
async function selectFolder() {
  const selected = await open({
    directory: true,
    multiple: false
  })

  if (selected) folder.value = selected
}
// 选择输出文件
const selectOutput = async () => {
  const selected = await open({
    directory: true,
    multiple: false
  })
  if (selected) output.value = selected + '\\tolbox_codeFeeder.txt'
}
// 添加忽略项
async function addIgnore() {
  const selected = await open({
    directory: true,
    multiple: true
  })
  if (selected) {
    if (Array.isArray(selected)) {
      ignore.value = [...new Set([...ignore.value, ...selected])]
    } else {
      ignore.value.push(selected)
    }
  }
}

// 移除忽略项
function removeIgnore(index) {
  ignore.value.splice(index, 1)
}

// 重置
function reset() {
  folder.value = ''
  output.value = ''
  ignore.value = []
}


// 处理
async function process() {
  if (!folder.value) {
    ElMessage.error('请选择文件夹')
    return
  }
  if (!output.value) {
    output.value = `${desktop.value}\\tolbox_codeFeeder.txt`
  }

  try {

    const res = await invoke('process_folder_command', {
      folder: folder.value,
      output: output.value,
      ignore: ignore.value
    })
    if (res.success) {
      ElMessage.success(res.message)
    } else {
      ElMessage.error(res.message)
    }
  } catch (error) {
    ElMessage.error(error)
  }
}
</script>

<style scoped>
.processor-container {
  min-height:100vh;
  overflow-y: auto;  /* 内容超出时显示滚动条 */
  width: 100vw;
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  align-items: center;
  background-color: #fff9f0;
  gap: 40px;
  padding: 60px 0;

}

.input-section {
  width: 60%;
  max-width: 500px;
  display: flex;
  align-items: center;
  gap: 20px;
  padding: 25px;
  background: rgba(255, 255, 255, 0.7);
  border-radius: 16px;
  box-shadow: 0 6px 18px rgba(0, 0, 0, 0.05);
  transition: all 0.3s ease;
}


.drop-zone {
  flex-grow: 1;
  cursor: pointer;
  text-align: center;
  padding: 15px;
  border: 2px dashed rgba(212, 160, 23, 0.3);
  border-radius: 8px;
  transition: all 0.3s;
  word-break: break-word; /* 确保容器内文本换行 */
}

.drop-zone:hover {
  border-color: rgba(212, 160, 23, 0.6);
}

.input-value {
  font-size: 1.2rem;
  color: #333;
  font-family: 'Maple Mono Normal NF CN', sans-serif;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.1);
  white-space: pre-line; /* 保留 \n 的换行 */
}
/*.styled-textarea {
  flex-grow: 1;
  background: transparent;
  border: none;
  outline: none;
  font-size: 1.2rem;
  color: #333;
  font-family: 'Maple Mono Normal NF CN', sans-serif;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.1);
  padding: 10px;
  resize: none; !* 禁止手动调整大小 *!
  word-break: break-all; !* 强制换行 *!
}*/

.ignore-container {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.ignore-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  width: 100%;
}
.ignore-tag {
  background: rgba(212, 160, 23, 0.1);
  border: 1px solid rgba(212, 160, 23, 0.3);
  border-radius: 15px;
  padding: 5px 15px;
  font-size: 0.9rem;
  color: #8a6d0b;
  display: inline-flex;
  align-items: center;
}

.remove-tag {
  margin-left: 8px;
  cursor: pointer;
  font-size: 1.1rem;
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

.reset-btn {
  background: rgba(212, 160, 23, 0.1);
  color: #8a6d0b;
}

.process-btn {
  background: #d4a017;
  color: white;
}

.action-btn:hover {
  transform: scale(1.1);
}
/* 动画效果 */
.input-section, .action-buttons {
  animation: fadeInUp 0.8s ease-out;
}



@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.folder-input { animation-delay: 0.1s; }
/*.output-input { animation-delay: 0.2s; }*/
.ignore-section { animation-delay: 0.3s; }
.action-buttons { animation-delay: 0.4s; }
</style>

