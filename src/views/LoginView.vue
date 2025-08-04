<template>
  <div class="login-wrapper">

    <div class="login-container">
      <div class="login-box-and-hint">
        <div class="login-box" data-tauri-drag-region>
          <h1 class="login-title nodrag">alpha-team tolbox</h1>

          <div class="input-group nodrag">
            <el-input
                v-model.trim="secretKey"
                placeholder="Enter your secret key (enter 'quit' to exit)"
                size="large"
                :show-password="true"
                style="width: 100%"
                @keyup.enter="handleKeyLogin"
            />

          </div>

          <el-button
              type="primary"
              size="large"
              class="login-btn"
              @click="handleKeyLogin"
          >
            Verify Key
          </el-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import {onMounted, ref} from 'vue'
import {ElMessage} from 'element-plus'
import {getCurrentWindow} from '@tauri-apps/api/window'
import {invoke} from "@tauri-apps/api/core";
import {
  checkSecretKeyExpiry,
  getSecretKey,
  getSecretKeyExpiry,
  setSecretKeyWithDefaultExpiry
} from "@/db/modules/user.js";



const appWindow = getCurrentWindow()
const secretKey = ref('')

onMounted(async ()=>{
  await checkSecretKeyExpiry()
  secretKey.value = await getSecretKey()
})

const creating = ref(false)
const handleKeyLogin = async () => {
  try {
    if (creating.value) return

    if (!secretKey.value) {
      ElMessage.error("请输入密钥")
      return
    }

    if (secretKey.value === "quit") {
      await appWindow.close()
      return
    }

    //todo 向后端发送请求！并校验后跳转
    if (true) {
      creating.value = true
      //无值说明过期，说明需要重新设置
      const ske = await getSecretKeyExpiry()
      if (ske === '') {
        await setSecretKeyWithDefaultExpiry(secretKey.value)
      }
      //让当前窗口禁用   退出登录修改！
      await invoke('create_main_window')
      await appWindow.close()

    }

  } catch (error) {
    ElMessage.error(`失败: ${error}`)
  }finally {
    creating.value = false
  }
}

</script>

<style scoped>

.nodrag {
  -webkit-app-region: no-drag;
}
.login-wrapper {
  --primary-bg: #d6e2f3;
  --secondary-bg: #e8f0fe;
  --text-primary: #4a5a7b;
  --text-secondary: #7d8b99;
  --border-color: #c0d0e8;
  --shadow-color: rgba(118, 138, 200, 0.15);

  height: 100vh;
  background: linear-gradient(135deg, var(--primary-bg) 0%, var(--secondary-bg) 100%);
  font-family: 'Maple Mono Normal NF CN', 'Helvetica Neue', sans-serif;
}

.login-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  padding: 20px;
}

.login-box {
  width: 420px;
  padding: 40px;
  background: rgba(255, 255, 255, 0.9);
  border-radius: 24px;
  box-shadow: 0 8px 32px var(--shadow-color);
  backdrop-filter: blur(4px);
  border: 1px solid var(--border-color);
  display: flex;
  flex-direction: column;
  gap: 28px;
}

.login-title {
  font-size: 32px;
  letter-spacing: 3px;
  color: var(--text-primary);
  text-align: center;
  margin: 0;
  text-shadow:
      1px 0 0 currentColor,
      0 1px 0 currentColor,
      2px 2px 4px rgba(0,0,0,0.1);
}

.input-group {
  margin-bottom: 16px;
}

:deep(.el-input) {
  --el-input-bg-color: rgba(255,255,255,0.7);
  --el-input-text-color: var(--text-primary);
  --el-input-border: 1px solid var(--border-color);
  --el-input-hover-border: 1px solid #a0b9e0;

  .el-input__wrapper {
    padding: 12px 16px;
    border-radius: 12px;
    transition: all 0.3s ease;
  }

  .el-input__inner::placeholder {
    color: #a8b5c8;
  }
}

.login-btn {
  width: 100%;
  padding: 14px;
  font-size: 16px;
  letter-spacing: 2px;
  border-radius: 12px;
  background: linear-gradient(135deg, #7a8fff 0%, #6ec1ff 100%);
  border: none;
  margin-top: 10px;
  transition: all 0.3s ease;

  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(118, 138, 200, 0.3);
  }

  &:active {
    transform: translateY(0);
  }
}

.login-box-and-hint {
  position: relative;
  display: inline-block;
}

</style>

