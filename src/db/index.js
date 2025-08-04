import Database from "@tauri-apps/plugin-sql";


// 连接数据库
async function getDb() {
    return await Database.load("sqlite:tolbox.db");
}


export async function getSetting(key) {
    const db = await getDb();
    const rows = await db.select(
        "SELECT value FROM settings WHERE key=?",
        [key]
    );
    if (rows.length > 0) {
        return rows[0].value;
    }else {
        return null
    }
}
// 获取设置（如果没有则插入默认值）
export async function getOrInitSetting(key, defaultValue) {
    const db = await getDb();
    const rows = await db.select(
        "SELECT value FROM settings WHERE key=?",
        [key]
    );

    if (rows.length > 0) {
        return rows[0].value;
    } else {
        await db.execute(
            "INSERT INTO settings (key, value) VALUES (?, ?)",
            [key, defaultValue]
        );
        return defaultValue;
    }

}

// 更新设置
export async function setSetting(key, value) {
    const db = await getDb();
    await db.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)",
        [key, value]
    );
}
