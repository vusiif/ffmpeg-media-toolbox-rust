# Rust FFmpeg Processor — AI Development Plan

> 本文档是 Rust 版本项目的长期开发约束与架构规范。
>
> 所有参与本项目开发的 AI，在生成、修改、重构代码之前，都必须阅读并遵守本文档。
>
> 本项目不是 C++ / Qt 版本的翻译版。
>
> Rust 版本必须坚持自己的产品定位：
>
> **轻量、快速、批处理优先、CLI-first、自动化友好。**

---

# 1. 项目定位

本项目是一个：

**使用 Rust + egui + FFmpeg 构建的轻量级跨平台多媒体处理工具。**

核心目标：

```text
Fast
Simple
Batch-first
CLI-first
Reliable
Scriptable
Low-overhead
```

主要解决：

```text
格式转换
图片处理
音频转换
视频基础处理
批量任务
目录任务
任务队列
预设
自动化
FFmpeg 命令生成
```

本项目的核心体验应该是：

```text
Drop Files
    ↓
Choose Action
    ↓
Choose Preset
    ↓
Run
```

而不是：

```text
打开复杂编辑器
↓
调整大量面板
↓
实时预览
↓
时间轴编辑
```

---

# 2. 与 C++ / Qt 版本的定位区别

C++ / Qt 版本：

```text
Media Workstation
```

目标：

```text
功能完整
UI 精致
复杂预览
深度桌面集成
高级参数
未来可能集成 libav
```

Rust 版本：

```text
Media Processor
```

目标：

```text
轻量
快速启动
批处理
任务调度
CLI
Preset
自动化
高可靠性
较少复杂 UI
```

不要为了“功能一致”而强行复制 Qt 版本。

---

# 3. Rust 版本产品原则

Rust 版本长期遵循以下原则：

1. CLI 和 GUI 使用同一个 Core。
2. GUI 只是 Core 的一个 Frontend。
3. GUI 不包含 FFmpeg 业务逻辑。
4. GUI 不直接启动 FFmpeg。
5. GUI 不直接生成 FFmpeg 参数。
6. GUI 不承担复杂视频编辑职责。
7. 批处理是一等公民。
8. Preset 是一等公民。
9. 自动化是一等公民。
10. Headless 使用场景是一等公民。

---

# 4. 技术栈

默认技术栈：

```text
Language
Rust stable

GUI
egui
eframe

Async
Tokio

Serialization
serde
serde_json

CLI
clap

Error
thiserror

Logging
tracing
tracing-subscriber

FFmpeg
ffmpeg CLI
ffprobe CLI

Build
Cargo
```

可按需求使用：

```text
uuid
directories
rfd
egui_extras
image
```

任何新 dependency 都必须有明确理由。

---

# 5. 禁止默认引入的技术

除非用户明确要求，否则禁止引入：

```text
Qt
Slint
iced
Tauri
Electron
WebView
React
Vue
Node.js
Python Runtime
C++ Runtime Dependency
libavcodec binding
libavformat binding
libavfilter binding
GStreamer
libmpv
```

尤其禁止为了一个很小的功能引入大型 Framework。

---

# 6. 为什么选择 egui

Rust 版本使用：

```text
egui + eframe
```

原因：

```text
纯 Rust
开发速度快
架构简单
适合工具型软件
适合高信息密度 UI
适合实时更新任务状态
非常适合 Job Queue
非常适合参数面板
非常适合 Debug / Log / Command UI
```

本项目不追求：

```text
原生系统 Widget 外观
复杂动画
复杂 Designer
超大型 retained-mode GUI 架构
```

因此 egui 的 immediate-mode 模型符合本项目定位。

---

# 7. GUI 不得成为核心

项目核心必须能够：

```text
完全不启动 GUI
```

仍然执行媒体处理。

例如：

```bash
media-rs convert video.mov --preset h265-small
```

以及：

```bash
media-rs image *.png --resize 1920 --format webp
```

必须可以直接工作。

如果某个核心功能：

```text
只能通过 GUI 使用
```

说明架构可能设计错误。

---

# 8. Cargo Workspace

项目建议使用：

```text
Cargo Workspace
```

推荐结构：

```text
media-rs/
│
├── Cargo.toml
│
├── crates/
│   │
│   ├── media-core/
│   │
│   ├── media-cli/
│   │
│   └── media-gui/
│
├── presets/
│
├── assets/
│
├── tests/
│
└── docs/
```

---

# 9. media-core

`media-core` 是整个项目最重要的 crate。

负责：

```text
FFmpeg
FFprobe
Media Model
Pipeline
Job
Job Queue
Preset
Scheduler
Output
Errors
```

绝对禁止：

```text
media-core
    ↓
depends on egui
```

Core 必须完全不知道 GUI 的存在。

---

# 10. media-cli

负责：

```text
Command Line Interface
```

依赖：

```text
media-core
clap
```

不包含媒体处理实现。

CLI 只是：

```text
parse arguments
    ↓
create core request
    ↓
run core
    ↓
print result
```

---

# 11. media-gui

负责：

```text
egui / eframe frontend
```

依赖：

```text
media-core
eframe
egui
```

GUI 只负责：

```text
Input
View
Interaction
Presentation
```

禁止在 GUI crate 重复实现 Core 逻辑。

---

# 12. 推荐 Core 目录

```text
media-core/src/
│
├── lib.rs
│
├── error.rs
│
├── config.rs
│
├── ffmpeg/
│   ├── mod.rs
│   ├── locator.rs
│   ├── capabilities.rs
│   ├── command.rs
│   ├── process.rs
│   ├── progress.rs
│   └── probe.rs
│
├── media/
│   ├── mod.rs
│   ├── file.rs
│   ├── stream.rs
│   └── metadata.rs
│
├── pipeline/
│   ├── mod.rs
│   ├── operation.rs
│   ├── compiler.rs
│   └── filtergraph.rs
│
├── image/
│   ├── mod.rs
│   ├── crop.rs
│   ├── resize.rs
│   ├── rotate.rs
│   ├── flip.rs
│   └── join.rs
│
├── jobs/
│   ├── mod.rs
│   ├── job.rs
│   ├── queue.rs
│   ├── scheduler.rs
│   └── history.rs
│
├── preset/
│   ├── mod.rs
│   ├── preset.rs
│   ├── loader.rs
│   └── builtin.rs
│
└── output/
    ├── mod.rs
    ├── naming.rs
    └── path.rs
```

