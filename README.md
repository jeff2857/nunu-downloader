# nunu-downloader

[努努书坊](https://www.kanunu8.com/) 小说下载器

## Usage

```
USAGE:
    nunu-downloader [OPTIONS] <url>

ARGS:
    <url>    Url of the novel's index page

OPTIONS:
    -h, --help             Print help information
    -o, --output <file>    Set output file name
    -V, --version          Print version information

```

`url` 是小说首页链接，如下载 莫言 的 《檀香刑》:

```
nunu-downloader https://www.kanunu8.com/book3/8254/index.html
```

程序会有如下输出：

```
Downloading 《檀香刑》 作者: 莫言 ...
Downloading 第一章 眉娘浪语 ...
Downloading 第二章 赵甲狂言 ...
Downloading 第三章 小甲傻话 ...
Downloading 第四章 钱丁恨声 ...
Downloading 第五章 斗须 ...
Downloading 第六章 比脚 ...
Downloading 第七章 悲歌 ...
Downloading 第八章 神坛 ...
Downloading 第九章 杰作 ...
Downloading 第十章 践约 ...
Downloading 第十一章 金枪 ...
Downloading 第十二章 夹缝 ...
Downloading 第十三章 破城 ...
Downloading 第十四章 赵甲道白 ...
Downloading 第十五章 眉娘诉说 ...
Downloading 第十六章 孙丙说戏 ...
Downloading 第十七章 小甲放歌 ...
Downloading 第十八章 知县绝唱 ...
Downloading 檀香刑 finished
```

下载完成后在当前目录下保存为文件 `檀香刑_莫言.txt`
