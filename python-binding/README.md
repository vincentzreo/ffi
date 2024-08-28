# python-binding

Describe your project here.

安装配置：

```shell
rye init python-binding --build-system maturin
rye sync
rye install maturin
rye add --dev pip
rye add --dev ipython

maturin develop
rye run ipython
```

要点：

- 如果修改项目名：
  - pyproject.toml 中 project 名称，module-name
  - 目录名
  - **init**.py 中 import 名称一致
- 新撰写的 function / class 记得在 lib.rs 下引入，并且在 **init**.py 中引入
- 使用 magic function 使得代码更加 python 化