---

# 13. GUI 目录

```text
media-gui/src/
│
├── main.rs
├── app.rs
├── state.rs
│
├── pages/
│   ├── mod.rs
│   ├── home.rs
│   ├── convert.rs
│   ├── image.rs
│   ├── batch.rs
│   ├── queue.rs
│   ├── presets.rs
│   └── settings.rs
│
├── widgets/
│   ├── mod.rs
│   ├── file_drop.rs
│   ├── job_row.rs
│   ├── media_info.rs
│   ├── preset_picker.rs
│   └── output_picker.rs
│
└── theme/
    ├── mod.rs
    └── style.rs
```

---

# 14. CLI 目录

```text
media-cli/src/
│
├── main.rs
│
├── args.rs
│
└── commands/
    ├── mod.rs
    ├── convert.rs
    ├── image.rs
    ├── probe.rs
    ├── batch.rs
    └── preset.rs
```

---

# 15. Rust 数据模型原则

必须充分利用 Rust 类型系统。

禁止大量使用：

```rust
String
bool
Option<String>
```

来表示复杂状态。

例如错误：

```rust
struct VideoSettings {
    codec: String,
    quality: String,
}
```

推荐：

```rust
enum VideoCodec {
    Copy,
    H264,
    H265,
    AV1,
    VP9,
}
```

---

# 16. 使用 enum 表示互斥状态

例如：

```rust
enum QualityMode {
    Crf(u8),
    Bitrate(u64),
    Lossless,
}
```

而不是：

```rust
struct Quality {
    crf: Option<u8>,
    bitrate: Option<u64>,
    lossless: bool,
}
```

尽量做到：

```text
invalid state cannot be represented
```

---

# 17. 错误处理

核心错误统一使用：

```rust
thiserror
```

例如：

```rust
#[derive(Debug, thiserror::Error)]
pub enum MediaError {
    #[error("FFmpeg executable was not found")]
    FfmpegNotFound,

    #[error("FFprobe executable was not found")]
    FfprobeNotFound,

    #[error("Input file does not exist: {0}")]
    InputNotFound(PathBuf),

    #[error("Unsupported encoder: {0}")]
    UnsupportedEncoder(String),

    #[error("FFmpeg process failed")]
    ProcessFailed,
}
```

禁止核心业务：

```rust
unwrap()
expect()
panic!()
```

除非：

```text
程序内部绝对不变量
测试
启动阶段不可恢复错误
```

---

# 18. anyhow 使用限制

CLI 最外层可以考虑：

```text
anyhow
```

但 Core API 不应全部返回：

```rust
anyhow::Result<T>
```

Core 应使用：

```text
明确、结构化的 Error Enum
```

方便：

```text
GUI
CLI
未来其他 frontend
```

分别处理。

---

# 19. Path 处理

文件路径统一使用：

```rust
Path
PathBuf
```

禁止内部核心长期使用：

```rust
String
```

保存文件路径。

必须考虑：

```text
空格
Unicode
中文
日文
Emoji
特殊字符
```

---

# 20. 禁止 Shell String

禁止：

```rust
Command::new("cmd")
    .arg("/C")
    .arg(format!("ffmpeg -i {} ...", path));
```

正确：

```rust
Command::new(ffmpeg)
    .arg("-i")
    .arg(input)
    .arg("-c:v")
    .arg("libx265")
    .arg(output);
```

原因：

```text
路径安全
Unicode
空格
Shell Injection
跨平台
```

---

# 21. FFmpeg Locator

必须存在统一：

```rust
FfmpegLocator
```

负责：

```text
检测 ffmpeg
检测 ffprobe
自定义路径
PATH 查找
版本获取
Executable 验证
```

禁止：

```text
不同模块分别寻找 FFmpeg
```

---

# 22. FFmpeg 能力扫描

必须支持：

```bash
ffmpeg -version
ffmpeg -formats
ffmpeg -codecs
ffmpeg -encoders
ffmpeg -decoders
ffmpeg -filters
ffmpeg -hwaccels
```

结构：

```rust
pub struct FfmpegCapabilities {
    pub version: String,
    pub formats: HashSet<String>,
    pub codecs: HashSet<String>,
    pub encoders: HashSet<String>,
    pub decoders: HashSet<String>,
    pub filters: HashSet<String>,
    pub hardware_accelerators: HashSet<String>,
}
```

不能假定：

```text
NVENC 存在
AV1 存在
AVIF 存在
libx265 存在
```

必须实际检测。

---

# 23. FFprobe

所有媒体探测统一使用：

```text
ffprobe
```

推荐：

```bash
ffprobe \
    -v error \
    -of json \
    -show_format \
    -show_streams \
    -show_chapters \
    input
```

Rust 使用：

```text
serde
serde_json
```

进行反序列化。

---

# 24. FFprobe Raw DTO 与 Domain Model 分离

推荐：

```text
FFprobe JSON
    ↓
Raw DTO
    ↓
Parser / Converter
    ↓
MediaFile
```

禁止 Domain Model 与 FFprobe 原始 JSON 强绑定。

---

# 25. MediaFile

推荐：

```rust
pub struct MediaFile {
    pub path: PathBuf,
    pub file_size: u64,
    pub duration: Option<Duration>,
    pub video_streams: Vec<VideoStream>,
    pub audio_streams: Vec<AudioStream>,
    pub subtitle_streams: Vec<SubtitleStream>,
    pub metadata: Metadata,
}
```

