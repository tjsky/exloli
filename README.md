# exloli

从 E 站里站下载指定关键词的画(ben)廊(zi)并上传到 Telegraph 并发布到 Telegram 频道

## 手动安装

```
cargo install --git https://github.com/Aloxaf/exloli
```

注: 由于依赖 libxml, Windows 上的安装可能比较麻烦...

## 用法

1. 创建 Telegram Channel, 并设为公开.
   私有 Channel 需要手动获取 chat id, 方法: 邀请 `@get_id_bot` 到 Channel 中, 然后发送 `/my_id@get_id_bot`
2. 创建 Telegram Bot, 记录 TOKEN, 并拉进 Channel
3. 创建 Telegraph 账号, 记录 TOKEN. 创建方法: 访问 `https://api.telegra.ph/createAccount?short_name={}&author_name={}&author_url={}`
4. 在当前目录下建立 config.toml, 然后运行 exloli

config.toml 模板如下

```toml
# [可选] 日志等级, 可选 INFO, DEBUG, ERROR. 默认 INFO
log_level = "INFO"
# [可选] 图片下载并发数. 默认 4
threads_num = "4"

[exhentai]
# E 站用户名
username = "username"
# E 站密码
password = "password"
# 可选, 使用 cookie 登录
cookie = "ipb_member_id=xx; ipb_pass_hash=xx; igneous=xx;"
# 搜索关键词
keyword = "female:lolicon language:Chinese"
# 是否在 Watched 页面搜索 (默认在首页搜索
# Watched 页面的搜索定制性更高 (当然前提是你定制了
search_watched = false
# 上传前多少页的本子, 重复的会自动过滤
max_pages = 2
# 最大展示的图片数量
max_img_cnt = 50
# 是否在本地缓存
local_cache = false
# 缓存位置
cache_path = "./data"

[telegraph]
# 是否上传 telegraph
upload = true
# telegraph 账号 token
access_token = "TOKEN"
# 作者名称
author_name = "exloli"
# 作者地址(通常为频道链接)
author_url = "https://t.me/exlolicon"

[telegram]
# telegram 频道 ID, 公共频道直接 @+频道名, 私有频道需要需要获取数字格式的 id
channel_id = "@exlolicon"
# 机器人 token
token = "TOKEN"

```

抓取完会自动退出, 如果想要定时抓取需要自行设定计划任务.
