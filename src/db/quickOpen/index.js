import  Database  from '@tauri-apps/plugin-sql'

// 初始化数据库
async function initDb() {
    const db = await Database.load('sqlite:quickopen.db')

    // 创建应用表
    await db.execute(`
    CREATE TABLE IF NOT EXISTS pinned_apps (
      id INTEGER PRIMARY KEY autoincrement,
      name TEXT NOT NULL,
      path TEXT NOT NULL
    )
  `)

    // 创建文件表
    await db.execute(`
    CREATE TABLE IF NOT EXISTS pinned_files (
      id INTEGER PRIMARY KEY autoincrement,
      path TEXT NOT NULL,
      is_dir BOOLEAN NOT NULL
    )
  `)

    return db
}

// 获取所有已Pin的应用
export async function getPinnedApps() {
    const db = await initDb()
    return await db.select('SELECT * FROM pinned_apps')
}

// 获取所有已Pin的文件
export async function getPinnedFiles() {
    const db = await initDb()
    return await db.select('SELECT * FROM pinned_files')
}

// 添加或更新Pin的应用
export async function upsertPinnedApp(app) {
    const db = await initDb()
    await db.execute(
        'INSERT OR REPLACE INTO pinned_apps (name, path) VALUES (?, ?)',
        [app.name, app.path]
    )
}

// 添加或更新Pin的文件
export async function upsertPinnedFile(file) {
    const db = await initDb()
    await db.execute(
        'INSERT OR REPLACE INTO pinned_files (path, is_dir) VALUES (?, ?)',
        [file.path, file.isDirectory]
    )
}

// 删除Pin的应用
export async function deletePinnedApp(app) {
    const db = await initDb()
    await db.execute('DELETE FROM pinned_apps WHERE id = ?', [app.id])
}

// 删除Pin的文件
export async function deletePinnedFile(file) {
    const db = await initDb()
    await db.execute('DELETE FROM pinned_files WHERE id = ?', [file.id])
}