---

# 26. 不要把 FFmpeg 语法泄漏到整个项目

例如业务层应该：

```rust
ResizeOperation {
    width: 1920,
    height: 1080,
}
```

而不是：

```rust
String::from("scale=1920:1080")
```

FFmpeg syntax 只能集中出现在：

```text
Command Compiler
FilterGraph Compiler
```

---

# 27. Media Pipeline

处理统一表示为：

```text
Input
 ↓
Operation
 ↓
Operation
 ↓
Operation
 ↓
Output
```

例如：

```text
Crop
 ↓
Resize
 ↓
Rotate
 ↓
Watermark
```

---

# 28. Operation

推荐：

```rust
pub enum MediaOperation {
    Crop(CropOperation),
    Resize(ResizeOperation),
    Rotate(RotateOperation),
    Flip(FlipOperation),
}
```

未来：

```rust
Trim
Watermark
Subtitle
FrameRate
AudioNormalize
```

都可以加入。

---

# 29. Image Pipeline

第一阶段：

```rust
pub enum ImageOperation {
    Crop(CropOperation),
    Resize(ResizeOperation),
    Rotate(RotateOperation),
    Flip(FlipOperation),
}
```

必须保持：

```text
non-destructive
```

即：

```text
只存 Operation
不修改原文件
```

---

# 30. Pipeline Compiler

必须存在：

```text
PipelineCompiler
```

负责：

```text
MediaPipeline
    ↓
FFmpeg FilterGraph
```

只有 Compiler 能生成：

```text
crop=
scale=
transpose=
hflip
vflip
```

业务层不得直接拼接 filter。

---

# 31. FFmpeg Command

内部表示：

```rust
pub struct FfmpegCommand {
    pub program: PathBuf,
    pub args: Vec<OsString>,
}
```

用于：

```text
实际执行
Command Preview
Logging
CLI
```

---

# 32. CommandBuilder

统一：

```rust
pub trait CommandBuilder {
    fn build(&self) -> Result<FfmpegCommand, MediaError>;
}
```

所有 FFmpeg 任务必须经过：

```text
CommandBuilder
```

禁止：

```text
Convert Page 自己拼参数
Image Page 自己拼参数
CLI 自己拼参数
```

---

# 33. FFmpeg Process

必须统一封装：

```text
FfmpegProcess
```

负责：

```text
spawn
stdout
stderr
progress
exit status
cancel
kill
```

不要每个 Job 自己实现一遍 Tokio Process。

---

# 34. Progress

使用：

```bash
-progress pipe:1
-nostats
```

解析：

```text
key=value
```

不要依赖 human-readable stderr。

模型：

```rust
pub struct FfmpegProgress {
    pub frame: Option<u64>,
    pub out_time: Option<Duration>,
    pub fps: Option<f64>,
    pub speed: Option<f64>,
    pub total_size: Option<u64>,
}
```

---

# 35. Async Runtime

FFmpeg 进程和 Job Scheduler 可以使用：

```text
Tokio
```

GUI 主线程不得：

```text
block_on 长任务
wait FFmpeg
同步读取大量输出
```

egui 主线程只处理：

```text
UI
State snapshot
User action
```

---

# 36. GUI 与后台通信

推荐：

```text
GUI
 ↓ command
Channel
 ↓
Background Runtime
 ↓
Core
 ↓ events
Channel
 ↓
GUI State
```

可以使用：

```text
tokio::sync::mpsc
```

或经过论证的其他 Channel。

禁止：

```text
GUI update()
里面直接等待 FFmpeg
```

---

# 37. egui Repaint

后台有新事件时：

```text
request_repaint
```

不要为了任务进度：

```text
永远 144 FPS 重绘整个 GUI
```

没有 UI 变化时尽量减少 repaint。

---

# 38. Job

所有媒体处理都必须变成：

```text
Job
```

推荐：

```rust
pub struct Job {
    pub id: JobId,
    pub request: JobRequest,
    pub status: JobStatus,
    pub progress: Option<JobProgress>,
}
```

---

# 39. JobStatus

推荐：

```rust
pub enum JobStatus {
    Pending,
    Preparing,
    Running,
    Completed,
    Failed(JobError),
    Cancelled,
}
```

不要使用：

```rust
running: bool,
failed: bool,
completed: bool,
```

这种容易出现非法组合的设计。

---

# 40. JobRequest

推荐：

```rust
pub enum JobRequest {
    Convert(ConvertRequest),
    Image(ImageRequest),
    Probe(ProbeRequest),
}
```

未来：

```text
Trim
Join
ExtractAudio
Gif
```

加入新的 Variant。

---

# 41. Job Queue

统一：

```text
JobQueue
```

负责：

```text
enqueue
cancel
retry
remove
reorder
clear completed
```

---

# 42. Scheduler

Rust 版本的重要特色之一：

```text
Scheduler
```

Scheduler 与 Queue 分开。

Queue：

```text
保存任务
管理状态
```

Scheduler：

```text
决定什么时候运行
运行几个
资源分配
```

---

# 43. Workload 分类

推荐：

```rust
pub enum Workload {
    Image,
    Audio,
    VideoCpu,
    VideoGpu,
    Probe,
}
```

默认并发策略：

```text
Image      4
Audio      2
Video CPU  1
Video GPU  1~2
Probe      4
```

后期可以调整。

---

# 44. Automatic Concurrency

Rust 版本可以把：

```text
Automatic Scheduler
```

作为特色。

第一版不用复杂资源预测。

简单规则即可：

```text
CPU Video
默认只跑一个

Image
允许多个

Probe
允许多个
```

用户可以设置：

```text
Automatic
1
2
4
8
```

---

# 45. FFmpeg 内部线程

默认：

```text
FFmpeg Threads = Automatic
```

不要自动强制：

```bash
-threads 16
```

除非用户在 Advanced Settings 中明确设置。

