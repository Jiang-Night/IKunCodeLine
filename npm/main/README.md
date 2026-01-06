# @ikuncode/ikuncodeline

IKunCodeLine 是 ikuncode 站特供版 Claude Code 状态栏工具。

## 安装
```bash
npm install -g @ikuncode/ikuncodeline
```

安装后默认路径：`~/.claude/ikuncodeline/ikuncodeline`

## 使用
```bash
ikuncodeline --help
ikuncodeline --version
```

## Claude Code 配置
在 `~/.claude/settings.json` 中设置：
```json
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/ikuncodeline/ikuncodeline",
    "padding": 0
  }
}
```

## 余额配置
在 `settings.json` 的 `env` 中加入：
```json
{
  "env": {
    "ANTHROPIC_AUTH_TOKEN": "xxx",
    "ANTHROPIC_BASE_URL": "xxx",
    "BALANCE_API_KEY": "YOUR_TOKEN",
    "BALANCE_API_USER": "12345"
  }
}
```

- 官网：https://api.ikuncode.cc/
- 当前仓库：https://github.com/Jiang-Night/IKunCodeLine
- 原作者仓库：https://github.com/Haleclipse/CCometixLine
