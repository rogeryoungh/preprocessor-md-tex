# Preprocessor TeX in Markdown

当我使用 `hugo/hexo/...` 等静态站生成工具时，markdown 的各种转义总是使得 `KaTeX` 无法正常渲染，于是我造了这个小工具，它只是简单的遍历所有 `*.md` 文件，提前转义特殊字符。

注意这个工具依赖于转义，所以对一个文件执行第二遍替换时，会在第一遍的基础上修改。可能会产生不符合预期的行为。

我发现 TeX 和 markdown 不兼容的细节有：

- 下划线 `_`：被翻译为 `<em>`。
  - 修复：替换为 `\_`。
- 星号 `*`：被翻译为 `<em>`。
  - 修复：替换为 `\*`。
- 双反斜杠 `\\`：被翻译为 `\`。
  - 修复：替换为 `\\\\`。

仅在正文的公式块中执行以上替换，特别的代码块中不应触发 `$`。

## Usage

```
$ preprocesser-md-tex test
$ # or
$ preprocesser-md-tex test t-md md
```

------

Just walk through all `*.md` file and do simple text replacement.

**Note**: This tool depands on replacement, so when a file is replaced in the second pass, it will be modified on the basis of the first pass. May produce unexpected behavior. 