Job 并发与 FFmpeg 内部 threading 是两个独立概念。

---

# 46. Rust 版本核心特色：Batch

Batch 是 P0。

支持：

```text
Multiple Files
Directory
Recursive Directory
```

例如：

```bash
media-rs convert ./videos \
    --recursive \
    --preset h265-small
```

---

# 47. Directory Processing

Rust 版应比 Qt 版更重视：

```text
Directory Workflow
```

例如：

```text
Input Directory
Output Directory
Recursive
Pattern
```

支持：

```text
*.png
*.jpg
*.mov
```

---

# 48. 批处理过滤

未来支持：

```text
include
exclude
```

例如：

```bash
--include "*.mov"
--exclude "*proxy*"
```

第一版无需实现复杂 Glob DSL。

---

# 49. Naming Template

统一：

```text
OutputNameTemplate
```

支持：

```text
{name}
{ext}
{index}
{width}
{height}
```

以后：

```text
{codec}
{preset}
{date}
```

CLI 与 GUI 必须共享。

---

# 50. 输出策略

支持：

```text
Same Directory
Custom Directory
Mirror Directory Tree
```

其中：

```text
Mirror Directory Tree
```

是 Rust Batch 版本很值得做的能力。

例如：

```text
input/
├── a/
│   └── 1.png
└── b/
    └── 2.png
```

输出：

```text
output/
├── a/
│   └── 1.webp
└── b/
    └── 2.webp
```

---

# 51. 文件冲突策略

统一：

```rust
pub enum ConflictPolicy {
    Ask,
    Replace,
    Rename,
    Skip,
}
```

CLI 模式不能依赖 Ask。

CLI 默认建议：

```text
Skip
```

或要求用户显式指定策略。

---

# 52. Preset

Preset 是 Rust 版本的一等公民。

Preset 必须是：

```text
结构化数据
```

不是：

```text
一个 FFmpeg command string
```

---

# 53. Preset 格式

建议使用：

```text
JSON
```

第一阶段即可。

例如：

```json
{
  "name": "H265 Small",
  "type": "video",
  "video": {
    "codec": "h265",
    "quality": {
      "mode": "crf",
      "value": 26
    }
  }
}
```

如果以后认为 TOML 更适合用户手写，可以再考虑。

---

# 54. Preset 可移植性

Preset 必须：

```text
GUI 可用
CLI 可用
可导出
可导入
```

例如：

```bash
media-rs convert *.mov --preset my-preset.json
```

---

# 55. Built-in Presets

提供：

```text
H264 Compatible
H265 High Quality
H265 Small
AV1 Archive

MP3 320k
AAC 256k
Opus 160k
FLAC

WebP High Quality
WebP Small
JPEG High Quality
PNG Lossless
```

实际显示需要考虑 FFmpeg capabilities。

---

# 56. CLI-first

Rust 版核心功能必须设计 CLI。

推荐：

```bash
media-rs probe file.mp4

media-rs convert input.mov \
    --format mp4 \
    --codec h265

media-rs convert *.mov \
    --preset h265-small

media-rs image *.png \
    --resize 1920x1080 \
    --format webp

media-rs batch ./input \
    --recursive \
    --preset webp-small \
    --output ./output
```

---

# 57. CLI 输出

普通模式：

```text
Processing 12 files
[3/12] photo03.png -> photo03.webp
```

支持：

```text
--quiet
```

以及未来：

```text
--json
```

---

# 58. Machine-readable CLI

P1 可以增加：

```bash
media-rs ... --json
```

输出：

```json
{
  "status": "completed",
  "jobs": [...]
}
```

方便：

```text
脚本
自动化
其他程序
```

调用。

---

# 59. Exit Code

CLI 必须定义稳定 Exit Code。

例如：

```text
0 success
1 generic error
2 invalid arguments
3 ffmpeg unavailable
4 job failed
5 partial batch failure
```

具体编号可调整，但一旦发布后不要随意变化。

---

# 60. Rust 版第一阶段 GUI

GUI 只做几个主要页面：

```text
Quick Convert
Batch
Image
Queue
Presets
Settings
```

不要做复杂 Sidebar 嵌套系统。

---

# 61. Quick Convert

核心目标：

```text
拖文件
↓
选择 Preset
↓
Run
```

高级设置折叠。

默认界面应非常简单。

---

# 62. Batch Page

Batch Page 是核心页面。

需要：

```text
Input Files / Directory
Recursive
Preset
Output Directory
Naming
Conflict Policy
Run
```

---

# 63. Queue Page

Rust GUI 的 Queue 页面可以保持高信息密度。

例如：

```text
STATUS   FILE                  PROGRESS    SPEED
RUNNING  video01.mov           74%         2.1x
WAITING  video02.mov
DONE     image01.png           100%
FAILED   video03.mkv
```

---

# 64. Queue 操作

支持：

```text
Pause Scheduling
Resume Scheduling

Cancel
Retry
Remove

Clear Completed
Retry Failed
```

注意：

```text
Pause Scheduling
```

不是暂停已经运行的 FFmpeg。

第一版不要实现复杂 FFmpeg Process suspend/resume。

---

# 65. Image Tools

P0：

```text
Convert
Crop
Resize
Rotate
Flip
Join
```

但是 Rust 版本不需要做特别复杂的 Canvas Editor。

---

# 66. Image Crop

Rust GUI 第一版可以采用：

```text
数字输入
+
简单可视 Crop Rect
```

支持：

```text
Free
1:1
4:3
3:2
16:9
9:16
```

不要为了 Crop 立即建立复杂图像编辑器 Framework。

---

# 67. Resize

支持：

```text
Exact
Fit
Fill
Percentage
```

以及：

```text
Keep Aspect Ratio
Prevent Upscaling
```

---

# 68. Rotate / Flip

支持：

```text
90 CW
90 CCW
180

Horizontal Flip
Vertical Flip
```

