# Changelog

All notable changes to this project will be documented in this file.

## [0.2.1] - 2026-05-16

### Fixed
- 更新 README 和文档中的版本号

## [0.2.0] - 2026-05-16

### Added
- 支持 SVG 和 HTML 作为自定义图标（使用 inner_html）
- 添加全局默认配置功能
  - `set_default_icons()` - 设置默认图标
  - `set_default_styles()` - 设置默认样式
  - `set_defaults()` - 同时设置图标和样式
- 新增 `DefaultIcons` 结构体用于配置四种通知类型的默认图标
- 新增 `DefaultStyles` 结构体用于配置四种通知类型的默认样式
- 演示应用添加 "Set Custom Defaults" 按钮

### Changed
- 简化 API：将 `custom_bg_color`、`custom_bg_image` 和 `custom_styles` 合并为单个 `custom_styles` 字符串参数
- `add_notification_custom()` 函数参数从 7 个减少到 5 个
- 使用 `inner_html` 渲染图标，支持任意 HTML 内容（Emoji、SVG、Font Awesome 等）
- 样式优先级：自定义样式 > 全局默认样式 > 基础样式
- 图标优先级：自定义图标 > 全局默认图标 > 硬编码默认图标

### Removed
- 移除 `NotificationStyles` 结构体（改用 CSS 字符串）

### Fixed
- 修复 Leptos view 宏中 style 属性语法问题
- 优化样式合并逻辑

### Documentation
- 更新 README 添加新功能使用说明
- 添加 SVG 图标示例
- 添加全局配置示例

## [0.1.0] - 2026-05-15

### Added
- 初始版本发布
- 四种通知类型：Info、Success、Warning、Error
- 流畅的进入/退出动画
- 可视化进度条
- 支持自定义背景颜色和图片
- 支持自定义图标
- 支持完整 CSS 样式自定义
