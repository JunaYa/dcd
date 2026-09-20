# Eye Monitor

按参考截图实现的 macOS 护眼提醒应用，使用 Vue 3 + Tauri 2。

## 功能

- 今日：疲劳值、实际使用时长、当天疲劳曲线、手动休息和暂停提醒。
- 分析：按日期范围、天/周/月汇总使用时长、疲劳峰值时长和休息次数，导出 CSV。
- 规则：工作时长、休息时长、重复提醒间隔、提前通知、结束声音和观影模式。
- 设置：开机启动、状态栏和程序坞图标、桌面浮层/窗口提醒、提醒文案、跳过按钮及背景图片。
- 状态栏：使用摘要、今日曲线、休息、暂停、打开主窗口、设置、复制分享摘要和退出。

桌面应用每秒累计使用时间，每分钟记录一个曲线点，每 30 秒及休息状态切换时保存至应用数据目录下的 `eye-monitor.json`。正常退出时也会保存。休息完成后疲劳值归零；跳过休息不清除疲劳。macOS 无输入超过 60 秒时按空闲处理，观影模式会继续累计；系统睡眠的时间不会算作使用时间。疲劳值是按工作/休息规则计算的计时指标。

首次启动使用真实的空记录。“预览示例数据”仅切换图表展示，不覆盖真实记录。浏览器预览不执行系统计时或修改系统启动项，浏览器设置与桌面设置分开保存。

## 运行与验证

```sh
pnpm install
pnpm tauri dev
pnpm build
node --experimental-strip-types --test tests/eye-model.test.mjs
CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_INCREMENTAL=0 cargo test --manifest-path src-tauri/Cargo.toml --lib eye_model
CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 pnpm tauri build --debug --bundles app
```

开发端口为 `1422`。调试包位于 `src-tauri/target/debug/bundle/macos/Eye Monitor.app`。打包应用运行不依赖开发服务器。当前验证平台为 macOS；其他系统的空闲检测尚未实现。

桌面浮层按当前主窗口所在显示器的尺寸覆盖桌面，不进入 macOS 全屏空间。默认背景为半透明模糊遮罩，结束或跳过休息后撤掉浮层，保留底层窗口状态。旧设置中的 `fullscreen` 值继续兼容，表示桌面浮层。