---

# 69. Image Join

支持：

```text
Horizontal
Vertical
Grid
```

设置：

```text
Spacing
Margin
Background
Alignment
Cell Size
```

---

# 70. 图片 Preview

Preview 应保持轻量。

优先：

```text
缩略图
低分辨率 Preview
```

禁止默认加载巨大图片的完整像素到 GUI。

应考虑：

```text
Preview Cache
Thumbnail
Texture Memory
```

---

# 71. Video Preview

Rust 版本第一阶段：

```text
不做完整视频播放器
```

这是与 Qt 版本的重要区别。

允许：

```text
显示媒体信息
显示封面/截图
显示缩略图
```

而不是：

```text
完整 Playback Engine
Audio Sync
Timeline Seeking
```

---

# 72. Video Thumbnail

可以通过 FFmpeg：

```text
生成临时缩略图
```

供 GUI 显示。

这样避免第一阶段引入：

```text
GStreamer
libmpv
libav
```

---

# 73. 视频功能

P1：

```text
Trim
Join
Extract Audio
Mute
Screenshot
GIF
Resize
FPS
```

都以：

```text
utility
```

形式存在。

不要变成 Video Editor。

---

# 74. Fast Trim / Accurate Trim

支持：

```text
Fast
Accurate
```

Fast：

```text
尽量 stream copy
```

Accurate：

```text
允许重新编码
```

---

# 75. Smart Remux

格式转换支持：

```text
Smart
Copy
Encode
```

Smart 判断：

```text
容器是否兼容
Codec 是否兼容
是否有 Filter
是否要求改变参数
```

如果无需重编码：

```text
stream copy
```

---

# 76. Command Preview

GUI 和 CLI 都应该支持查看最终 FFmpeg 命令。

GUI：

```text
Copy Command
```

CLI：

```bash
--dry-run
```

例如：

```bash
media-rs convert a.mov --preset h265 --dry-run
```

输出：

```text
ffmpeg ...
```

但实际不执行。

---

# 77. Dry Run

`--dry-run` 应当是 Rust 版本的重要特色。

所有 Job 都应该尽量支持。

用途：

```text
Debug
Automation
Script verification
Preset development
```

---

# 78. Logging

使用：

```text
tracing
```

日志分级：

```text
TRACE
DEBUG
INFO
WARN
ERROR
```

---

# 79. Job Log

每个 Job 保存：

```text
command
start time
end time
exit status
stderr
error
```

不要默认无限保存 FFmpeg 巨量 stderr。

需要：

```text
合理日志长度限制
或写入日志文件
```

---

# 80. GUI Log

GUI 默认只显示：

```text
重要事件
错误
任务结果
```

高级页面可以：

```text
Show FFmpeg Log
```

---

# 81. CLI Logging

支持：

```text
-v
-vv
-vvv
```

或类似 verbosity 机制。

默认不要输出大量 Debug 信息。

---

# 82. Settings

Rust GUI 设置：

```text
General

Default Output Directory
Conflict Policy

FFmpeg

FFmpeg Path
FFprobe Path
Version
Rescan Capabilities

Processing

Concurrent Jobs
Scheduler Mode

GUI

Theme
Scale

Advanced

FFmpeg Log Level
Default Extra Arguments
```

---

# 83. 配置文件

建议使用：

```text
serde
```

保存 AppConfig。

使用平台标准配置目录。

不要把：

```text
config.json
```

默认扔在 executable 同目录。

---

# 84. 可携带模式

P1 可以考虑：

```text
Portable Mode
```

例如：

```text
media-rs.toml
```

存在 executable 附近时启用 portable settings。

不是 v0.1 必需。

---

# 85. Theme

Rust 版本 UI 应：

```text
简洁
高信息密度
少动画
少装饰
```

可以支持：

```text
System
Dark
Light
```

不要花大量开发时间复制 Qt 版视觉效果。

---

# 86. UI 风格目标

Rust GUI 的视觉方向：

```text
Developer Tool
+
Modern Utility
```

而不是：

```text
Creative Editing Suite
```

类似体验目标：

```text
快
清楚
紧凑
能看到重要数据
```

---

# 87. Keyboard-first

Rust 版本应该比 Qt 版更重视快捷键。

例如：

```text
Ctrl+O   Add Files
Ctrl+Shift+O Add Directory

Ctrl+Enter Start

Ctrl+P   Presets
Ctrl+L   Logs

Delete   Remove Job
R        Retry
```

具体快捷键发布前再确定。

---

# 88. Drag & Drop

GUI 必须支持：

```text
单文件
多个文件
目录
```

自动判断：

```text
Video
Audio
Image
Unknown
```

---

# 89. Headless 思维

新增功能之前 AI 必须问：

```text
这个功能能不能在没有 GUI 的情况下工作？
```

如果不能：

```text
是否真的属于 core？
```

---

# 90. Automation

P1 后重点发展：

```text
Preset
CLI
Directory
Dry Run
JSON Output
```

P2 可考虑：

```text
Watch Folder
```

---

# 91. Watch Folder

未来特色：

```text
监控某个目录
↓
出现新文件
↓
按照 Preset 自动处理
```

例如：

```text
screenshots/
↓
自动转 WebP
```

或者：

```text
camera/
↓
自动生成 Proxy
```

第一版不实现。

---

# 92. Watch Folder 的安全规则

未来实现时必须：

```text
避免处理未写完文件
避免重复处理
记录 processed state
提供失败隔离
```

不要简单看到文件就立即启动 FFmpeg。

---

# 93. Pipeline 文件

未来可以支持：

```text
保存 Processing Pipeline
```

例如：

```json
{
  "operations": [
    {
      "type": "resize",
      "width": 1920
    },
    {
      "type": "format",
      "format": "webp"
    }
  ]
}
```

CLI：

```bash
media-rs run pipeline.json ./images
```

这是 Rust 版本非常值得发展的方向。

---

# 94. 不做复杂项目文件

Rust 版暂时不要引入：

```text
Project
Timeline Project
Media Bin Project
```

Preset 和 Pipeline File 足够。

---

# 95. 性能原则

Rust 版本追求：

```text
Low overhead
```

但不要做无意义微优化。

优先：

```text
正确
稳定
不卡 UI
内存合理
```

---

# 96. 不把视频读进内存

视频操作始终：

```text
Process / Stream based
```

禁止：

```text
read entire video into Vec<u8>
```

---

# 97. 大型 Batch

设计时考虑：

```text
10 files
100 files
10000 files
```

JobQueue 不应该因为：

```text
上万个 Job
```

就不可用。

GUI 可以使用：

```text
分页
过滤
只绘制可见区域
```

利用 egui 的虚拟滚动能力。

---

# 98. Progress Event 节流

FFmpeg 可能频繁发送状态。

不要每一个状态立即：

```text
写磁盘
序列化
全 GUI 刷新
```

可以适当节流：

```text
100~250 ms
```

具体根据体验调整。

---

# 99. Job History

P1。

保存：

```text
Completed
Failed
Cancelled
```

但不要保存无限历史。

例如：

```text
最近 500
```

或配置限制。

---

# 100. Retry

Retry 必须创建：

```text
新的执行 attempt
```

原始 Job 配置不变。

不要因为失败修改原始任务参数。

---

# 101. Cancellation

取消任务：

```text
优先 graceful termination
```

必要时：

```text
kill
```

取消后：

```text
正确更新状态
清理临时文件
按策略处理半成品
```

---

# 102. 原文件不可破坏

默认所有任务：

```text
Input is immutable
```

输出为新文件。

如果用户明确：

```text
Replace Original
```

也应：

```text
先生成 temporary output
↓
验证成功
↓
原子或安全替换
```

不能直接让 FFmpeg 覆盖唯一原文件。

---

# 103. Temp

必须统一：

```text
TempManager
```

负责：

```text
Thumbnail
Intermediate
Temporary output
Concat lists
```

退出后尽可能清理。

---

# 104. 平台支持

目标：

```text
Windows
Linux
macOS
```

尽量保持 Core 平台无关。

平台代码隔离：

```text
platform/
```

---

# 105. Windows

Windows 必须考虑：

```text
UTF-16 paths
长路径
隐藏 console window
```

不要基于 Unix 行为假设 Windows。

---

# 106. Linux

考虑：

```text
Wayland
X11
FFmpeg PATH
不同发行版
```

不要写死 `/usr/bin/ffmpeg`。

---

# 107. macOS

考虑：

```text
App Bundle
Executable location
Sandbox / permission
```

但第一阶段先保证正常桌面运行。

---

# 108. CLI 与 GUI 一致性

以下行为必须共享：

```text
Codec validation
Preset parsing
Naming
Output resolution
Pipeline
Command generation
FFmpeg execution
Error model
```

GUI 和 CLI 不得出现：

```text
同一个 Preset 生成不同 command
```

---

# 109. GUI State 与 Domain State 分离

例如：

```rust
struct ConvertPageState {
    selected_tab: usize,
    advanced_open: bool,
}
```

属于 GUI。

而：

```rust
struct ConvertRequest {
    input: PathBuf,
    output: OutputSpec,
    video: VideoSettings,
}
```

属于 Core。

禁止混在一起。

---

# 110. 不要过度 ECS

除非出现明确需求，否则：

```text
不要为了 egui 引入 ECS
```

这是一个媒体工具，不是游戏引擎。

---

# 111. 不要过度 Actor 化

也不要因为 Tokio 就：

```text
每个对象都是 Actor
```

简单的：

```text
Task
Channel
State
```

足够时，不建立复杂 Actor Framework。

---

# 112. Lock 原则

尽量减少：

```text
Arc<Mutex<巨大 AppState>>
```

特别是：

```text
UI thread
async task
```

共享同一个巨大 Mutex。

优先：

```text
message passing
immutable event
small shared state
```

---

# 113. 不要滥用 Arc

不是所有数据都需要：

```rust
Arc<T>
```

明确 ownership 后再决定。

---

# 114. Core API

Core API 尽量：

```text
typed
small
predictable
testable
```

避免：

```rust
fn execute(options: HashMap<String, String>)
```

---

# 115. FFmpeg Extra Arguments

高级用户未来可以输入：

```text
Extra FFmpeg Arguments
```

但是必须：

```text
明确标记 Advanced
```

并且不要让其轻易破坏 Core 生成参数。

需要定义：

```text
extra arguments 插入位置
```

避免不可预测行为。

---

# 116. v0.1 功能

Rust v0.1：

```text
FFmpeg detection

FFprobe detection

Capability scan

Media probe

Quick Convert

Video Convert

Audio Convert

Image Convert

Smart Remux

Image Resize

Image Crop

Image Rotate

Image Flip

Horizontal Join

Vertical Join

Grid Join

Batch Files

Batch Directory

Recursive Batch

Output Naming

Preset

Job Queue

Automatic Scheduler

Progress

Cancel

Retry

Logs

Command Preview

CLI

Dry Run

Settings
```

---

# 117. v0.2

```text
Video Trim

Video Join

Extract Audio

Mute

Video Screenshot

GIF

Image Sequence -> Video

Video -> Images

Metadata

Hardware Encoder

Recent Jobs

JSON CLI Output

Pipeline Files

Preset Import / Export
```

---

# 118. v0.3

Rust 版重点不是复制 Qt 高级编辑。

优先：

```text
Watch Folder

Automation Rules

Advanced Batch

Directory Tree Mirroring

Batch Rename

Job History

Resource-aware Scheduler

Pipeline Runner

Headless Workflow

Script Integration
```

---

# 119. Rust 版暂缓功能

除非用户明确要求：

```text
Full Video Player

Timeline

Multi-track

Waveform Editor

Transitions

Keyframe Animation

Real-time Filter Preview

Node Editor

Professional Subtitle Editor

Professional Color Grading

libav Zero-copy Pipeline
```

---

# 120. Rust 版核心竞争力

未来 Rust 版本应该形成：

```text
media-rs
```

既是：

```text
GUI App
```

也是：

```text
CLI Tool
```

也是：

```text
Batch Processor
```

也是：

```text
Automation Engine
```

而不是：

```text
另一个 Qt GUI
```

---

# 121. 推荐使用场景

最终目标：

```bash
media-rs convert *.mov --preset h265-small

media-rs image ./images \
    --recursive \
    --preset webp-small

media-rs run social-media.json *.mp4

media-rs probe video.mp4 --json
```

GUI 只是这些能力的可视化入口。

---

# 122. 测试优先级

Core 必须大量可测试。

优先测试：

```text
FFprobe JSON parser

FFmpeg capability parser

Command Builder

FilterGraph compiler

Naming template

Output path

Preset parser

Job state machine

Scheduler

Smart Remux

Pipeline compiler

CLI arguments
```

GUI 测试优先级低于 Core。

---

# 123. Integration Test

准备：

```text
tests/assets/
```

保存少量：

```text
tiny image
tiny audio
tiny video
```

用于真实 FFmpeg Integration Test。

测试文件必须小。

---

# 124. 测试不依赖用户 FFmpeg 配置

Unit Test：

```text
不调用真实 FFmpeg
```

Integration Test：

```text
显式标记
```

如果 FFmpeg 不存在：

```text
skip
```

而不是失败整个 Unit Test Suite。

---

# 125. Benchmark

早期不要大量 Benchmark。

后期可测：

```text
10000 Job queue

Preset parsing

FFprobe JSON parsing

Large media list GUI
```

不要 Benchmark FFmpeg 编码性能然后归因于 Rust。

真正编码速度主要由 FFmpeg 决定。

---

# 126. AI 开发前检查

AI 开始实现任何功能前必须确认：

```text
[ ] 这是 Core、CLI 还是 GUI？

[ ] 是否可以在无 GUI 情况工作？

[ ] 是否已有对应 Core 模块？

[ ] 是否正在重复 FFmpeg Command Logic？

[ ] 是否应该作为 Job？

[ ] 是否应该作为 Operation？

[ ] 是否可以进入 Pipeline？

[ ] CLI 是否也应该支持？

[ ] Batch 是否应该支持？

[ ] 是否需要 Capability Check？

[ ] 是否阻塞 GUI？

[ ] 是否需要新 Dependency？

[ ] 是否真的需要新 Abstraction？
```

---

# 127. AI 禁止行为

AI 不得无理由：

```text
把项目改成 Slint

把项目改成 Tauri

加入 Qt

加入 C++

加入 Python

加入 Node

引入 Web UI

直接绑定 libav

让 GUI 依赖 FFmpeg syntax

在 GUI 中调用 Command::new(ffmpeg)

在 CLI 中复制 FFmpeg Builder

到处使用 String 表示状态

大量 unwrap()

用一个 Arc<Mutex<App>> 控制所有系统

每个 Job 自己造 Runtime

每个页面自己启动 Tokio Runtime

为简单问题造大型 Framework
```

---

# 128. Dependency 规则

新增 crate 前：

```text
1. std 能不能解决？
2. 已有 dependency 能不能解决？
3. crate 是否维护活跃？
4. 是否跨平台？
5. 是否引入大量传递依赖？
6. 是否真的值得？
```

小功能尽量不用大型 dependency。

---

# 129. Async 规则

不是所有函数都应该：

```rust
async fn
```

只有实际需要 async I/O / Process 等场景使用 async。

纯计算：

```text
parser
builder
validation
naming
```

保持同步。

---

# 130. 生命周期与所有权

不要为了消除 borrow checker 错误：

```text
clone everything
```

先重新考虑 ownership。

但也不要为了避免一次合理 clone：

```text
构造复杂生命周期体系
```

保持实用主义。

---

# 131. 注释

注释解释：

```text
Why
Invariant
FFmpeg quirk
Platform limitation
```

而不是重复代码。

---

# 132. Unsafe

默认：

```text
禁止 unsafe
```

除非：

```text
平台 API
经过严格隔离
无法通过 safe Rust 实现
```

任何 unsafe：

```text
必须说明 Safety Contract
```

第一阶段预计不需要 unsafe。

---

# 133. 代码风格

使用：

```text
cargo fmt
cargo clippy
```

CI 最低检查：

```bash
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test
```

---

# 134. Warning Policy

项目自身代码尽量保持：

```text
zero warnings
```

不要通过：

```rust
#![allow(...)]
```

大面积隐藏问题。

---

# 135. CI

未来推荐：

```text
Windows
Linux
macOS
```

运行：

```text
fmt
clippy
test
build
```

发布流程后期再建立。

---

# 136. Documentation

核心模块：

```text
JobQueue
Scheduler
PipelineCompiler
FfmpegProcess
CommandBuilder
```

必须写清：

```text
职责
输入
输出
线程模型
错误模型
```

---

# 137. API Documentation

公共 Core API 使用：

```rust
/// docs
```

但不要为了内部函数写大量无价值文档。

---

# 138. Release Size

Rust 版本强调轻量。

Release 时关注：

```text
LTO
strip
panic strategy
debug symbols
```

但只有确认需要后调整。

不要第一天沉迷缩小 2 MB executable。

---

# 139. Startup

启动阶段不要：

```text
同步扫描数万个文件
同步跑大量 FFmpeg capability command
同步生成所有 thumbnail
```

可以：

```text
后台检测
缓存 capabilities
```

---

# 140. Capability Cache

FFmpeg capabilities 可以缓存。

Cache Key 至少考虑：

```text
FFmpeg executable path
FFmpeg version
```

如果发生变化：

```text
重新扫描
```

---

# 141. Preset Validation

加载 Preset 时验证：

```text
格式正确
值范围正确
Codec 存在
Encoder 可用
```

如果当前环境不支持：

```text
Preset 保留
但显示 unavailable reason
```

不要静默删除。

---

# 142. User-facing Errors

GUI 不应该只显示：

```text
Process exited with code 1
```

应显示：

```text
H.265 encoder unavailable

The selected FFmpeg build does not provide the requested encoder.

View FFmpeg Log
```

CLI 同时输出：

```text
human readable error
```

并返回正确 Exit Code。

---

# 143. Queue Persistence

v0.1 可以不持久化 Running Queue。

P1 后可以保存：

```text
Pending
Failed
Completed summary
```

程序异常退出后：

```text
Running
```

应转换成：

```text
Interrupted
```

而不是继续认为 Running。

---

# 144. Interrupted 状态

未来可以加入：

```rust
JobStatus::Interrupted
```

用于：

```text
Crash
Force Close
System Restart
```

---

# 145. 核心 UI 结构

推荐：

```text
┌─────────────────────────────────────────┐
│ Media RS                          ⚙     │
├───────────┬─────────────────────────────┤
│ Quick     │                             │
│ Batch     │                             │
│ Image     │        Current Page         │
│ Queue     │                             │
│ Presets   │                             │
│           │                             │
├───────────┴─────────────────────────────┤
│ Running 1 | Waiting 12 | Failed 0       │
└─────────────────────────────────────────┘
```

保持：

```text
simple
dense
fast
```

---

# 146. 不追求复制 Qt UI

AI 禁止以：

```text
“Qt 版这里有”
```

作为 Rust 版必须实现的理由。

应该问：

```text
它符合 Rust 版定位吗？
```

如果不符合：

```text
不做
```

---

# 147. Rust 版本发展路线

```text
v0.1
Fast Media Processor

↓

v0.2
Powerful Batch Utility

↓

v0.3
Automation Engine

↓

v1.x
CLI + GUI Media Workflow Platform
```

---

# 148. Rust 版最重要的架构图

```text
                    ┌─────────────┐
                    │ media-core  │
                    │             │
                    │ FFmpeg      │
                    │ Pipeline    │
                    │ Jobs        │
                    │ Scheduler   │
                    │ Presets     │
                    └──────┬──────┘
                           │
             ┌─────────────┴─────────────┐
             │                           │
             ▼                           ▼
      ┌─────────────┐             ┌─────────────┐
      │  media-cli  │             │  media-gui  │
      │             │             │             │
      │    clap     │             │ egui/eframe │
      └─────────────┘             └─────────────┘
```

---

# 149. FFmpeg 执行架构

```text
JobRequest
    │
    ▼
Validation
    │
    ▼
Pipeline
    │
    ▼
CommandBuilder
    │
    ▼
FfmpegCommand
    │
    ▼
FfmpegProcess
    │
    ▼
ffmpeg
    │
    ├── Progress
    ├── Log
    └── Exit Status
            │
            ▼
         JobQueue
```

---

# 150. GUI 后台架构

```text
egui thread
    │
    │ Command
    ▼
Channel
    │
    ▼
Tokio Runtime
    │
    ▼
media-core
    │
    │ Event
    ▼
Channel
    │
    ▼
GUI State
    │
    ▼
egui repaint
```

---

# 151. 十条最高优先级约束

如果 AI 只能记住十条：

1. Rust 版不是 Qt 版翻译版。

2. Rust 版定位是轻量、Batch-first、CLI-first。

3. `media-core` 永远不能依赖 egui。

4. GUI 和 CLI 必须共享 Core。

5. 所有 FFmpeg 调用必须通过统一 Process / CommandBuilder。

6. FFmpeg Filter 必须由结构化 Pipeline 编译。

7. 所有处理必须成为 Job。

8. Job Queue 与 Scheduler 分离。

9. 默认不做复杂视频 Preview / Timeline。

10. 新功能优先考虑 CLI、Batch 和 Automation 能否使用。

---

# 152. AI 实现功能时的回答要求

AI 输出代码时：

```text
1. 明确文件路径。

2. 给出完整实现。

3. 不用伪代码代替核心逻辑。

4. 新增 dependency 必须说明原因。

5. 修改 Core API 必须说明影响。

6. 新功能必须说明 CLI / GUI / Core 分别修改什么。

7. 如果支持 Batch，应同时考虑 Batch。

8. 必须考虑错误处理。

9. 必须考虑测试。

10. 不擅自扩展需求。
```

---

# 153. 完成定义

功能只有满足以下条件才能认为完成：

```text
Core 能工作

CLI 或 GUI 正确调用 Core

无重复 FFmpeg Logic

错误正确处理

不会阻塞 GUI

支持 Cancel（如果属于长任务）

输出路径安全

输入文件不会损坏

日志可用

测试覆盖核心逻辑
```

不是：

```text
能编译
```

就算完成。

---

# 154. 最终产品哲学

Qt 版本追求：

```text
Powerful Media Workstation
```

Rust 版本追求：

```text
Fast Media Processor
```

Rust 版本不应该问：

```text
“怎样复制 Qt 版本的所有功能？”
```

而应该问：

```text
“怎样用最少操作，
最快、最可靠地处理大量媒体文件？”
```

整个项目的所有功能、架构和 UI 决策，都必须围绕这个问题展开。

---

# 155. 最终设计目标

理想情况下，用户可以：

```text
第一次使用：

拖入文件
→ 选 Preset
→ Run
```

熟练以后：

```bash
media-rs convert *.mov --preset h265-small
```

更进一步：

```bash
media-rs batch ./media \
    --recursive \
    --pipeline publish.json
```

最终实现：

```text
GUI
CLI
Batch
Preset
Automation
```

共享同一个：

```text
Rust Media Core
```

这就是 Rust 版本与 C++ / Qt 版本最重要的区别。
